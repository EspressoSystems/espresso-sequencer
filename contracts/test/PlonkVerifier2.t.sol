// SPDX-License-Identifier: Unlicensed

/* solhint-disable contract-name-camelcase, func-name-mixedcase, one-contract-per-file */
/* solhint-disable no-inline-assembly  */

// NOTE: For developers and auditors: we mainly test the consistency between the outputs between
// Solidity and Jellyfish library, with the help of fuzzer-generated inputs from Forge Testing.
// Inside the logic of `verify()`, variables values only need to be consistent and valid
// (i.e. valid group or field elements) and don't need to be from a correct proof/public input.
// Only the last step `_verifyOpeningProof` will test *correctness* of these parameters.
// Therefore, we employ more randomly generated dummy inputs for most tests for robustness,
// and only rely on Rust-code to generate correct inputs for the `_verifyOpeningProof`.

pragma solidity ^0.8.0;

// Libraries
import "forge-std/Test.sol";
import { BN254 } from "bn254/BN254.sol";
import { IPlonkVerifier } from "../src/interfaces/IPlonkVerifier.sol";
import { LightClientStateUpdateVKMock as VkTest } from "./mocks/LightClientStateUpdateVKMock.sol";

// Target contract
import { PlonkVerifier2 as V } from "../src/libraries/PlonkVerifier2.sol";

/// @dev Common helpers/utils for PlonkVerifier tests
contract PlonkVerifierCommonTest is Test {
    /// @dev Sanitize a single value to be valid scalar field Bn254::Fr.
    function sanitizeScalarField(uint256 a) public pure returns (uint256) {
        a = bound(a, 0, BN254.R_MOD - 1);
        BN254.validateScalarField(BN254.ScalarField.wrap(a));
        return a;
    }

    /// @dev Sanitize all values in `a` to be valid scalar fields Bn254::Fr.
    /// This is helpful to sanitize fuzzer-generated random `uint[]` values.
    function sanitizeScalarFields(uint256[8] memory a) public pure returns (uint256[8] memory) {
        for (uint256 i = 0; i < a.length; i++) {
            a[i] = sanitizeScalarField(a[i]);
        }
        return a;
    }

    /// @dev Generate a random valid (format-wise) proof from a random seed
    function dummyProof(uint64 seed) public returns (IPlonkVerifier.PlonkProof memory) {
        string[] memory cmds = new string[](3);
        cmds[0] = "diff-test";
        cmds[1] = "dummy-proof";
        cmds[2] = vm.toString(seed);

        bytes memory result = vm.ffi(cmds);
        (IPlonkVerifier.PlonkProof memory proof) = abi.decode(result, (IPlonkVerifier.PlonkProof));
        return proof;
    }
}

contract PlonkVerifier2_verify_Test is PlonkVerifierCommonTest {
    /// @dev Test happy path of `verify`.
    function testFuzz_verify_succeeds(uint64 seed) external {
        vm.pauseGasMetering();
        string[] memory cmds = new string[](3);
        cmds[0] = "diff-test";
        cmds[1] = "plonk-verify";
        cmds[2] = vm.toString(seed);

        bytes memory result = vm.ffi(cmds);
        (
            IPlonkVerifier.VerifyingKey memory verifyingKey,
            uint256[8] memory publicInput,
            IPlonkVerifier.PlonkProof memory proof
        ) = abi.decode(result, (IPlonkVerifier.VerifyingKey, uint256[8], IPlonkVerifier.PlonkProof));

        vm.resumeGasMetering();
        assert(V.verify(verifyingKey, publicInput, proof));
    }

    /// @dev Test when bad verifying key is supplied, the verification should fail
    function testFuzz_badVerifyingKey_fails(uint256 nthPoint) external {
        string[] memory cmds = new string[](2);
        cmds[0] = "diff-test";
        cmds[1] = "plonk-verify";

        bytes memory result = vm.ffi(cmds);
        (
            IPlonkVerifier.VerifyingKey memory verifyingKey,
            uint256[8] memory publicInput,
            IPlonkVerifier.PlonkProof memory proof
        ) = abi.decode(result, (IPlonkVerifier.VerifyingKey, uint256[8], IPlonkVerifier.PlonkProof));

        // there are 18 points in verifying key
        // randomly choose one to mutate
        nthPoint = bound(nthPoint, 0, 17);

        BN254.G1Point memory badPoint;
        assembly {
            // the first point offset is 0x40
            let badPointRef := add(verifyingKey, add(mul(nthPoint, 0x20), 0x40))
            badPoint := mload(badPointRef)
        }

        // modify the point to be invalid
        badPoint = BN254.add(badPoint, BN254.P1());

        assembly {
            let badPointRef := add(verifyingKey, add(mul(nthPoint, 0x20), 0x40))
            mstore(badPointRef, badPoint)
        }

        assert(!V.verify(verifyingKey, publicInput, proof));
    }

    // @dev Test when bad public input is supplied, the verification should fail
    // We know our `gen_circuit_for_test` in `diff_test.rs` has only 8 public inputs
    function testFuzz_badPublicInput_fails(uint256[8] calldata randPublicInput) external {
        uint256[8] memory badPublicInput;
        for (uint256 i = 0; i < 8; i++) {
            badPublicInput[i] = randPublicInput[i];
        }
        badPublicInput = sanitizeScalarFields(badPublicInput);

        string[] memory cmds = new string[](2);
        cmds[0] = "diff-test";
        cmds[1] = "plonk-verify";

        bytes memory result = vm.ffi(cmds);
        (IPlonkVerifier.VerifyingKey memory verifyingKey,, IPlonkVerifier.PlonkProof memory proof) =
            abi.decode(result, (IPlonkVerifier.VerifyingKey, uint256[8], IPlonkVerifier.PlonkProof));

        assert(!V.verify(verifyingKey, badPublicInput, proof));
    }

    /// @dev Test when bad proof is supplied, the verification should fail
    function testFuzz_badProof_fails(uint64 seed) external {
        IPlonkVerifier.PlonkProof memory badProof = dummyProof(seed);

        string[] memory cmds = new string[](2);
        cmds[0] = "diff-test";
        cmds[1] = "plonk-verify";

        bytes memory result = vm.ffi(cmds);
        (IPlonkVerifier.VerifyingKey memory verifyingKey, uint256[8] memory publicInput,) =
            abi.decode(result, (IPlonkVerifier.VerifyingKey, uint256[8], IPlonkVerifier.PlonkProof));

        assert(!V.verify(verifyingKey, publicInput, badProof));
    }
}
