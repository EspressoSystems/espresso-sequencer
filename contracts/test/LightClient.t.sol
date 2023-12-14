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
        (LC.LightClientState memory state) = abi.decode(result, (LC.LightClientState));

        genesis = state;
        lc = new LC(genesis, BLOCKS_PER_EPOCH_TEST);
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
}

contract LightClient_constructor_Test is LightClientCommonTest {
    function setUp() public {
        init();
    }

    /// @dev Test the constructor has initialized the contract state properly, espeically genesis
    /// block.
    function test_GenesisInitialized() external {
        assert(lc.BLOCKS_PER_EPOCH() == BLOCKS_PER_EPOCH_TEST);
        // assertEqState(lc.genesisState, genesis);
        // assertEqState(lc.finalizedState, genesis);
        assert(lc.currentEpoch() == 0);
    }

    // TODO: malformed gensis would revert
}
