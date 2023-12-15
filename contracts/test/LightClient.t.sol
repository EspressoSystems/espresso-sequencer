// SPDX-License-Identifier: Unlicensed

/* solhint-disable contract-name-camelcase, func-name-mixedcase, one-contract-per-file */

pragma solidity ^0.8.0;

// Libraries
import "forge-std/Test.sol";

// Target contract
import { LightClient as LC } from "../src/LightClient.sol";

/// @dev Common helpers for LightClient tests
contract LightClientCommonTest is Test {
    LC lc;
    uint32 public constant BLOCKS_PER_EPOCH_TEST = 2;
    LC.LightClientState public genesis;

    /// @dev initialized ledger like genesis and system params
    function init() public {
        string[] memory cmds = new string[](3);
        cmds[0] = "diff-test";
        cmds[1] = "mock-genesis";
        cmds[2] = vm.toString(BLOCKS_PER_EPOCH_TEST);

        bytes memory result = vm.ffi(cmds);
        (LC.LightClientState memory state, bytes32 votingSTComm, bytes32 frozenSTComm) =
            abi.decode(result, (LC.LightClientState, bytes32, bytes32));

        genesis = state;
        lc = new LC(genesis, BLOCKS_PER_EPOCH_TEST);
        bytes32 expectedStakeTableComm = lc.computeStakeTableComm(state);
        assertEq(votingSTComm, expectedStakeTableComm);
        assertEq(frozenSTComm, expectedStakeTableComm);
    }

    /// @dev assertEq for `struct LightClientState`
    function assertEqState(LC.LightClientState memory a, LC.LightClientState memory b) public {
        assert(a.viewNum == b.viewNum);
        assert(a.blockHeight == b.blockHeight);
        assertEq(a.blockCommRoot, b.blockCommRoot);
        assertEq(a.feeLedgerComm, b.feeLedgerComm);
        assertEq(a.stakeTableBlsKeyComm, b.stakeTableBlsKeyComm);
        assertEq(a.stakeTableSchnorrKeyComm, b.stakeTableSchnorrKeyComm);
        assertEq(a.stakeTableAmountComm, b.stakeTableAmountComm);
        assertEq(a.threshold, b.threshold);
    }

    /// @dev helper getter since solidity doesn't return struct but tuples only
    function getGenesisState() public view returns (LC.LightClientState memory) {
        (
            uint64 viewNum,
            uint64 blockHeight,
            uint256 blockCommRoot,
            uint256 feeLedgerComm,
            uint256 stakeTableBlsKeyComm,
            uint256 stakeTableSchnorrKeyComm,
            uint256 stakeTableAmountComm,
            uint256 threshold
        ) = lc.genesisState();

        return LC.LightClientState(
            viewNum,
            blockHeight,
            blockCommRoot,
            feeLedgerComm,
            stakeTableBlsKeyComm,
            stakeTableSchnorrKeyComm,
            stakeTableAmountComm,
            threshold
        );
    }

    /// @dev helper getter since solidity doesn't return struct but tuples only
    function getFinalizedState() public view returns (LC.LightClientState memory) {
        (
            uint64 viewNum,
            uint64 blockHeight,
            uint256 blockCommRoot,
            uint256 feeLedgerComm,
            uint256 stakeTableBlsKeyComm,
            uint256 stakeTableSchnorrKeyComm,
            uint256 stakeTableAmountComm,
            uint256 threshold
        ) = lc.finalizedState();

        return LC.LightClientState(
            viewNum,
            blockHeight,
            blockCommRoot,
            feeLedgerComm,
            stakeTableBlsKeyComm,
            stakeTableSchnorrKeyComm,
            stakeTableAmountComm,
            threshold
        );
    }
}

contract LightClient_constructor_Test is LightClientCommonTest {
    function setUp() public {
        init();
    }

    /// @dev Test the constructor has initialized the contract state properly, espeically genesis
    /// block.
    function test_GenesisInitialized() external {
        assert(lc.BLOCKS_PER_EPOCH() == BLOCKS_PER_EPOCH_TEST);
        assertEqState(getGenesisState(), genesis);
        assertEqState(getFinalizedState(), genesis);
        assert(lc.currentEpoch() == 0);

        bytes32 stakeTableComm = lc.computeStakeTableComm(genesis);
        assertEq(lc.votingStakeTableCommitment(), stakeTableComm);
        assertEq(lc.frozenStakeTableCommitment(), stakeTableComm);
    }

    // TODO: malformed gensis would revert
    function test_RevertWhen_InvalidGenesis() external {
        LC.LightClientState memory badGenesis = genesis;

        // wrong viewNum would revert
        badGenesis.viewNum = 1;
        vm.expectRevert(LC.InvalidArgs.selector);
        lc = new LC(badGenesis, BLOCKS_PER_EPOCH_TEST);
        badGenesis.viewNum = genesis.viewNum; // revert to correct

        // wrong blockHeight would revert
        badGenesis.blockHeight = 1;
        vm.expectRevert(LC.InvalidArgs.selector);
        lc = new LC(badGenesis, BLOCKS_PER_EPOCH_TEST);
        badGenesis.blockHeight = genesis.blockHeight; // revert to correct

        // zero-valued stake table commitments would revert
        badGenesis.stakeTableBlsKeyComm = 0;
        vm.expectRevert(LC.InvalidArgs.selector);
        lc = new LC(badGenesis, BLOCKS_PER_EPOCH_TEST);
        badGenesis.stakeTableBlsKeyComm = genesis.stakeTableBlsKeyComm; // revert to correct
        badGenesis.stakeTableSchnorrKeyComm = 0;
        vm.expectRevert(LC.InvalidArgs.selector);
        lc = new LC(badGenesis, BLOCKS_PER_EPOCH_TEST);
        badGenesis.stakeTableSchnorrKeyComm = genesis.stakeTableSchnorrKeyComm; // revert to correct
        badGenesis.stakeTableAmountComm = 0;
        vm.expectRevert(LC.InvalidArgs.selector);
        lc = new LC(badGenesis, BLOCKS_PER_EPOCH_TEST);
        badGenesis.stakeTableAmountComm = genesis.stakeTableAmountComm; // revert to correct

        // zero-valued BLOCK_PER_EPOCH would revert
        vm.expectRevert(LC.InvalidArgs.selector);
        lc = new LC(genesis, 0);
    }
}
