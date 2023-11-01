// SPDX-License-Identifier: Unlicensed

/* solhint-disable contract-name-camelcase, func-name-mixedcase, one-contract-per-file */

pragma solidity ^0.8.0;

// Libraries
import "forge-std/Test.sol";
import { BN254 } from "bn254/BN254.sol";
import { IPlonkVerifier } from "../src/interfaces/IPlonkVerifier.sol";

// Target contract
import { Transcript as T } from "../src/libraries/Transcript.sol";

contract Transcript_appendMessage_Test is Test {
    using T for T.TranscriptData;

    /// forge-config: default.fuzz.runs = 10
    /// @dev Test if `appendMessage` matches that of the Jellyfish's code
    function testFuzz_appendMessage_matches(
        T.TranscriptData memory transcript,
        bytes memory message
    ) external {
        string[] memory cmds = new string[](7);
        cmds[0] = "cargo";
        cmds[1] = "run";
        cmds[2] = "--bin";
        cmds[3] = "diff-test";
        cmds[4] = "transcript-append-msg";
        cmds[5] = vm.toString(abi.encode(transcript));
        cmds[6] = vm.toString(abi.encode(message));

        bytes memory result = vm.ffi(cmds);
        (T.TranscriptData memory updated) = abi.decode(result, (T.TranscriptData));

        transcript.appendMessage(message);
        assertEq(updated.transcript, transcript.transcript);
        assertEq(updated.state[0], transcript.state[0]);
        assertEq(updated.state[1], transcript.state[1]);
    }
}

// contract Transcript_appendFieldElement_Test is Test {
//     // TODO:
// }

// contract Transcript_appendGroupElement_Test is Test {
//     // TODO:
// }

// contract Transcript_getAndAppendChallenge_Test is Test {
//     // TODO:
// }

// contract Transcript_appendVkAndPubInput_Test is Test {
//     // TODO:
// }

// contract Transcript_appendProofEvaluations_Test is Test {
//     // TODO:
// }

contract WhateverTest is Test {
    function test_whatever() external {
        T.TranscriptData memory transcript;
        transcript.transcript = "\x03\x04";
        transcript.state[0] = "a";
        transcript.state[1] = "b";

        // bytes memory message = "hi";

        string[] memory cmds = new string[](6);
        // string[] memory cmds = new string[](7);
        cmds[0] = "cargo";
        cmds[1] = "run";
        cmds[2] = "--bin";
        cmds[3] = "diff-test";
        cmds[4] = "test-only";
        cmds[5] = vm.toString(abi.encode(transcript));
        // cmds[6] = vm.toString(message);

        bytes memory result = vm.ffi(cmds);
        (T.TranscriptData memory recv) = abi.decode(result, (T.TranscriptData));
        assertEq(recv.transcript, transcript.transcript);
    }
}
