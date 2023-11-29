// SPDX-License-Identifier: Unlicensed

/* solhint-disable contract-name-camelcase, func-name-mixedcase, one-contract-per-file */

pragma solidity ^0.8.0;

// Libraries
import "forge-std/Test.sol";
import { BN254 } from "bn254/BN254.sol";
import { BLSSig } from "../src/libraries/BLSSig.sol";
import { EdOnBN254 } from "../src/libraries/EdOnBn254.sol";
import { IStakeTable } from "../src/interfaces/IStakeTable.sol";
import { LightClient } from "../src/LightClient.sol";

// Token contract
import { ExampleToken } from "../src/ExampleToken.sol";

// Target contract
import { StakeTable as S } from "../src/StakeTable.sol";

contract StakeTable_register_Test is Test {
    event Registered(bytes32, uint64, IStakeTable.StakeType, uint256);

    S public stakeTable;
    ExampleToken public token;
    LightClient public lightClientContract;
    address public lightClientAddress;
    address public tokenAddress;
    address exampleTokenCreator;
    uint256 constant INITIAL_BALANCE = 1_000;

    function setUp() public {
        exampleTokenCreator = msg.sender;
        vm.prank(exampleTokenCreator);
        token = new ExampleToken(INITIAL_BALANCE);
        tokenAddress = address(token);

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
        lightClientAddress = address(lightClientContract);

        stakeTable = new S(tokenAddress,lightClientAddress);
    }

    /// @dev Tests the key registering process
    function testRegister() external {
        // Generate a BLS signature and other values using rust code
        string[] memory cmds = new string[](3);
        cmds[0] = "diff-test";
        cmds[1] = "gen-bls-sig";
        cmds[2] = vm.toString(msg.sender);

        bytes memory result = vm.ffi(cmds);
        (
            uint256 blsSigX,
            uint256 blsSigY,
            uint256 blsVKx0,
            uint256 blsVKx1,
            uint256 blsVKy0,
            uint256 blsVKy1,
            uint256 schnorrVKx,
            uint256 schnorrVKy,
            address msgSenderAddress
        ) = abi.decode(
            result,
            (uint256, uint256, uint256, uint256, uint256, uint256, uint256, uint256, address)
        );

        uint64 depositAmount = 10;
        uint64 validUntilEpoch = 5;

        // Prepare for the token transfer
        vm.prank(msgSenderAddress);
        token.approve(address(stakeTable), depositAmount);

        // Note: (x,y) coordinates for each field component must be inverted.
        BN254.G2Point memory blsVK = BN254.G2Point(blsVKx1, blsVKx0, blsVKy1, blsVKy0);
        BN254.G1Point memory sig = BN254.G1Point(blsSigX, blsSigY);
        EdOnBN254.EdOnBN254Point memory schnorrVK = EdOnBN254.EdOnBN254Point(schnorrVKx, schnorrVKy);

        assertEq(msg.sender, msgSenderAddress);
        vm.prank(msgSenderAddress);

        // Failed signature verification
        BN254.G1Point memory badSig = BN254.P1();
        vm.expectRevert(BLSSig.BLSSigVerificationFailed.selector);
        stakeTable.register(
            blsVK, schnorrVK, depositAmount, IStakeTable.StakeType.Native, badSig, validUntilEpoch
        );

        // Throw "Restaking not implemented" error
        vm.expectRevert(S.RestakingNotImplemented.selector);
        stakeTable.register(
            blsVK, schnorrVK, depositAmount, IStakeTable.StakeType.Restake, sig, validUntilEpoch
        );

        // Invalid next registration epoch
        vm.prank(msgSenderAddress);
        vm.expectRevert(abi.encodeWithSelector(S.InvalidNextRegistrationEpoch.selector, 1, 0));
        stakeTable.register(blsVK, schnorrVK, depositAmount, IStakeTable.StakeType.Native, sig, 0);

        // Balances before registration
        assertEq(token.balanceOf(msgSenderAddress), INITIAL_BALANCE);

        uint256 nativeAmount;
        uint256 restakedAmount;
        (nativeAmount, restakedAmount) = stakeTable.totalStake();
        assertEq(nativeAmount, 0);
        assertEq(restakedAmount, 0);

        // Check event is emitted
        vm.expectEmit(false, false, false, true, address(stakeTable));
        IStakeTable.Node memory node;
        node.account = msgSenderAddress;
        node.balance = depositAmount;
        node.stakeType = IStakeTable.StakeType.Native;
        node.schnorrVK = schnorrVK;
        node.registerEpoch = 1;

        // The function IStakeTable._hashBlsKey is private but its code is small hence we copy it
        // there.
        emit Registered(
            keccak256(abi.encode(blsVK.x0, blsVK.x1, blsVK.y0, blsVK.y1)),
            node.registerEpoch,
            node.stakeType,
            node.balance
        );

        // Happy path
        vm.prank(msgSenderAddress);
        bool res = stakeTable.register(
            blsVK, schnorrVK, depositAmount, IStakeTable.StakeType.Native, sig, validUntilEpoch
        );
        assertTrue(res);

        // Balance after registration
        assertEq(token.balanceOf(msgSenderAddress), INITIAL_BALANCE - depositAmount);
        (nativeAmount, restakedAmount) = stakeTable.totalStake();
        assertEq(nativeAmount, depositAmount);
        assertEq(restakedAmount, 0);

        // The node is already registered
        vm.prank(msgSenderAddress);
        vm.expectRevert(S.NodeAlreadyRegistered.selector);
        stakeTable.register(
            blsVK, schnorrVK, depositAmount, IStakeTable.StakeType.Native, sig, validUntilEpoch
        );
    }
}
