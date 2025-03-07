// SPDX-License-Identifier: Unlicensed

/* solhint-disable contract-name-camelcase, func-name-mixedcase, one-contract-per-file */

pragma solidity ^0.8.0;

// Libraries
import "forge-std/Test.sol";
import { IPlonkVerifier as V } from "../src/interfaces/IPlonkVerifier.sol";

// Target contract
import { LightClient as LC } from "../src/LightClient.sol";
import { LightClientCommonTest } from "./LightClient.t.sol";

contract LightClientBench is LightClientCommonTest {
    LC.LightClientState state;
    LC.StakeTableState nextStakeTable;
    V.PlonkProof proof;

    function setUp() public {
        init();
        // Generating a few consecutive states and proofs
        string[] memory cmds = new string[](3);
        cmds[0] = "diff-test";
        cmds[1] = "mock-consecutive-finalized-states";
        cmds[2] = vm.toString(STAKE_TABLE_CAPACITY / 2);

        bytes memory result = vm.ffi(cmds);
        (
            LC.LightClientState[] memory states,
            LC.StakeTableState[] memory nextStakeTables,
            V.PlonkProof[] memory proofs
        ) = abi.decode(result, (LC.LightClientState[], LC.StakeTableState[], V.PlonkProof[]));

        state = states[0];
        proof = proofs[0];
        nextStakeTable = nextStakeTables[0];
    }

    /// @dev for benchmarking purposes only
    function testCorrectUpdateBench() external {
        vm.pauseGasMetering();
        LC.LightClientState memory st = state;
        LC.StakeTableState memory nextSTState = nextStakeTable;
        V.PlonkProof memory pf = proof;

        vm.prank(permissionedProver);
        vm.resumeGasMetering();
        lc.newFinalizedState(st, nextSTState, pf);
    }
}
