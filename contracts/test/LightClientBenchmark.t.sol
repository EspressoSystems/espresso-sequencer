// SPDX-License-Identifier: Unlicensed

/* solhint-disable contract-name-camelcase, func-name-mixedcase, one-contract-per-file */

pragma solidity ^0.8.0;

// Libraries
import "forge-std/Test.sol";
import { IPlonkVerifier as V } from "../src/interfaces/IPlonkVerifier.sol";

// Target contract
import { LightClient as LC } from "../src/LightClient.sol";
import { LightClientCommonTest } from "./LightClient.t.sol";

contract LightClient_newFinalizedState_Test is LightClientCommonTest {
    function setUp() public {
        init();
    }

    /// @dev for benchmarking purposes only
    function testCorrectUpdateBench() external {
        vm.pauseGasMetering();
        // Generating a few consecutive states and proofs
        string[] memory cmds = new string[](5);
        cmds[0] = "diff-test";
        cmds[1] = "mock-consecutive-finalized-states";
        cmds[2] = vm.toString(STAKE_TABLE_CAPACITY / 2);
        cmds[3] = vm.toString(uint64(3));
        cmds[4] = vm.toString(uint64(3));

        bytes memory result = vm.ffi(cmds);
        (LC.LightClientState[] memory states, V.PlonkProof[] memory proofs, ) =
            abi.decode(result, (LC.LightClientState[], V.PlonkProof[], LC.StakeState[]));
        vm.prank(permissionedProver);
        vm.resumeGasMetering();
        lc.newFinalizedState(states[0], proofs[0]);
    }
}
