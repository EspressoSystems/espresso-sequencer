// SPDX-License-Identifier: Unlicensed

/* solhint-disable contract-name-camelcase, func-name-mixedcase, one-contract-per-file */

pragma solidity ^0.8.0;

// Libraries
import "forge-std/Test.sol";
import { BN254 } from "bn254/BN254.sol";
import { IPlonkVerifier } from "../src/interfaces/IPlonkVerifier.sol";
import { VkTest } from "./stubs/Transfer1In2Out24DepthVk.sol";

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

contract Transcript_appendFieldElement_Test is Test {
    using T for T.TranscriptData;

    /// forge-config: default.fuzz.runs = 10
    /// @dev Test if `appendFieldElement` matches that of Jellyfish
    function testFuzz_appendFieldElement_matches(
        T.TranscriptData memory transcript,
        uint256 fieldElement
    ) external {
        fieldElement = bound(fieldElement, 0, BN254.R_MOD - 1);
        BN254.validateScalarField(fieldElement);

        string[] memory cmds = new string[](7);
        cmds[0] = "cargo";
        cmds[1] = "run";
        cmds[2] = "--bin";
        cmds[3] = "diff-test";
        cmds[4] = "transcript-append-field";
        cmds[5] = vm.toString(abi.encode(transcript));
        cmds[6] = vm.toString(bytes32(fieldElement));

        bytes memory result = vm.ffi(cmds);
        (T.TranscriptData memory updated) = abi.decode(result, (T.TranscriptData));

        transcript.appendFieldElement(fieldElement);
        assertEq(updated.transcript, transcript.transcript);
        assertEq(updated.state[0], transcript.state[0]);
        assertEq(updated.state[1], transcript.state[1]);
    }
}

contract Transcript_appendGroupElement_Test is Test {
    using T for T.TranscriptData;

    /// forge-config: default.fuzz.runs = 10
    /// @dev Test if `appendGroupElement` matches that of Jellyfish
    function testFuzz_appendGroupElement_matches(
        T.TranscriptData memory transcript,
        uint256 randScalar
    ) external {
        randScalar = bound(randScalar, 0, BN254.R_MOD - 1);
        BN254.validateScalarField(randScalar);
        BN254.G1Point memory randPoint = BN254.scalarMul(BN254.P1(), randScalar);

        string[] memory cmds = new string[](7);
        cmds[0] = "cargo";
        cmds[1] = "run";
        cmds[2] = "--bin";
        cmds[3] = "diff-test";
        cmds[4] = "transcript-append-group";
        cmds[5] = vm.toString(abi.encode(transcript));
        cmds[6] = vm.toString(abi.encode(randPoint));

        bytes memory result = vm.ffi(cmds);
        (T.TranscriptData memory updated) = abi.decode(result, (T.TranscriptData));

        transcript.appendGroupElement(randPoint);
        assertEq(updated.transcript, transcript.transcript);
        assertEq(updated.state[0], transcript.state[0]);
        assertEq(updated.state[1], transcript.state[1]);
    }
}

contract Transcript_getAndAppendChallenge_Test is Test {
    using T for T.TranscriptData;

    /// forge-config: default.fuzz.runs = 10
    /// @dev Test if `getAndAppendChallenge` matches that of Jellyfish
    function testFuzz_getAndAppendChallenge_matches(T.TranscriptData memory transcript) external {
        string[] memory cmds = new string[](6);
        cmds[0] = "cargo";
        cmds[1] = "run";
        cmds[2] = "--bin";
        cmds[3] = "diff-test";
        cmds[4] = "transcript-get-chal";
        cmds[5] = vm.toString(abi.encode(transcript));

        bytes memory result = vm.ffi(cmds);
        (T.TranscriptData memory updated, uint256 chal) =
            abi.decode(result, (T.TranscriptData, uint256));

        uint256 challenge = transcript.getAndAppendChallenge();

        assertEq(updated.transcript, transcript.transcript);
        assertEq(updated.state[0], transcript.state[0]);
        assertEq(updated.state[1], transcript.state[1]);
        assertEq(chal, challenge);
    }
}

contract Transcript_appendVkAndPubInput_Test is Test {
    using T for T.TranscriptData;

    /// forge-config: default.fuzz.runs = 5
    /// @dev Test if `appendVkAndPubInput` matches that of Jellyfish
    function testFuzz_appendVkAndPubInput_matches(
        T.TranscriptData memory transcript,
        uint256[] memory publicInput
    ) external {
        for (uint256 i = 0; i < publicInput.length; i++) {
            publicInput[i] = bound(publicInput[i], 0, BN254.R_MOD - 1);
            BN254.validateScalarField(publicInput[i]);
        }
        IPlonkVerifier.VerifyingKey memory vk = VkTest.getVk();

        string[] memory cmds = new string[](8);
        cmds[0] = "cargo";
        cmds[1] = "run";
        cmds[2] = "--bin";
        cmds[3] = "diff-test";
        cmds[4] = "transcript-append-vk-and-pi";
        cmds[5] = vm.toString(abi.encode(transcript));
        cmds[6] = vm.toString(abi.encode(vk));
        cmds[7] = vm.toString(abi.encode(publicInput));

        bytes memory result = vm.ffi(cmds);
        (T.TranscriptData memory updated) = abi.decode(result, (T.TranscriptData));

        transcript.appendVkAndPubInput(vk, publicInput);

        assertEq(updated.transcript, transcript.transcript, "transcript field mismatch");
        assertEq(updated.state[0], transcript.state[0], "state[0] field mismatch");
        assertEq(updated.state[1], transcript.state[1], "state[1] field mismatch");
    }
}

// contract Transcript_appendProofEvaluations_Test is Test {
//     // TODO:
// }

// contract WhateverTest is Test {
//     function test_whatever() external {
//         return;
//     }
// }
