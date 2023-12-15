// SPDX-License-Identifier: Unlicensed

/* solhint-disable contract-name-camelcase, func-name-mixedcase, one-contract-per-file */

pragma solidity ^0.8.0;

// Libraries
import "forge-std/Test.sol";

using stdStorage for StdStorage;

import { ERC20 } from "solmate/utils/SafeTransferLib.sol";
import { BN254 } from "bn254/BN254.sol";
import { BLSSig } from "../src/libraries/BLSSig.sol";
import { EdOnBN254 } from "../src/libraries/EdOnBn254.sol";
import { AbstractStakeTable } from "../src/interfaces/AbstractStakeTable.sol";
import { LightClient } from "../src/LightClient.sol";
import { LightClientTest } from "../test/mocks/LightClientTest.sol";

// Token contract
import { ExampleToken } from "../src/ExampleToken.sol";

// Target contract
import { StakeTable as S } from "../src/StakeTable.sol";

contract StakeTable_Test is Test {
    event Registered(bytes32, uint64, AbstractStakeTable.StakeType, uint256);
    event Deposit(bytes32, uint256);
    event Exit(bytes32, uint64);

    S public stakeTable;
    ExampleToken public token;
    LightClientTest public lightClientContract;
    uint256 constant INITIAL_BALANCE = 1_000_000_000;
    address exampleTokenCreator;

    function genClientWallet(address sender, uint8 seed)
        private
        returns (BN254.G2Point memory, EdOnBN254.EdOnBN254Point memory, BN254.G1Point memory)
    {
        // Generate a BLS signature and other values using rust code
        string[] memory cmds = new string[](4);
        cmds[0] = "diff-test";
        cmds[1] = "gen-client-wallet";
        cmds[2] = vm.toString(sender);
        cmds[3] = vm.toString(seed);

        bytes memory result = vm.ffi(cmds);
        (
            BN254.G1Point memory blsSig,
            BN254.G2Point memory blsVK,
            uint256 schnorrVKx,
            uint256 schnorrVKy
        ) = abi.decode(result, (BN254.G1Point, BN254.G2Point, uint256, uint256));

        return (
            blsVK,
            EdOnBN254.EdOnBN254Point(schnorrVKx, schnorrVKy), // schnorrVK
            blsSig
        );
    }

    function registerWithSeed(address sender, uint8 seed, uint64 depositAmount, bool expectRevert)
        private
        returns (BN254.G2Point memory, uint64)
    {
        (
            BN254.G2Point memory blsVK,
            EdOnBN254.EdOnBN254Point memory schnorrVK,
            BN254.G1Point memory sig
        ) = genClientWallet(sender, seed);

        uint64 validUntilEpoch = 1000;

        // Transfer some tokens to sender
        vm.prank(exampleTokenCreator);
        token.transfer(sender, depositAmount);

        // Prepare for the token transfer
        vm.prank(sender);
        token.approve(address(stakeTable), depositAmount);

        vm.prank(sender);
        if (expectRevert) {
            vm.expectRevert(S.NodeAlreadyRegistered.selector);
        }
        bool res = stakeTable.register(
            blsVK,
            schnorrVK,
            depositAmount,
            AbstractStakeTable.StakeType.Native,
            sig,
            validUntilEpoch
        );
        if (!expectRevert) {
            assertTrue(res);
        } else {
            assertFalse(res);
        }

        return (blsVK, depositAmount);
    }

    /// @dev  Helper function to simulate a successful registration
    function runSuccessfulRegistration() private returns (BN254.G2Point memory, uint256) {
        return registerWithSeed(exampleTokenCreator, 34, 10, false);
    }

    function setUp() public {
        exampleTokenCreator = makeAddr("tokenCreator");
        vm.prank(exampleTokenCreator);
        token = new ExampleToken(INITIAL_BALANCE);

        LightClient.LightClientState memory genesis = LightClient.LightClientState({
            viewNum: 0,
            blockHeight: 0,
            blockCommRoot: BN254.ScalarField.wrap(0),
            feeLedgerComm: BN254.ScalarField.wrap(0),
            stakeTableBlsKeyComm: BN254.ScalarField.wrap(0),
            stakeTableSchnorrKeyComm: BN254.ScalarField.wrap(0),
            stakeTableAmountComm: BN254.ScalarField.wrap(0),
            threshold: 0
        });
        lightClientContract = new LightClientTest(genesis, 10);
        stakeTable = new S(address(token), address(lightClientContract), 10);
    }

    /// `register` function

    function testFuzz_RevertWhen_UsingRestakeToken(uint64 depositAmount, uint64 validUntilEpoch)
        external
    {
        (
            BN254.G2Point memory blsVK,
            EdOnBN254.EdOnBN254Point memory schnorrVK,
            BN254.G1Point memory sig
        ) = genClientWallet(exampleTokenCreator, 0);

        uint64 curEpoch = stakeTable.currentEpoch();
        depositAmount = uint64(bound(depositAmount, 1, INITIAL_BALANCE));
        validUntilEpoch = uint64(bound(validUntilEpoch, curEpoch, curEpoch + 10));

        // Throw "Restaking not implemented" error
        vm.expectRevert(S.RestakingNotImplemented.selector);
        stakeTable.register(
            blsVK,
            schnorrVK,
            depositAmount,
            AbstractStakeTable.StakeType.Restake,
            sig,
            validUntilEpoch
        );
    }

    function testFuzz_RevertWhen_InvalidBLSSig(uint256 _scalar) external {
        uint64 depositAmount = 10;
        uint64 validUntilEpoch = 5;

        (BN254.G2Point memory blsVK, EdOnBN254.EdOnBN254Point memory schnorrVK,) =
            genClientWallet(exampleTokenCreator, 0);

        // Ensure the scalar is valid
        // Note: Apparently BN254.scalarMul is not well defined when the scalar is 0
        BN254.ScalarField scalar = BN254.ScalarField.wrap(bound(_scalar, 1, BN254.R_MOD - 1));
        BN254.validateScalarField(scalar);
        BN254.G1Point memory badSig = BN254.scalarMul(BN254.P1(), scalar);
        BN254.validateG1Point(badSig);

        // Failed signature verification
        vm.expectRevert(BLSSig.BLSSigVerificationFailed.selector);
        stakeTable.register(
            blsVK,
            schnorrVK,
            depositAmount,
            AbstractStakeTable.StakeType.Native,
            badSig,
            validUntilEpoch
        );
    }

    function testFuzz_RevertWhen_InvalidNextRegistrationEpoch(uint64 rand) external {
        lightClientContract.setCurrentEpoch(3);
        uint64 currentEpoch = stakeTable.currentEpoch();

        uint64 depositAmount = 10;
        vm.prank(exampleTokenCreator);
        token.approve(address(stakeTable), depositAmount);

        (
            BN254.G2Point memory blsVK,
            EdOnBN254.EdOnBN254Point memory schnorrVK,
            BN254.G1Point memory sig
        ) = genClientWallet(exampleTokenCreator, 0);

        // Invalid next registration epoch
        uint64 validUntilEpoch = uint64(bound(rand, 0, currentEpoch - 1));
        vm.prank(exampleTokenCreator);
        vm.expectRevert(
            abi.encodeWithSelector(
                S.InvalidNextRegistrationEpoch.selector, currentEpoch + 1, validUntilEpoch
            )
        );
        stakeTable.register(
            blsVK,
            schnorrVK,
            depositAmount,
            AbstractStakeTable.StakeType.Native,
            sig,
            validUntilEpoch
        );

        // Valid next registration epoch
        validUntilEpoch = uint64(bound(rand, currentEpoch + 1, type(uint64).max));
        vm.prank(exampleTokenCreator);
        stakeTable.register(
            blsVK,
            schnorrVK,
            depositAmount,
            AbstractStakeTable.StakeType.Native,
            sig,
            validUntilEpoch
        );
    }

    function test_RevertWhen_NodeAlreadyRegistered() external {
        uint64 depositAmount = 10;
        uint64 validUntilEpoch = 5;

        (
            BN254.G2Point memory blsVK,
            EdOnBN254.EdOnBN254Point memory schnorrVK,
            BN254.G1Point memory sig
        ) = genClientWallet(exampleTokenCreator, 0);

        // Prepare for the token transfer
        vm.prank(exampleTokenCreator);
        token.approve(address(stakeTable), depositAmount);

        // Successful call to register
        vm.prank(exampleTokenCreator);
        stakeTable.register(
            blsVK,
            schnorrVK,
            depositAmount,
            AbstractStakeTable.StakeType.Native,
            sig,
            validUntilEpoch
        );

        // The node is already registered
        vm.prank(exampleTokenCreator);
        vm.expectRevert(S.NodeAlreadyRegistered.selector);
        stakeTable.register(
            blsVK,
            schnorrVK,
            depositAmount,
            AbstractStakeTable.StakeType.Native,
            sig,
            validUntilEpoch
        );
    }

    function test_RevertWhen_TransferFailed() external {
        uint64 depositAmount = 10;
        uint64 validUntilEpoch = 10;

        (
            BN254.G2Point memory blsVK,
            EdOnBN254.EdOnBN254Point memory schnorrVK,
            BN254.G1Point memory sig
        ) = genClientWallet(exampleTokenCreator, 0);

        assertEq(ERC20(token).balanceOf(exampleTokenCreator), INITIAL_BALANCE);
        vm.prank(exampleTokenCreator);
        // The call to register is expected to fail because the depositAmount has not been approved
        // and thus the stake table contract cannot lock the stake.
        vm.expectRevert("TRANSFER_FROM_FAILED");
        stakeTable.register(
            blsVK,
            schnorrVK,
            depositAmount,
            AbstractStakeTable.StakeType.Native,
            sig,
            validUntilEpoch
        );

        // A user with 0 balance cannot register either
        address newUser = makeAddr("New user with zero balance");
        (blsVK, schnorrVK, sig) = genClientWallet(newUser, 0);

        vm.prank(newUser);
        vm.expectRevert("TRANSFER_FROM_FAILED");
        stakeTable.register(
            blsVK,
            schnorrVK,
            depositAmount,
            AbstractStakeTable.StakeType.Native,
            sig,
            validUntilEpoch
        );
    }

    /// @dev Tests a correct registration
    function test_Registration_succeeds() external {
        (
            BN254.G2Point memory blsVK,
            EdOnBN254.EdOnBN254Point memory schnorrVK,
            BN254.G1Point memory sig
        ) = genClientWallet(exampleTokenCreator, 0);

        uint64 depositAmount = 10;
        uint64 validUntilEpoch = 5;

        // Prepare for the token transfer
        vm.prank(exampleTokenCreator);
        token.approve(address(stakeTable), depositAmount);

        // Balances before registration
        assertEq(token.balanceOf(exampleTokenCreator), INITIAL_BALANCE);

        uint256 nativeAmount;
        uint256 restakedAmount;
        (nativeAmount, restakedAmount) = stakeTable.totalStake();
        assertEq(nativeAmount, 0);
        assertEq(restakedAmount, 0);

        AbstractStakeTable.Node memory node;
        node.account = exampleTokenCreator;
        node.balance = depositAmount;
        node.stakeType = AbstractStakeTable.StakeType.Native;
        node.schnorrVK = schnorrVK;
        node.registerEpoch = 1;

        // Check event is emitted after calling successfully `register`
        vm.expectEmit(false, false, false, true, address(stakeTable));
        emit Registered(
            stakeTable._hashBlsKey(blsVK), node.registerEpoch, node.stakeType, node.balance
        );
        vm.prank(exampleTokenCreator);
        bool res = stakeTable.register(
            blsVK,
            schnorrVK,
            depositAmount,
            AbstractStakeTable.StakeType.Native,
            sig,
            validUntilEpoch
        );

        assertTrue(res);

        // Balance after registration
        assertEq(token.balanceOf(exampleTokenCreator), INITIAL_BALANCE - depositAmount);
        (nativeAmount, restakedAmount) = stakeTable.totalStake();
        assertEq(nativeAmount, depositAmount);
        assertEq(restakedAmount, 0);
    }

    /// `deposit` function
    function test_RevertWhen_CallerIsNotTheNodeOwner_Deposit() external {
        BN254.G2Point memory blsVK;
        (blsVK,) = runSuccessfulRegistration();

        /// Try to deposit while not being the owner of the node
        vm.prank(makeAddr("Not node owner"));

        vm.expectRevert(S.Unauthenticated.selector);
        stakeTable.deposit(blsVK, 5);
    }

    function test_RevertWhen_RegistrationNotCompletedYet() external {
        BN254.G2Point memory blsVK;
        (blsVK,) = runSuccessfulRegistration();

        /// Try to deposit but the registration is not completed yet
        vm.prank(exampleTokenCreator);
        vm.expectRevert(S.PrematureDeposit.selector);
        stakeTable.deposit(blsVK, 5);
    }

    function test_RevertWhen_DepositingWhileExiting_Deposit() external {
        BN254.G2Point memory blsVK;
        (blsVK,) = runSuccessfulRegistration();

        /// Try to deposit while an exit request is already in progress.

        // Ensure the registration is completed.
        lightClientContract.setCurrentEpoch(3);
        vm.prank(exampleTokenCreator);
        stakeTable.requestExit(blsVK);
        vm.prank(exampleTokenCreator);
        vm.expectRevert(S.ExitRequestInProgress.selector);
        stakeTable.deposit(blsVK, 10);
    }

    function testFuzz_RevertWhen_DepositingWithoutEnoughApprovedFunds(uint64 rand) external {
        uint64 depositAmount = 20;
        BN254.G2Point memory blsVK;
        (blsVK,) = runSuccessfulRegistration();

        // Prepare for the token transfer
        rand = uint64(bound(rand, 0, depositAmount - 1));
        vm.prank(exampleTokenCreator);
        token.approve(address(stakeTable), rand);

        // Ensure the registration is completed.
        lightClientContract.setCurrentEpoch(3);
        vm.expectRevert("TRANSFER_FROM_FAILED");
        vm.prank(exampleTokenCreator);
        stakeTable.deposit(blsVK, depositAmount);
    }

    function test_Deposits_succeeds() external {
        uint256 depositAmount;
        BN254.G2Point memory blsVK;
        (blsVK, depositAmount) = runSuccessfulRegistration();

        // Prepare for the token transfer
        uint64 newDepositAmount = 20;
        vm.prank(exampleTokenCreator);
        token.approve(address(stakeTable), newDepositAmount);

        // Balance before
        uint64 balanceUserBefore = uint64(token.balanceOf(exampleTokenCreator));
        assertEq(balanceUserBefore, INITIAL_BALANCE - depositAmount);
        AbstractStakeTable.Node memory node = stakeTable.lookupNode(blsVK);
        uint64 stake = node.balance;

        lightClientContract.setCurrentEpoch(3);
        vm.expectEmit(false, false, false, true, address(stakeTable));
        emit Deposit(stakeTable._hashBlsKey(blsVK), newDepositAmount);
        uint64 newBalance;
        uint64 effectiveEpoch;
        uint64 expectedNewBalance = stake + newDepositAmount;
        vm.prank(exampleTokenCreator);
        (newBalance, effectiveEpoch) = stakeTable.deposit(blsVK, newDepositAmount);
        assertEq(newBalance, expectedNewBalance);
        assertEq(effectiveEpoch, stakeTable.currentEpoch() + 1);

        // Balance after
        uint256 balanceUserAfter = token.balanceOf(exampleTokenCreator);
        assertEq(balanceUserAfter, balanceUserBefore - newDepositAmount);
        node = stakeTable.lookupNode(blsVK);
        uint256 newStake = node.balance;
        assertEq(newStake, expectedNewBalance);
    }

    /// `requestExit` function
    function test_RevertWhen_CallerIsNotTheNodeOwner_RequestExit() external {
        BN254.G2Point memory blsVK;
        (blsVK,) = runSuccessfulRegistration();

        /// Try to deposit while not being the owner of the node
        vm.prank(makeAddr("Not node owner"));

        vm.expectRevert(S.Unauthenticated.selector);
        stakeTable.requestExit(blsVK);
    }

    function test_RevertWhen_TryingToExitTwice() external {
        BN254.G2Point memory blsVK;
        (blsVK,) = runSuccessfulRegistration();

        lightClientContract.setCurrentEpoch(5);
        // Request Exit once, nothing happens.
        vm.prank(exampleTokenCreator);
        stakeTable.requestExit(blsVK);

        // Request another time, an error is raised.
        vm.prank(exampleTokenCreator);
        vm.expectRevert(S.ExitRequestInProgress.selector);
        stakeTable.requestExit(blsVK);
    }

    function test_RevertTryingToExitTooEarly() external {
        BN254.G2Point memory blsVK;
        (blsVK,) = runSuccessfulRegistration();

        lightClientContract.setCurrentEpoch(0);

        // Request another time, an error is raised.
        vm.prank(exampleTokenCreator);
        vm.expectRevert(S.PrematureExit.selector);
        stakeTable.requestExit(blsVK);
    }

    function test_RequestExit_Succeeds() external {
        BN254.G2Point memory blsVK;
        (blsVK,) = runSuccessfulRegistration();

        // Check the right event is emitted
        lightClientContract.setCurrentEpoch(5);

        vm.expectEmit(false, false, false, true, address(stakeTable));
        bytes32 key = stakeTable._hashBlsKey(blsVK);
        uint64 exitEpoch = 6;
        emit Exit(key, exitEpoch);

        // Make the exit request
        vm.prank(exampleTokenCreator);
        bool res = stakeTable.requestExit(blsVK);

        // Check the outcome of the request
        assertTrue(res);
        AbstractStakeTable.Node memory node = stakeTable.lookupNode(blsVK);
        assertEq(node.exitEpoch, exitEpoch);
    }

    // `withdrawFunds` function
    function testFuzz_RevertWhen_WithdrawingTooEarly(uint64 withdrawEpoch) external {
        BN254.G2Point memory blsVK;
        (blsVK,) = runSuccessfulRegistration();

        AbstractStakeTable.Node memory node = stakeTable.lookupNode(blsVK);
        uint64 minWithdrawEpoch = node.exitEpoch + stakeTable.exitEscrowPeriod(node);

        withdrawEpoch = uint64(bound(withdrawEpoch, 0, minWithdrawEpoch - 1));
        lightClientContract.setCurrentEpoch(withdrawEpoch);
        vm.expectRevert(S.PrematureWithdrawal.selector);
        stakeTable.withdrawFunds(blsVK);
    }

    function test_withdrawFunds_Succeeds() external {
        BN254.G2Point memory blsVK;
        uint256 depositAmount;
        (blsVK, depositAmount) = runSuccessfulRegistration();

        AbstractStakeTable.Node memory node = stakeTable.lookupNode(blsVK);
        uint64 withdrawEpoch = node.exitEpoch + stakeTable.exitEscrowPeriod(node);
        lightClientContract.setCurrentEpoch(withdrawEpoch);

        // Balance before
        assertEq(token.balanceOf(exampleTokenCreator), INITIAL_BALANCE - depositAmount);

        // Withdraw the funds
        uint64 balance = stakeTable.withdrawFunds(blsVK);
        assertEq(balance, uint64(depositAmount));

        // Balance after
        assertEq(token.balanceOf(exampleTokenCreator), INITIAL_BALANCE);

        // Node is deleted
        node = stakeTable.lookupNode(blsVK);
        AbstractStakeTable.Node memory nullNode = AbstractStakeTable.Node(
            address(0), AbstractStakeTable.StakeType.Native, 0, 0, 0, EdOnBN254.EdOnBN254Point(0, 0)
        );

        assertEq(abi.encode(node), abi.encode(nullNode));
    }

    // Queue logic

    uint256 private constant ARRAY_SIZE = 20;

    /// Helper function to handle registrations in testFuzz_SequencesOfEvents
    /// This function was extracted to make sol-lint happy by reducing cyclotomic complexity
    function handleRegistrations(
        uint256 i,
        uint8[ARRAY_SIZE] memory rands,
        BN254.G2Point[ARRAY_SIZE] memory registeredKeys,
        bool[ARRAY_SIZE] memory isKeyActive
    ) private returns (bool) {
        address sender = makeAddr(string(abi.encode(i)));
        uint64 randDepositAmount = uint64(rands[i]);

        // Check if the seed has already been used. In this case the registration will fail.
        bool seedUsed = false;
        for (uint256 j = 0; j < i; j++) {
            if ((rands[i] == rands[j]) && (isKeyActive[j])) {
                seedUsed = true;
                break;
            }
        }

        if (seedUsed) {
            registerWithSeed(sender, rands[i], randDepositAmount, true);
            return false;
        } else {
            (BN254.G2Point memory blsVK,) =
                registerWithSeed(sender, rands[i], randDepositAmount, false);
            registeredKeys[i] = blsVK;
            isKeyActive[i] = true;
            // Invariants specific to a successful registration
            assertGe(stakeTable._firstAvailableRegistrationEpoch(), stakeTable.currentEpoch() + 1);
            assertGe(stakeTable.numPendingRegistrations(), 1);
            return true;
        }
    }

    ///@dev Test invariants about our queue logic holds during a random sequence of register,
    /// requestExit, and advanceEpoch operations
    function testFuzz_SequencesOfEvents(
        uint8[ARRAY_SIZE] memory events,
        uint8[ARRAY_SIZE] memory rands
    ) external {
        BN254.G2Point[ARRAY_SIZE] memory registeredKeys;

        // Tracks the indices corresponding to an active key
        bool[ARRAY_SIZE] memory isKeyActive;

        uint64 numRegistrations = 0;
        uint64 numExits = 0;

        for (uint256 i = 0; i < ARRAY_SIZE; i++) {
            uint256 ev = bound(events[i], 0, 2);

            bool exitRequestSuccessful = false;

            if (ev == 0) {
                // Registrations
                bool res = handleRegistrations(i, rands, registeredKeys, isKeyActive);
                if (res) {
                    numRegistrations++;
                }
            } else if (ev == 1) {
                // Exits
                if (numRegistrations == 0) {
                    break;
                }
                uint256 indexRegistration = bound(rands[i], 0, numRegistrations - 1);
                bytes32 hashNode = stakeTable._hashBlsKey(registeredKeys[indexRegistration]);
                (
                    address sender,
                    AbstractStakeTable.StakeType stakeType,
                    uint64 balance,
                    uint64 registerEpoch,
                    uint64 exitEpoch,
                ) = stakeTable.nodes(hashNode);

                balance;
                stakeType;

                BN254.G2Point memory blsVK = registeredKeys[indexRegistration];

                bool canExit = (stakeTable.currentEpoch() >= registerEpoch + 1) && (exitEpoch == 0);
                if (canExit) {
                    vm.prank(sender);
                    bool res = stakeTable.requestExit(blsVK);
                    assertTrue(res);
                    numExits++;
                    exitRequestSuccessful = true;

                    // Invariants specific to a successful exit
                    assertGe(stakeTable._firstAvailableExitEpoch(), stakeTable.currentEpoch() + 1);
                    assertGe(stakeTable.numPendingExits(), 1);
                } else {
                    vm.prank(sender);
                    vm.expectRevert();
                    bool res = stakeTable.requestExit(blsVK);
                    assertFalse(res);
                }
            } else {
                // Advance epoch
                // ev == 2
                uint64 currentEpoch = lightClientContract.currentEpoch();
                uint64 nextEpoch = currentEpoch + 1;
                lightClientContract.setCurrentEpoch(nextEpoch);
                assertEq(stakeTable.currentEpoch(), nextEpoch);
            }

            // Global invariants
            assertLe(stakeTable.numPendingRegistrations(), stakeTable.maxChurnRate());
            assertLe(stakeTable.numPendingExits(), stakeTable.maxChurnRate());
            assertLe(numExits, numRegistrations);
        }
    }
}
