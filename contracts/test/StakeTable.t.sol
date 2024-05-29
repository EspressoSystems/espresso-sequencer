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
import { LightClient as LC } from "../src/LightClient.sol";
import { LightClientMock as LCTest } from "../test/mocks/LightClientMock.sol";

// Token contract
import { ExampleToken } from "../src/ExampleToken.sol";

// Target contract
import { StakeTable as S } from "../src/StakeTable.sol";

contract StakeTableCommonTest is Test {
    function genClientWallet(address sender, uint8 seed)
        internal
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
}

contract StakeTable_Test is StakeTableCommonTest {
    S public stakeTable;
    ExampleToken public token;

    LCTest public lc;
    LC.LightClientState public genesis;
    uint32 public constant BLOCKS_PER_EPOCH_TEST = 10;

    uint256 public constant INITIAL_BALANCE = 1_000_000_000;
    address public exampleTokenCreator;

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
        stakeTable.register(
            blsVK,
            schnorrVK,
            depositAmount,
            AbstractStakeTable.StakeType.Native,
            sig,
            validUntilEpoch
        );

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

        uint64 numInitValidators = 5;
        string[] memory cmds = new string[](4);
        cmds[0] = "diff-test";
        cmds[1] = "mock-genesis";
        cmds[2] = vm.toString(BLOCKS_PER_EPOCH_TEST);
        cmds[3] = vm.toString(numInitValidators);

        bytes memory result = vm.ffi(cmds);
        (LC.LightClientState memory state,,) =
            abi.decode(result, (LC.LightClientState, bytes32, bytes32));

        genesis = state;

        lc = new LCTest(genesis, BLOCKS_PER_EPOCH_TEST);
        stakeTable = new S(address(token), address(lc), 10);
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
        lc.setCurrentEpoch(3);
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
        vm.expectEmit(true, true, true, true, address(stakeTable));
        emit AbstractStakeTable.Registered(
            stakeTable._hashBlsKey(blsVK), node.registerEpoch, node.stakeType, node.balance
        );
        vm.prank(exampleTokenCreator);
        stakeTable.register(
            blsVK,
            schnorrVK,
            depositAmount,
            AbstractStakeTable.StakeType.Native,
            sig,
            validUntilEpoch
        );

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
        lc.setCurrentEpoch(3);
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
        lc.setCurrentEpoch(3);
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

        lc.setCurrentEpoch(3);
        vm.expectEmit(false, false, false, true, address(stakeTable));
        emit AbstractStakeTable.Deposit(stakeTable._hashBlsKey(blsVK), newDepositAmount);
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

        lc.setCurrentEpoch(5);
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

        lc.setCurrentEpoch(0);

        // Request another time, an error is raised.
        vm.prank(exampleTokenCreator);
        vm.expectRevert(S.PrematureExit.selector);
        stakeTable.requestExit(blsVK);
    }

    function test_RequestExit_Succeeds() external {
        BN254.G2Point memory blsVK;
        (blsVK,) = runSuccessfulRegistration();

        // Check the right event is emitted
        lc.setCurrentEpoch(5);

        vm.expectEmit(false, false, false, true, address(stakeTable));
        bytes32 key = stakeTable._hashBlsKey(blsVK);
        uint64 exitEpoch = 6;
        emit AbstractStakeTable.Exit(key, exitEpoch);

        // Make the exit request
        vm.prank(exampleTokenCreator);
        stakeTable.requestExit(blsVK);

        // Check the outcome of the request
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
        lc.setCurrentEpoch(withdrawEpoch);
        vm.expectRevert(S.PrematureWithdrawal.selector);
        stakeTable.withdrawFunds(blsVK);
    }

    function test_withdrawFunds_Succeeds() external {
        BN254.G2Point memory blsVK;
        uint256 depositAmount;
        (blsVK, depositAmount) = runSuccessfulRegistration();

        AbstractStakeTable.Node memory node = stakeTable.lookupNode(blsVK);
        uint64 withdrawEpoch = node.exitEpoch + stakeTable.exitEscrowPeriod(node);
        lc.setCurrentEpoch(withdrawEpoch);

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
}
