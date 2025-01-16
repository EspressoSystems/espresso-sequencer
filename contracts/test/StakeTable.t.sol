// SPDX-License-Identifier: Unlicensed

/* solhint-disable contract-name-camelcase, func-name-mixedcase, one-contract-per-file */

pragma solidity ^0.8.0;

// Libraries
import "forge-std/Test.sol";
// import {console} from "forge-std/console.sol";

using stdStorage for StdStorage;

import { ERC20 } from "solmate/utils/SafeTransferLib.sol";
import { BN254 } from "bn254/BN254.sol";
import { BLSSig } from "../src/libraries/BLSSig.sol";
import { EdOnBN254 } from "../src/libraries/EdOnBn254.sol";
import { AbstractStakeTable } from "../src/interfaces/AbstractStakeTable.sol";
import { LightClient } from "../src/LightClient.sol";
import { LightClientMock } from "../test/mocks/LightClientMock.sol";

// Token contract
import { ExampleToken } from "../src/ExampleToken.sol";

// Target contract
import { StakeTable as S } from "../src/StakeTable.sol";

contract StakeTable_register_Test is Test {
    S public stakeTable;
    ExampleToken public token;
    LightClientMock public lcMock;
    uint256 public constant INITIAL_BALANCE = 10 ether;
    address public exampleTokenCreator;

    function genClientWallet(address sender, string memory seed)
        private
        returns (BN254.G2Point memory, EdOnBN254.EdOnBN254Point memory, BN254.G1Point memory)
    {
        // Generate a BLS signature and other values using rust code
        string[] memory cmds = new string[](4);
        cmds[0] = "diff-test";
        cmds[1] = "gen-client-wallet";
        cmds[2] = vm.toString(sender);
        cmds[3] = seed;

        bytes memory result = vm.ffi(cmds);
        (
            BN254.G1Point memory blsSig,
            BN254.G2Point memory blsVK,
            uint256 schnorrVKx,
            uint256 schnorrVKy,
        ) = abi.decode(result, (BN254.G1Point, BN254.G2Point, uint256, uint256, address));

        return (
            blsVK, // blsVK
            EdOnBN254.EdOnBN254Point(schnorrVKx, schnorrVKy), // schnorrVK
            blsSig // sig
        );
    }

    function setUp() public {
        exampleTokenCreator = makeAddr("tokenCreator");
        vm.prank(exampleTokenCreator);
        token = new ExampleToken(INITIAL_BALANCE);

        string[] memory cmds = new string[](3);
        cmds[0] = "diff-test";
        cmds[1] = "mock-genesis";
        cmds[2] = "5";

        bytes memory result = vm.ffi(cmds);
        (
            LightClientMock.LightClientState memory state,
            LightClientMock.StakeTableState memory stakeState
        ) = abi.decode(result, (LightClient.LightClientState, LightClient.StakeTableState));
        LightClientMock.LightClientState memory genesis = state;
        LightClientMock.StakeTableState memory genesisStakeTableState = stakeState;

        lcMock = new LightClientMock(genesis, genesisStakeTableState, 864000);
        address lightClientAddress = address(lcMock);
        stakeTable = new S(address(token), lightClientAddress, 10);
    }

    function testFuzz_RevertWhen_InvalidBLSSig(uint256 scalar) external {
        uint64 depositAmount = 10 ether;
        uint64 validUntilEpoch = 5;
        string memory seed = "123";

        (BN254.G2Point memory blsVK, EdOnBN254.EdOnBN254Point memory schnorrVK,) =
            genClientWallet(exampleTokenCreator, seed);

        // Prepare for the token transfer
        vm.startPrank(exampleTokenCreator);
        token.approve(address(stakeTable), depositAmount);

        // Ensure the scalar is valid
        // Note: Apparently BN254.scalarMul is not well defined when the scalar is 0
        scalar = bound(scalar, 1, BN254.R_MOD - 1);
        BN254.validateScalarField(BN254.ScalarField.wrap(scalar));
        BN254.G1Point memory badSig = BN254.scalarMul(BN254.P1(), BN254.ScalarField.wrap(scalar));
        BN254.validateG1Point(badSig);

        // Failed signature verification
        vm.expectRevert(BLSSig.BLSSigVerificationFailed.selector);
        stakeTable.register(blsVK, schnorrVK, depositAmount, badSig, validUntilEpoch);
        vm.stopPrank();
    }

    // commenting out epoch related tests for now
    // function testFuzz_RevertWhen_InvalidNextRegistrationEpoch(uint64 rand) external {
    //     LCMock.setCurrentEpoch(3);
    //     uint64 currentEpoch = stakeTable.currentEpoch();

    //     uint64 depositAmount = 10 ether;
    //     vm.prank(exampleTokenCreator);
    //     token.approve(address(stakeTable), depositAmount);

    //     (
    //         BN254.G2Point memory blsVK,
    //         EdOnBN254.EdOnBN254Point memory schnorrVK,
    //         BN254.G1Point memory sig
    //     ) = genClientWallet(exampleTokenCreator);

    //     // Invalid next registration epoch
    //     uint64 validUntilEpoch = uint64(bound(rand, 0, currentEpoch - 1));
    //     vm.prank(exampleTokenCreator);
    //     vm.expectRevert(
    //         abi.encodeWithSelector(
    //             S.InvalidNextRegistrationEpoch.selector, currentEpoch + 1, validUntilEpoch
    //         )
    //     );
    //     stakeTable.register(
    //         blsVK,
    //         schnorrVK,
    //         depositAmount,
    //         sig,
    //         validUntilEpoch
    //     );

    //     // Valid next registration epoch
    //     validUntilEpoch = uint64(bound(rand, currentEpoch + 1, type(uint64).max));
    //     vm.prank(exampleTokenCreator);
    //     stakeTable.register(
    //         blsVK,
    //         schnorrVK,
    //         depositAmount,
    //         sig,
    //         validUntilEpoch
    //     );
    // }

    function test_RevertWhen_NodeAlreadyRegistered() external {
        uint64 depositAmount = 10 ether;
        uint64 validUntilEpoch = 5;
        string memory seed = "123";

        (
            BN254.G2Point memory blsVK,
            EdOnBN254.EdOnBN254Point memory schnorrVK,
            BN254.G1Point memory sig
        ) = genClientWallet(exampleTokenCreator, seed);

        // Prepare for the token transfer
        vm.prank(exampleTokenCreator);
        token.approve(address(stakeTable), depositAmount);

        // Successful call to register
        vm.prank(exampleTokenCreator);
        stakeTable.register(blsVK, schnorrVK, depositAmount, sig, validUntilEpoch);

        // The node is already registered
        vm.prank(exampleTokenCreator);
        vm.expectRevert(S.NodeAlreadyRegistered.selector);
        stakeTable.register(blsVK, schnorrVK, depositAmount, sig, validUntilEpoch);
    }

    function test_RevertWhen_NoTokenAllowanceOrBalance() external {
        uint64 depositAmount = 10 ether;
        uint64 validUntilEpoch = 10;
        string memory seed = "123";

        (
            BN254.G2Point memory blsVK,
            EdOnBN254.EdOnBN254Point memory schnorrVK,
            BN254.G1Point memory sig
        ) = genClientWallet(exampleTokenCreator, seed);

        assertEq(ERC20(token).balanceOf(exampleTokenCreator), INITIAL_BALANCE);
        vm.prank(exampleTokenCreator);
        // The call to register is expected to fail because the depositAmount has not been approved
        // and thus the stake table contract cannot lock the stake.
        vm.expectRevert(abi.encodeWithSelector(S.InsufficientAllowance.selector, 0, depositAmount));
        stakeTable.register(blsVK, schnorrVK, depositAmount, sig, validUntilEpoch);

        // A user with 0 balance cannot register either
        address newUser = makeAddr("New user with zero balance");
        (blsVK, schnorrVK, sig) = genClientWallet(newUser, seed);

        vm.startPrank(newUser);
        // Prepare for the token transfer by giving the StakeTable contract the required allowance
        token.approve(address(stakeTable), depositAmount);
        vm.expectRevert(abi.encodeWithSelector(S.InsufficientBalance.selector, 0));
        stakeTable.register(blsVK, schnorrVK, depositAmount, sig, validUntilEpoch);
        vm.stopPrank();
    }

    function test_RevertWhen_WrongStakeAmount() external {
        uint64 depositAmount = 5 ether;
        uint64 validUntilEpoch = 10;
        string memory seed = "123";

        (
            BN254.G2Point memory blsVK,
            EdOnBN254.EdOnBN254Point memory schnorrVK,
            BN254.G1Point memory sig
        ) = genClientWallet(exampleTokenCreator, seed);

        assertEq(ERC20(token).balanceOf(exampleTokenCreator), INITIAL_BALANCE);
        vm.prank(exampleTokenCreator);
        // The call to register is expected to fail because the correct depositAmount has not been
        // approved/supplied
        // and thus the stake table contract cannot lock the stake.
        vm.expectRevert(abi.encodeWithSelector(S.InsufficientStakeAmount.selector, depositAmount));
        stakeTable.register(blsVK, schnorrVK, depositAmount, sig, validUntilEpoch);
    }

    /// @dev Tests a correct registration
    function test_Registration_succeeds() external {
        uint64 depositAmount = 10 ether;
        uint64 validUntilEpoch = 5;
        string memory seed = "123";

        (
            BN254.G2Point memory blsVK,
            EdOnBN254.EdOnBN254Point memory schnorrVK,
            BN254.G1Point memory sig
        ) = genClientWallet(exampleTokenCreator, seed);

        // Prepare for the token transfer
        vm.prank(exampleTokenCreator);
        token.approve(address(stakeTable), depositAmount);

        // Balances before registration
        assertEq(token.balanceOf(exampleTokenCreator), INITIAL_BALANCE);

        uint256 totalStakeAmount;
        totalStakeAmount = stakeTable.totalStake();
        assertEq(totalStakeAmount, 0);

        // Check event is emitted after calling successfully `register`
        vm.expectEmit(false, false, false, true, address(stakeTable));
        emit AbstractStakeTable.Registered(exampleTokenCreator, 1, depositAmount);
        vm.prank(exampleTokenCreator);
        stakeTable.register(blsVK, schnorrVK, depositAmount, sig, validUntilEpoch);

        // Balance after registration
        assertEq(token.balanceOf(exampleTokenCreator), INITIAL_BALANCE - depositAmount);
        totalStakeAmount = stakeTable.totalStake();
        assertEq(totalStakeAmount, depositAmount);

        // lookup the node and verify the data
        AbstractStakeTable.Node memory node = stakeTable.lookupNode(exampleTokenCreator);
        assertEq(node.account, exampleTokenCreator);
        assertEq(node.balance, depositAmount);
        assertEq(node.registerEpoch, 1);
        assertTrue(stakeTable._isEqualBlsKey(node.blsVK, blsVK));
        assertTrue(EdOnBN254.isEqual(node.schnorrVK, schnorrVK));

        // lookup the stake and verify the amount
        uint256 stakeAmount = stakeTable.lookupStake(exampleTokenCreator);
        assertEq(stakeAmount, depositAmount);
    }

    /// @dev Tests a correct registration
    function test_RevertWhen_InvalidBlsVK_or_InvalidSchnorrVK_on_Registration() external {
        uint64 depositAmount = 10 ether;
        uint64 validUntilEpoch = 5;
        string memory seed = "123";

        // generate a valid blsVK and schnorrVK
        (
            BN254.G2Point memory blsVK,
            EdOnBN254.EdOnBN254Point memory schnorrVK,
            BN254.G1Point memory sig
        ) = genClientWallet(exampleTokenCreator, seed);

        // Prepare for the token transfer
        vm.startPrank(exampleTokenCreator);
        token.approve(address(stakeTable), depositAmount);

        // revert when the blsVK is the zero point
        BN254.G2Point memory zeroBlsVK = BN254.G2Point(
            BN254.BaseField.wrap(0),
            BN254.BaseField.wrap(0),
            BN254.BaseField.wrap(0),
            BN254.BaseField.wrap(0)
        );
        vm.expectRevert(S.InvalidBlsVK.selector);
        stakeTable.register(zeroBlsVK, schnorrVK, depositAmount, sig, validUntilEpoch);

        // revert when the schnorrVK is the zero point
        EdOnBN254.EdOnBN254Point memory zeroSchnorrVK = EdOnBN254.EdOnBN254Point(0, 0);
        vm.expectRevert(S.InvalidSchnorrVK.selector);
        stakeTable.register(blsVK, zeroSchnorrVK, depositAmount, sig, validUntilEpoch);

        // lookup the node and verify the data but expect the node to be empty
        AbstractStakeTable.Node memory node = stakeTable.lookupNode(exampleTokenCreator);
        assertEq(node.account, address(0));

        vm.stopPrank();
    }

    function test_UpdateConsensusKeys_Succeeds() public {
        uint64 depositAmount = 10 ether;
        uint64 validUntilEpoch = 5;
        string memory seed = "123";

        //Step 1: generate a new blsVK and schnorrVK and register this node
        (
            BN254.G2Point memory blsVK,
            EdOnBN254.EdOnBN254Point memory schnorrVK,
            BN254.G1Point memory sig
        ) = genClientWallet(exampleTokenCreator, seed);

        // Prepare for the token transfer by granting allowance to the contract
        vm.startPrank(exampleTokenCreator);
        token.approve(address(stakeTable), depositAmount);

        // Balances before registration
        assertEq(token.balanceOf(exampleTokenCreator), INITIAL_BALANCE);

        // Check event is emitted after calling successfully `register`
        vm.expectEmit(false, false, false, true, address(stakeTable));
        emit AbstractStakeTable.Registered(exampleTokenCreator, 1, depositAmount);
        stakeTable.register(blsVK, schnorrVK, depositAmount, sig, validUntilEpoch);

        // Step 2: generate a new blsVK and schnorrVK
        seed = "234";
        (
            BN254.G2Point memory newBlsVK,
            EdOnBN254.EdOnBN254Point memory newSchnorrVK,
            BN254.G1Point memory newBlsSig
        ) = genClientWallet(exampleTokenCreator, seed);

        // assert that the new blsVK and schnorrVK are not the same as the old ones
        assertFalse(stakeTable._isEqualBlsKey(newBlsVK, blsVK));
        assertFalse(EdOnBN254.isEqual(newSchnorrVK, schnorrVK));

        // Step 3: update the consensus keys
        vm.expectEmit(false, false, false, true, address(stakeTable));
        emit AbstractStakeTable.UpdatedConsensusKeys(exampleTokenCreator, newBlsVK, newSchnorrVK);
        stakeTable.updateConsensusKeys(newBlsVK, newSchnorrVK, newBlsSig);

        // Step 4: verify the update
        AbstractStakeTable.Node memory node = stakeTable.lookupNode(exampleTokenCreator);
        assertTrue(stakeTable._isEqualBlsKey(node.blsVK, newBlsVK));
        assertTrue(EdOnBN254.isEqual(node.schnorrVK, newSchnorrVK));
        assertEq(node.balance, depositAmount);
        assertEq(node.account, exampleTokenCreator);

        vm.stopPrank();
    }

    function test_RevertWhen_UpdateConsensusKeysWithSameKeys() public {
        uint64 depositAmount = 10 ether;
        uint64 validUntilEpoch = 5;
        string memory seed = "123";

        //Step 1: generate a new blsVK and schnorrVK and register this node
        (
            BN254.G2Point memory blsVK,
            EdOnBN254.EdOnBN254Point memory schnorrVK,
            BN254.G1Point memory sig
        ) = genClientWallet(exampleTokenCreator, seed);

        // Prepare for the token transfer by granting allowance to the contract
        vm.startPrank(exampleTokenCreator);
        token.approve(address(stakeTable), depositAmount);

        // Balances before registration
        assertEq(token.balanceOf(exampleTokenCreator), INITIAL_BALANCE);

        stakeTable.register(blsVK, schnorrVK, depositAmount, sig, validUntilEpoch);

        // Step 2: update the consensus keys with the same keys
        vm.expectRevert(S.NoKeyChange.selector);
        stakeTable.updateConsensusKeys(blsVK, schnorrVK, sig);

        vm.stopPrank();
    }

    function test_RevertWhen_UpdateConsensusKeysWithEmptyKeys() public {
        uint64 depositAmount = 10 ether;
        uint64 validUntilEpoch = 5;
        string memory seed = "123";

        //Step 1: generate a new blsVK and schnorrVK and register this node
        (
            BN254.G2Point memory blsVK,
            EdOnBN254.EdOnBN254Point memory schnorrVK,
            BN254.G1Point memory sig
        ) = genClientWallet(exampleTokenCreator, seed);

        // Prepare for the token transfer by granting allowance to the contract
        vm.startPrank(exampleTokenCreator);
        token.approve(address(stakeTable), depositAmount);

        // Balances before registration
        assertEq(token.balanceOf(exampleTokenCreator), INITIAL_BALANCE);

        stakeTable.register(blsVK, schnorrVK, depositAmount, sig, validUntilEpoch);

        // empty keys
        BN254.G2Point memory emptyBlsVK = BN254.G2Point(
            BN254.BaseField.wrap(0),
            BN254.BaseField.wrap(0),
            BN254.BaseField.wrap(0),
            BN254.BaseField.wrap(0)
        );
        EdOnBN254.EdOnBN254Point memory emptySchnorrVK = EdOnBN254.EdOnBN254Point(0, 0);

        // Step 2: attempt to update the consensus keys with the same keys
        vm.expectRevert(S.InvalidBlsVK.selector);
        stakeTable.updateConsensusKeys(emptyBlsVK, emptySchnorrVK, sig);

        vm.stopPrank();
    }

    function test_RevertWhen_UpdateConsensusKeysWithInvalidSignature() public {
        uint64 depositAmount = 10 ether;
        uint64 validUntilEpoch = 5;
        string memory seed = "123";

        //Step 1: generate a new blsVK and schnorrVK and register this node
        (
            BN254.G2Point memory blsVK,
            EdOnBN254.EdOnBN254Point memory schnorrVK,
            BN254.G1Point memory sig
        ) = genClientWallet(exampleTokenCreator, seed);

        // Prepare for the token transfer by granting allowance to the contract
        vm.startPrank(exampleTokenCreator);
        token.approve(address(stakeTable), depositAmount);

        // Balances before registration
        assertEq(token.balanceOf(exampleTokenCreator), INITIAL_BALANCE);

        BN254.G1Point memory badSig =
            BN254.G1Point(BN254.BaseField.wrap(0), BN254.BaseField.wrap(0));

        stakeTable.register(blsVK, schnorrVK, depositAmount, sig, validUntilEpoch);

        // Step 2: generate a new blsVK and schnorrVK
        seed = "234";
        (BN254.G2Point memory newBlsVK, EdOnBN254.EdOnBN254Point memory newSchnorrVK,) =
            genClientWallet(exampleTokenCreator, seed);

        // Step 3: attempt to update the consensus keys with the new keys but invalid signature
        vm.expectRevert(BLSSig.BLSSigVerificationFailed.selector);
        stakeTable.updateConsensusKeys(newBlsVK, newSchnorrVK, badSig);

        vm.stopPrank();
    }

    function test_RevertWhen_UpdateConsensusKeysWithZeroBlsKeyButNewSchnorrVK() public {
        uint64 depositAmount = 10 ether;
        uint64 validUntilEpoch = 5;
        string memory seed = "123";

        //Step 1: generate a new blsVK and schnorrVK and register this node
        (
            BN254.G2Point memory blsVK,
            EdOnBN254.EdOnBN254Point memory schnorrVK,
            BN254.G1Point memory sig
        ) = genClientWallet(exampleTokenCreator, seed);

        // Prepare for the token transfer by granting allowance to the contract
        vm.startPrank(exampleTokenCreator);
        token.approve(address(stakeTable), depositAmount);

        // Balances before registration
        assertEq(token.balanceOf(exampleTokenCreator), INITIAL_BALANCE);

        vm.expectEmit(false, false, false, true, address(stakeTable));
        emit AbstractStakeTable.Registered(exampleTokenCreator, 1, depositAmount);
        stakeTable.register(blsVK, schnorrVK, depositAmount, sig, validUntilEpoch);

        // Step 2: generate an empty and new schnorrVK
        seed = "234";
        (, EdOnBN254.EdOnBN254Point memory newSchnorrVK,) =
            genClientWallet(exampleTokenCreator, seed);

        BN254.G2Point memory emptyBlsVK = BN254.G2Point(
            BN254.BaseField.wrap(0),
            BN254.BaseField.wrap(0),
            BN254.BaseField.wrap(0),
            BN254.BaseField.wrap(0)
        );

        // Step 3: update the consensus keys with the new schnorr Key but zero bls key which
        // indicates no change in the bls key
        vm.expectRevert(S.InvalidBlsVK.selector);
        stakeTable.updateConsensusKeys(emptyBlsVK, newSchnorrVK, sig);

        vm.stopPrank();
    }

    function test_RevertWhen_UpdateConsensusKeysWithZeroSchnorrVKButNewBlsVK() public {
        uint64 depositAmount = 10 ether;
        uint64 validUntilEpoch = 5;
        string memory seed = "123";

        //Step 1: generate a new blsVK and schnorrVK and register this node
        (
            BN254.G2Point memory blsVK,
            EdOnBN254.EdOnBN254Point memory schnorrVK,
            BN254.G1Point memory sig
        ) = genClientWallet(exampleTokenCreator, seed);

        // Prepare for the token transfer by granting allowance to the contract
        vm.startPrank(exampleTokenCreator);
        token.approve(address(stakeTable), depositAmount);

        // Balances before registration
        assertEq(token.balanceOf(exampleTokenCreator), INITIAL_BALANCE);

        stakeTable.register(blsVK, schnorrVK, depositAmount, sig, validUntilEpoch);

        // Step 2: generate a new blsVK
        seed = "234";
        (BN254.G2Point memory newBlsVK,, BN254.G1Point memory newSig) =
            genClientWallet(exampleTokenCreator, seed);

        // Step 3: generate empty schnorrVK
        EdOnBN254.EdOnBN254Point memory emptySchnorrVK = EdOnBN254.EdOnBN254Point(0, 0);

        // Step 4: update the consensus keys with the new bls keys but empty schnorrVK
        vm.expectRevert(S.InvalidSchnorrVK.selector);
        stakeTable.updateConsensusKeys(newBlsVK, emptySchnorrVK, newSig);

        vm.stopPrank();
    }

    function test_UpdateConsensusKeysWithSameBlsKeyButNewSchnorrVK_Succeeds() public {
        uint64 depositAmount = 10 ether;
        uint64 validUntilEpoch = 5;
        string memory seed = "123";

        //Step 1: generate a new blsVK and schnorrVK and register this node
        (
            BN254.G2Point memory blsVK,
            EdOnBN254.EdOnBN254Point memory schnorrVK,
            BN254.G1Point memory blsSig
        ) = genClientWallet(exampleTokenCreator, seed);

        // Prepare for the token transfer by granting allowance to the contract
        vm.startPrank(exampleTokenCreator);
        token.approve(address(stakeTable), depositAmount);

        // Balances before registration
        assertEq(token.balanceOf(exampleTokenCreator), INITIAL_BALANCE);

        vm.expectEmit(false, false, false, true, address(stakeTable));
        emit AbstractStakeTable.Registered(exampleTokenCreator, 1, depositAmount);
        stakeTable.register(blsVK, schnorrVK, depositAmount, blsSig, validUntilEpoch);

        // Step 2: generate a new schnorrVK
        seed = "234";
        (, EdOnBN254.EdOnBN254Point memory newSchnorrVK,) =
            genClientWallet(exampleTokenCreator, seed);

        // Step 3: update the consensus keys with the new schnorrVK
        vm.expectEmit(false, false, false, true, address(stakeTable));
        emit AbstractStakeTable.UpdatedConsensusKeys(exampleTokenCreator, blsVK, newSchnorrVK);
        stakeTable.updateConsensusKeys(blsVK, newSchnorrVK, blsSig);

        // Step 4: verify the update
        AbstractStakeTable.Node memory node = stakeTable.lookupNode(exampleTokenCreator);
        assertTrue(stakeTable._isEqualBlsKey(node.blsVK, blsVK)); // same as current bls vk
        assertTrue(EdOnBN254.isEqual(node.schnorrVK, newSchnorrVK)); // new schnorr vk
        assertEq(node.balance, depositAmount); //same balance
        assertEq(node.account, exampleTokenCreator); //same account

        vm.stopPrank();
    }

    function test_UpdateConsensusKeysWithNewBlsKeyButSameSchnorrVK_Succeeds() public {
        uint64 depositAmount = 10 ether;
        uint64 validUntilEpoch = 5;
        string memory seed = "123";

        //Step 1: generate a new blsVK and schnorrVK and register this node
        (
            BN254.G2Point memory blsVK,
            EdOnBN254.EdOnBN254Point memory schnorrVK,
            BN254.G1Point memory sig
        ) = genClientWallet(exampleTokenCreator, seed);

        // Prepare for the token transfer by granting allowance to the contract
        vm.startPrank(exampleTokenCreator);
        token.approve(address(stakeTable), depositAmount);

        // Balances before registration
        assertEq(token.balanceOf(exampleTokenCreator), INITIAL_BALANCE);

        vm.expectEmit(false, false, false, true, address(stakeTable));
        emit AbstractStakeTable.Registered(exampleTokenCreator, 1, depositAmount);
        stakeTable.register(blsVK, schnorrVK, depositAmount, sig, validUntilEpoch);

        // Step 2: generate an empty and new schnorrVK
        seed = "234";
        (BN254.G2Point memory newBlsVK,, BN254.G1Point memory newSig) =
            genClientWallet(exampleTokenCreator, seed);

        // Step 3: update the consensus keys with the same bls keys but new schnorrV
        vm.expectEmit(false, false, false, true, address(stakeTable));
        emit AbstractStakeTable.UpdatedConsensusKeys(exampleTokenCreator, newBlsVK, schnorrVK);
        stakeTable.updateConsensusKeys(newBlsVK, schnorrVK, newSig);

        // Step 4: verify the update
        AbstractStakeTable.Node memory node = stakeTable.lookupNode(exampleTokenCreator);
        assertTrue(stakeTable._isEqualBlsKey(node.blsVK, newBlsVK)); // same as current bls vk
        assertTrue(EdOnBN254.isEqual(node.schnorrVK, schnorrVK)); // same as current schnorr vk
        assertEq(node.balance, depositAmount); //same balance
        assertEq(node.account, exampleTokenCreator); //same account

        vm.stopPrank();
    }

    function test_lookupNodeAndLookupStake_fails() public {
        address randomUser = makeAddr("randomUser");

        // lookup the stake for an address that is not registered and verify the amount is empty
        uint256 stakeAmount = stakeTable.lookupStake(randomUser);
        assertEq(stakeAmount, 0);

        // lookup the node for an address that is not registered and verify the data is empty
        AbstractStakeTable.Node memory node = stakeTable.lookupNode(randomUser);
        assertEq(node.account, address(0));
        assertEq(node.balance, 0);
        assertEq(node.registerEpoch, 0);
        assertTrue(
            stakeTable._isEqualBlsKey(
                node.blsVK,
                BN254.G2Point(
                    BN254.BaseField.wrap(0),
                    BN254.BaseField.wrap(0),
                    BN254.BaseField.wrap(0),
                    BN254.BaseField.wrap(0)
                )
            )
        );
        assertTrue(EdOnBN254.isEqual(node.schnorrVK, EdOnBN254.EdOnBN254Point(0, 0)));

        // look up the stake for the zero address and verify the amount is empty
        stakeAmount = stakeTable.lookupStake(address(0));
        assertEq(stakeAmount, 0);

        // look up the node for the zero address and verify the data is empty
        node = stakeTable.lookupNode(address(0));
        assertEq(node.account, address(0));
        assertEq(node.balance, 0);
        assertEq(node.registerEpoch, 0);
        assertTrue(
            stakeTable._isEqualBlsKey(
                node.blsVK,
                BN254.G2Point(
                    BN254.BaseField.wrap(0),
                    BN254.BaseField.wrap(0),
                    BN254.BaseField.wrap(0),
                    BN254.BaseField.wrap(0)
                )
            )
        );
        assertTrue(EdOnBN254.isEqual(node.schnorrVK, EdOnBN254.EdOnBN254Point(0, 0)));
    }

    function test_WithdrawFunds_succeeds() public {
        // Register the node and set exit epoch
        uint64 depositAmount = 10 ether;
        uint64 validUntilEpoch = 5;
        string memory seed = "123";

        (
            BN254.G2Point memory blsVK,
            EdOnBN254.EdOnBN254Point memory schnorrVK,
            BN254.G1Point memory sig
        ) = genClientWallet(exampleTokenCreator, seed);

        // Prepare for the token transfer by granting allowance to the contract
        vm.startPrank(exampleTokenCreator);
        token.approve(address(stakeTable), depositAmount);

        // Balances before registration
        assertEq(token.balanceOf(exampleTokenCreator), INITIAL_BALANCE);

        // register the node
        vm.expectEmit(false, false, false, true, address(stakeTable));
        emit AbstractStakeTable.Registered(exampleTokenCreator, 1, depositAmount);
        stakeTable.register(blsVK, schnorrVK, depositAmount, sig, validUntilEpoch);

        // Withdraw the funds
        uint256 balance = stakeTable.withdrawFunds();

        // verify the withdraw
        assertEq(balance, depositAmount);
        assertEq(token.balanceOf(exampleTokenCreator), INITIAL_BALANCE);
        assertEq(stakeTable.totalStake(), 0);
        assertEq(stakeTable.lookupNode(exampleTokenCreator).balance, 0);
        assertEq(stakeTable.lookupNode(exampleTokenCreator).account, address(0));

        // test withdraw fails if the user tries to withdraw again
        vm.expectRevert(S.NodeNotRegistered.selector);
        stakeTable.withdrawFunds();

        vm.stopPrank();
    }

    function test_WithdrawFunds_RevertWhen_NodeNotRegistered() public {
        // Register the node and set exit epoch
        uint64 depositAmount = 10 ether;
        uint64 validUntilEpoch = 5;
        string memory seed = "123";

        // generate a new blsVK and schnorrVK and register this node
        (
            BN254.G2Point memory blsVK,
            EdOnBN254.EdOnBN254Point memory schnorrVK,
            BN254.G1Point memory sig
        ) = genClientWallet(exampleTokenCreator, seed);

        // Prepare for the token transfer by granting allowance to the contract
        vm.startPrank(exampleTokenCreator);
        token.approve(address(stakeTable), depositAmount);

        // register the node
        vm.expectEmit(false, false, false, true, address(stakeTable));
        emit AbstractStakeTable.Registered(exampleTokenCreator, 1, depositAmount);
        stakeTable.register(blsVK, schnorrVK, depositAmount, sig, validUntilEpoch);

        vm.stopPrank();

        vm.startPrank(makeAddr("randomUser"));
        // withdraw the funds
        vm.expectRevert(S.NodeNotRegistered.selector);
        stakeTable.withdrawFunds();
        vm.stopPrank();
    }
}
