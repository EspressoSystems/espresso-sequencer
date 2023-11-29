// SPDX-License-Identifier: Unlicensed

/* solhint-disable contract-name-camelcase, func-name-mixedcase, one-contract-per-file */

pragma solidity ^0.8.0;

// Libraries
import "forge-std/Test.sol";
import { BN254 } from "bn254/BN254.sol";
import { BLSSig } from "../src/libraries/BLSSig.sol";
import { EdOnBN254 } from "../src/libraries/EdOnBn254.sol";
import { AbstractStakeTable } from "../src/interfaces/AbstractStakeTable.sol";
import { LightClient } from "../src/LightClient.sol";

// Token contract
import { ExampleToken } from "../src/ExampleToken.sol";

// Target contract
import { StakeTable as S } from "../src/StakeTable.sol";

contract StakeTable_register_Test is Test {
    event Registered(bytes32, uint64, AbstractStakeTable.StakeType, uint256);

    S public stakeTable;
    ExampleToken public token;
    LightClient public lightClientContract;
    uint256 constant INITIAL_BALANCE = 1_000;
    address exampleTokenCreator;

    function genClientWallet(address sender)
        private
        returns (BN254.G2Point memory, EdOnBN254.EdOnBN254Point memory, BN254.G1Point memory)
    {
        // Generate a BLS signature and other values using rust code
        string[] memory cmds = new string[](3);
        cmds[0] = "diff-test";
        cmds[1] = "gen-client-wallet";
        cmds[2] = vm.toString(sender);

        bytes memory result = vm.ffi(cmds);
        (
            uint256 blsSigX,
            uint256 blsSigY,
            uint256 blsVKx0,
            uint256 blsVKx1,
            uint256 blsVKy0,
            uint256 blsVKy1,
            uint256 schnorrVKx,
            uint256 schnorrVKy
        ) = abi.decode(
            result, (uint256, uint256, uint256, uint256, uint256, uint256, uint256, uint256)
        );

        return (
            BN254.G2Point(blsVKx1, blsVKx0, blsVKy1, blsVKy0), // blsVK
            EdOnBN254.EdOnBN254Point(schnorrVKx, schnorrVKy), // schnorrVK
            BN254.G1Point(blsSigX, blsSigY) // sig
        );
    }

    function setUp() public {
        exampleTokenCreator = makeAddr("tokenCreator");
        vm.prank(exampleTokenCreator);
        token = new ExampleToken(INITIAL_BALANCE);

        LightClient.LightClientState memory genesis = LightClient.LightClientState({
            viewNum: 0,
            blockHeight: 0,
            blockCommRoot: 0,
            feeLedgerComm: 0,
            stakeTableBlsKeyComm: 0,
            stakeTableSchnorrKeyComm: 0,
            stakeTableAmountComm: 0,
            threshold: 0
        });
        lightClientContract = new LightClient(genesis,10);
        address lightClientAddress = address(lightClientContract);
        stakeTable = new S(address(token),lightClientAddress);
    }

    function test_RevertWhen_UsingRestakeToken() external {
        uint64 depositAmount = 10;
        uint64 validUntilEpoch = 5;

        (
            BN254.G2Point memory blsVK,
            EdOnBN254.EdOnBN254Point memory schnorrVK,
            BN254.G1Point memory sig
        ) = genClientWallet(exampleTokenCreator);

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

    function test_RevertWhen_InvalidBLSSig() external {
        uint64 depositAmount = 10;
        uint64 validUntilEpoch = 5;

        (BN254.G2Point memory blsVK, EdOnBN254.EdOnBN254Point memory schnorrVK,) =
            genClientWallet(exampleTokenCreator);

        // Failed signature verification
        BN254.G1Point memory badSig = BN254.P1();
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

    function testFuzz_RevertWhen_InvalidNextRegistrationEpoch(uint64 validUntilEpoch) external {
        uint64 blockNumber = 20000;
        vm.roll(blockNumber);

        uint64 currentEpoch = stakeTable.currentEpoch();
        validUntilEpoch = uint64(bound(validUntilEpoch, 0, type(uint64).max));
        uint64 depositAmount = 10;
        vm.prank(exampleTokenCreator);
        token.approve(address(stakeTable), depositAmount);

        (
            BN254.G2Point memory blsVK,
            EdOnBN254.EdOnBN254Point memory schnorrVK,
            BN254.G1Point memory sig
        ) = genClientWallet(exampleTokenCreator);

        // Invalid next registration epoch
        if (validUntilEpoch < currentEpoch) {
            vm.prank(exampleTokenCreator);
            vm.expectRevert(
                abi.encodeWithSelector(
                    S.InvalidNextRegistrationEpoch.selector, currentEpoch, validUntilEpoch
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
        }
    }

    function test_RevertWhen_NodeAlreadyRegistered() external {
        uint64 depositAmount = 10;
        uint64 validUntilEpoch = 5;

        (
            BN254.G2Point memory blsVK,
            EdOnBN254.EdOnBN254Point memory schnorrVK,
            BN254.G1Point memory sig
        ) = genClientWallet(exampleTokenCreator);

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
        uint64 validUntilEpoch = 5;

        (
            BN254.G2Point memory blsVK,
            EdOnBN254.EdOnBN254Point memory schnorrVK,
            BN254.G1Point memory sig
        ) = genClientWallet(exampleTokenCreator);

        vm.prank(exampleTokenCreator);
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
        ) = genClientWallet(exampleTokenCreator);

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
}
