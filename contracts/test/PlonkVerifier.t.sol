// SPDX-License-Identifier: Unlicensed

/* solhint-disable contract-name-camelcase, func-name-mixedcase, one-contract-per-file */

pragma solidity ^0.8.0;

// Libraries
import "forge-std/Test.sol";
import { BN254 } from "bn254/BN254.sol";
import { IPlonkVerifier } from "../src/interfaces/IPlonkVerifier.sol";
import { VkTest } from "./stubs/Transfer1In2Out24DepthVk.sol";

// Target contract
import { PlonkVerifier as V } from "../src/libraries/PlonkVerifier.sol";

/// @dev Common helpers/utils for PlonkVerifier tests
contract PlonkVerifierCommonTest is Test {
    /// @dev Sanitize all values in `a` to be valid scalar fields Bn254::Fr.
    /// This is helpful to sanitize fuzzer-generated random `uint[]` values.
    function sanitizeScalarFields(uint256[] memory a) public returns (uint256[] memory) {
        for (uint256 i = 0; i < a.length; i++) {
            a[i] = bound(a[i], 0, BN254.R_MOD - 1);
            BN254.validateScalarField(a[i]);
        }
        return a;
    }

    /// @dev Generate a random valid (format-wise) proof from a random seed
    function dummyProof(uint64 seed) public returns (IPlonkVerifier.PlonkProof memory) {
        string[] memory cmds = new string[](6);
        cmds[0] = "cargo";
        cmds[1] = "run";
        cmds[2] = "--bin";
        cmds[3] = "diff-test";
        cmds[4] = "dummy-proof";
        cmds[5] = vm.toString(seed);

        bytes memory result = vm.ffi(cmds);
        (IPlonkVerifier.PlonkProof memory proof) = abi.decode(result, (IPlonkVerifier.PlonkProof));
        return proof;
    }
}

contract PlonkVerifier_constants_Test is Test {
    /// @dev Test constants declared matches that from Jellyfish
    function test_correctConstants() external {
        string[] memory cmds = new string[](7);
        cmds[0] = "cargo";
        cmds[1] = "run";
        cmds[2] = "--bin";
        cmds[3] = "diff-test";
        cmds[4] = "plonk-constants";

        bytes memory result = vm.ffi(cmds);
        (
            uint256 k1,
            uint256 k2,
            uint256 k3,
            uint256 k4,
            uint256 betaHX0,
            uint256 betaHX1,
            uint256 betaHY0,
            uint256 betaHY1
        ) = abi.decode(
            result, (uint256, uint256, uint256, uint256, uint256, uint256, uint256, uint256)
        );

        assertEq(k1, V.COSET_K1);
        assertEq(k2, V.COSET_K2);
        assertEq(k3, V.COSET_K3);
        assertEq(k4, V.COSET_K4);
        assertEq(betaHX0, V.BETA_H_X0);
        assertEq(betaHX1, V.BETA_H_X1);
        assertEq(betaHY0, V.BETA_H_Y0);
        assertEq(betaHY1, V.BETA_H_Y1);
    }
}

contract PlonkVerifier_batchVerify_Test is Test {
    /// @dev Test if some of the user inputs are invalid
    function testFuzz_revertWhenInvalidArgs() external {
        // TODO: break down into more possible reasons of malformed inputs.
        return;
    }

    /// @dev Test happy path for `batchVerify`
    function test_correctProof_succeeds() external {
        return;
    }

    /// @dev Test bad path for `batchVerify`: valid bur wrong proof should return false
    function testFuzz_wrongProof_fails() external {
        return;
    }
}

contract PlonkVerifier_validateProof_Test is Test {
    /// @dev Test `_validateProof` correct catches invalidly form proof
    function testFuzz_validateProof_succeeds() external {
        return;
    }
}

contract PlonkVerifier_preparePcsInfo_Test is Test {
    /// @dev Test `preparePcsInfo` matches that of Jellyfish
    function testFuzz_preparePcsInfo_matches() external {
        return;
    }
}

contract PlonkVerifier_computeChallenges_Test is PlonkVerifierCommonTest {
    /// forge-config: default.fuzz.runs = 5
    /// @dev Test `computeChallenges` matches that of Jellyfish
    function testFuzz_computeChallenges_matches(
        uint64 seed,
        uint256[] memory publicInput,
        bytes memory extraTranscriptInitMsg
    ) external {
        IPlonkVerifier.VerifyingKey memory vk = VkTest.getVk();
        IPlonkVerifier.PlonkProof memory proof = dummyProof(seed);
        publicInput = sanitizeScalarFields(publicInput);
        string[] memory cmds = new string[](9);
        cmds[0] = "cargo";
        cmds[1] = "run";
        cmds[2] = "--bin";
        cmds[3] = "diff-test";
        cmds[4] = "plonk-compute-chal";
        cmds[5] = vm.toString(abi.encode(vk));
        cmds[6] = vm.toString(abi.encode(publicInput));
        cmds[7] = vm.toString(abi.encode(proof));
        cmds[8] = vm.toString(abi.encode(extraTranscriptInitMsg));

        bytes memory result = vm.ffi(cmds);
        (V.Challenges memory chal) = abi.decode(result, (V.Challenges));

        V.Challenges memory c = V._computeChallenges(vk, publicInput, proof, extraTranscriptInitMsg);
        assertEq(chal.alpha, c.alpha);
        assertEq(chal.alpha2, c.alpha2);
        assertEq(chal.alpha3, c.alpha3);
        assertEq(chal.beta, c.beta);
        assertEq(chal.gamma, c.gamma);
        assertEq(chal.zeta, c.zeta);
        assertEq(chal.v, c.v);
        assertEq(chal.u, c.u);
    }
}

contract PlonkVerifier_prepareOpeningProof_Test is Test {
    function testFuzz_prepareOpeningProof_matches() external {
        return;
    }
}

contract PlonkVerifier_computeLinPolyConstantTerm_Test is Test {
    function testFuzz_computeLinPolyConstantTerm_matches() external {
        return;
    }
}

contract PlonkVerifier_preparePolyCommitments_Test is Test {
    function testFuzz_preparePolyCommitments_matches() external {
        return;
    }
}

contract PlonkVerifier_prepareEvaluations_Test is Test {
    function testFuzz_prepareEvaluations_matches() external {
        return;
    }
}

contract PlonkVerifier_batchVerifyOpeningProofs_Test is Test {
    function testFuzz_batchVerifyOpeningProofs_matches() external {
        return;
    }
}

contract PlonkVerifier_linearizationScalarsAndBases_Test is Test {
    function testFuzz_linearizationScalarsAndBases_matches() external {
        return;
    }
}
