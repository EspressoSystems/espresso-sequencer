// SPDX-License-Identifier: Unlicensed

/* solhint-disable contract-name-camelcase, func-name-mixedcase, one-contract-per-file */
/* solhint-disable no-inline-assembly  */

// NOTE: For developers and auditors: we mainly test the consistency between the outputs between
// Solidity and Jellyfish library, with the help of fuzzer-generated inputs from Forge Testing.
// Inside the logic of `batchVerify()`, variables values only need to be consistent and valid
// (i.e. valid group or field elements) and don't need to be from a correct proof/public input.
// Only the last step `_batchVerifyOpeningProof` will test *correctness* of these parameters.
// Therefore, we employ more randomly generated dummy inputs for most tests for robustness,
// and only rely on Rust-code to generate correct inputs for the `_batchVerifyOpeningProof`.

pragma solidity ^0.8.0;

// Libraries
import "forge-std/Test.sol";
import { BN254 } from "bn254/BN254.sol";
import { IPlonkVerifier } from "../src/interfaces/IPlonkVerifier.sol";
import { LightClientStateUpdateVKTest as VkTest } from "./mocks/LightClientStateUpdateVKTest.sol";
import { PolynomialEval as Poly } from "../src/libraries/PolynomialEval.sol";

// Target contract
import { PlonkVerifier as V } from "../src/libraries/PlonkVerifier.sol";

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
    function sanitizeScalarFields(uint256[] memory a) public pure returns (uint256[] memory) {
        for (uint256 i = 0; i < a.length; i++) {
            a[i] = sanitizeScalarField(a[i]);
        }
        return a;
    }

    /// @dev Sanitize dummy verifyingKey such that it matches with the length of publicInput,
    /// This is only used for fuzz-generated-dummy-valued tests.
    function sanitizeVk(IPlonkVerifier.VerifyingKey memory vk, uint256 piLength)
        public
        pure
        returns (IPlonkVerifier.VerifyingKey memory)
    {
        vk.numInputs = piLength;
        return vk;
    }

    /// @dev copy a fixed array into a dynamic array, mostly used for converting fuzzer generated
    /// array into another accepted by most APIs.
    function copyCommScalars(uint256[30] memory a) public pure returns (uint256[] memory) {
        uint256[] memory b = new uint256[](a.length);
        for (uint256 i = 0; i < a.length; i++) {
            b[i] = a[i];
        }
        return b;
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

    /// @dev helper function to generate some dummy but format-valid arguments for
    /// `prepareOpeningProof` step. The verifyingKey should be fixed/loaded from library,
    /// proof should be generated via `dummyProof()`, other inputs are from fuzzers.
    function dummyArgsForOpeningProof(
        uint64 seed,
        uint256[] memory publicInput,
        bytes memory extraTranscriptInitMsg
    )
        public
        returns (
            IPlonkVerifier.VerifyingKey memory,
            IPlonkVerifier.PlonkProof memory,
            V.Challenges memory,
            Poly.EvalData memory
        )
    {
        IPlonkVerifier.VerifyingKey memory vk = sanitizeVk(VkTest.getVk(), publicInput.length);
        IPlonkVerifier.PlonkProof memory proof = dummyProof(seed);
        V.Challenges memory chal =
            V._computeChallenges(vk, publicInput, proof, extraTranscriptInitMsg);

        Poly.EvalDomain memory domain = Poly.newEvalDomain(vk.domainSize);
        // pre-compute evaluation data
        Poly.EvalData memory evalData = Poly.evalDataGen(domain, chal.zeta, publicInput);

        return (vk, proof, chal, evalData);
    }
}

contract PlonkVerifier_constants_Test is Test {
    /// @dev Test constants declared matches that from Jellyfish
    function test_correctConstants() external {
        string[] memory cmds = new string[](3);
        cmds[0] = "diff-test";
        cmds[1] = "plonk-constants";

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

// Mostly identical with `PlonkVerifier_batchVerify_Test`
contract PlonkVerifier_verify_Test is PlonkVerifierCommonTest {
    /// @dev Test happy path of `verify`.
    function test_verify_succeeds() external {
        vm.pauseGasMetering();
        string[] memory cmds = new string[](3);
        cmds[0] = "diff-test";
        cmds[1] = "plonk-batch-verify";
        cmds[2] = vm.toString(uint32(1));

        bytes memory result = vm.ffi(cmds);
        (
            IPlonkVerifier.VerifyingKey[] memory verifyingKeys,
            uint256[][] memory publicInputs,
            IPlonkVerifier.PlonkProof[] memory proofs,
            bytes[] memory extraTranscriptInitMsgs
        ) = abi.decode(
            result,
            (IPlonkVerifier.VerifyingKey[], uint256[][], IPlonkVerifier.PlonkProof[], bytes[])
        );

        vm.resumeGasMetering();
        assert(V.verify(verifyingKeys[0], publicInputs[0], proofs[0], extraTranscriptInitMsgs[0]));
    }

    /// @dev Test when bad verifying key is supplied, the verification should fail
    function testFuzz_badVerifyingKey_fails(uint256 nthPoint) external {
        string[] memory cmds = new string[](3);
        cmds[0] = "diff-test";
        cmds[1] = "plonk-batch-verify";
        cmds[2] = vm.toString(uint32(1));

        bytes memory result = vm.ffi(cmds);
        (
            IPlonkVerifier.VerifyingKey[] memory verifyingKeys,
            uint256[][] memory publicInputs,
            IPlonkVerifier.PlonkProof[] memory proofs,
            bytes[] memory extraTranscriptInitMsgs
        ) = abi.decode(
            result,
            (IPlonkVerifier.VerifyingKey[], uint256[][], IPlonkVerifier.PlonkProof[], bytes[])
        );

        // there are 18 points in verifying key
        // randomly choose one to mutate
        nthPoint = bound(nthPoint, 0, 17);

        BN254.G1Point memory badPoint;
        assembly {
            // the first 32 bytes is array length
            let firstVkRef := add(verifyingKeys, 0x20)
            // the first point offset is 0x40
            let badPointRef := add(mload(firstVkRef), add(mul(nthPoint, 0x20), 0x40))
            badPoint := mload(badPointRef)
        }

        // modify the point to be invalid
        badPoint = BN254.add(badPoint, BN254.P1());

        assembly {
            let firstVkRef := add(verifyingKeys, 0x20)
            let badPointRef := add(mload(firstVkRef), add(mul(nthPoint, 0x20), 0x40))
            mstore(badPointRef, badPoint)
        }

        assert(!V.verify(verifyingKeys[0], publicInputs[0], proofs[0], extraTranscriptInitMsgs[0]));
    }

    /// @dev Test when bad public input is supplied, the verification should fail
    /// We know our `gen_circuit_for_test` in `diff_test.rs` has only 3 public inputs
    function testFuzz_badPublicInput_fails(uint256[3] calldata randPublicInput) external {
        uint256[] memory badPublicInput = new uint256[](3);
        badPublicInput[0] = randPublicInput[0];
        badPublicInput[1] = randPublicInput[1];
        badPublicInput[2] = randPublicInput[2];
        badPublicInput = sanitizeScalarFields(badPublicInput);

        string[] memory cmds = new string[](3);
        cmds[0] = "diff-test";
        cmds[1] = "plonk-batch-verify";
        cmds[2] = vm.toString(uint32(1));

        bytes memory result = vm.ffi(cmds);
        (
            IPlonkVerifier.VerifyingKey[] memory verifyingKeys,
            ,
            IPlonkVerifier.PlonkProof[] memory proofs,
            bytes[] memory extraTranscriptInitMsgs
        ) = abi.decode(
            result,
            (IPlonkVerifier.VerifyingKey[], uint256[][], IPlonkVerifier.PlonkProof[], bytes[])
        );

        assert(!V.verify(verifyingKeys[0], badPublicInput, proofs[0], extraTranscriptInitMsgs[0]));
    }

    /// @dev Test when bad proof is supplied, the verification should fail
    function testFuzz_badProof_fails(uint64 seed) external {
        IPlonkVerifier.PlonkProof memory badProof = dummyProof(seed);

        string[] memory cmds = new string[](3);
        cmds[0] = "diff-test";
        cmds[1] = "plonk-batch-verify";
        cmds[2] = vm.toString(uint32(1));

        bytes memory result = vm.ffi(cmds);
        (
            IPlonkVerifier.VerifyingKey[] memory verifyingKeys,
            uint256[][] memory publicInputs,
            ,
            bytes[] memory extraTranscriptInitMsgs
        ) = abi.decode(
            result,
            (IPlonkVerifier.VerifyingKey[], uint256[][], IPlonkVerifier.PlonkProof[], bytes[])
        );

        assert(!V.verify(verifyingKeys[0], publicInputs[0], badProof, extraTranscriptInitMsgs[0]));
    }

    /// @dev Test when bad extraTranscriptInitMsg is supplied, the verification should fail
    function testFuzz_badExtraTranscriptInitMsg_fails(bytes calldata badMsg) external {
        string[] memory cmds = new string[](3);
        cmds[0] = "diff-test";
        cmds[1] = "plonk-batch-verify";
        cmds[2] = vm.toString(uint32(1));

        bytes memory result = vm.ffi(cmds);
        (
            IPlonkVerifier.VerifyingKey[] memory verifyingKeys,
            uint256[][] memory publicInputs,
            IPlonkVerifier.PlonkProof[] memory proofs,
        ) = abi.decode(
            result,
            (IPlonkVerifier.VerifyingKey[], uint256[][], IPlonkVerifier.PlonkProof[], bytes[])
        );

        assert(!V.verify(verifyingKeys[0], publicInputs[0], proofs[0], badMsg));
    }
}

contract PlonkVerifier_batchVerify_Test is PlonkVerifierCommonTest {
    /// @dev Test if some of the user inputs are invalid
    function testFuzz_revertWhenInvalidArgs(
        IPlonkVerifier.VerifyingKey[] memory verifyingKeys,
        uint256[][] memory publicInputs,
        IPlonkVerifier.PlonkProof[] memory proofs,
        bytes[] memory extraTranscriptInitMsgs
    ) external {
        vm.assume(
            verifyingKeys.length != proofs.length || publicInputs.length != proofs.length
                || extraTranscriptInitMsgs.length != proofs.length || proofs.length == 0
        );
        vm.expectRevert(V.InvalidPlonkArgs.selector);
        V.batchVerify(verifyingKeys, publicInputs, proofs, extraTranscriptInitMsgs);
    }

    /// @dev Test happy path of `batchVerify`.
    function test_batchVerify_succeeds() external {
        for (uint32 i = 1; i < 6; i++) {
            string[] memory cmds = new string[](3);
            cmds[0] = "diff-test";
            cmds[1] = "plonk-batch-verify";
            cmds[2] = vm.toString(i);

            bytes memory result = vm.ffi(cmds);
            (
                IPlonkVerifier.VerifyingKey[] memory verifyingKeys,
                uint256[][] memory publicInputs,
                IPlonkVerifier.PlonkProof[] memory proofs,
                bytes[] memory extraTranscriptInitMsgs
            ) = abi.decode(
                result,
                (IPlonkVerifier.VerifyingKey[], uint256[][], IPlonkVerifier.PlonkProof[], bytes[])
            );

            assert(V.batchVerify(verifyingKeys, publicInputs, proofs, extraTranscriptInitMsgs));
        }
    }

    /// @dev Test when bad verifying key are supplied, the verification should fail
    function testFuzz_badVerifyingKey_fails(uint256 nthPoint) external {
        string[] memory cmds = new string[](3);
        cmds[0] = "diff-test";
        cmds[1] = "plonk-batch-verify";
        cmds[2] = vm.toString(uint32(1));

        bytes memory result = vm.ffi(cmds);
        (
            IPlonkVerifier.VerifyingKey[] memory verifyingKeys,
            uint256[][] memory publicInputs,
            IPlonkVerifier.PlonkProof[] memory proofs,
            bytes[] memory extraTranscriptInitMsgs
        ) = abi.decode(
            result,
            (IPlonkVerifier.VerifyingKey[], uint256[][], IPlonkVerifier.PlonkProof[], bytes[])
        );

        // there are 18 points in verifying key
        // randomly choose one to mutate
        nthPoint = bound(nthPoint, 0, 17);

        BN254.G1Point memory badPoint;
        assembly {
            // the first 32 bytes is array length
            let firstVkRef := add(verifyingKeys, 0x20)
            // the first point offset is 0x40
            let badPointRef := add(mload(firstVkRef), add(mul(nthPoint, 0x20), 0x40))
            badPoint := mload(badPointRef)
        }

        // modify the point to be invalid
        badPoint = BN254.add(badPoint, BN254.P1());

        assembly {
            let firstVkRef := add(verifyingKeys, 0x20)
            let badPointRef := add(mload(firstVkRef), add(mul(nthPoint, 0x20), 0x40))
            mstore(badPointRef, badPoint)
        }

        assert(!V.batchVerify(verifyingKeys, publicInputs, proofs, extraTranscriptInitMsgs));
    }

    /// @dev Test when bad public inputs are supplied, the verification should fail
    /// We know our `gen_circuit_for_test` in `diff_test.rs` has only 3 public inputs
    function testFuzz_badPublicInputs_fails(uint256[3] calldata randPublicInput) external {
        uint256[] memory badPublicInput = new uint256[](3);
        badPublicInput[0] = randPublicInput[0];
        badPublicInput[1] = randPublicInput[1];
        badPublicInput[2] = randPublicInput[2];
        badPublicInput = sanitizeScalarFields(badPublicInput);

        string[] memory cmds = new string[](3);
        cmds[0] = "diff-test";
        cmds[1] = "plonk-batch-verify";
        cmds[2] = vm.toString(uint32(1));

        bytes memory result = vm.ffi(cmds);
        (
            IPlonkVerifier.VerifyingKey[] memory verifyingKeys,
            uint256[][] memory publicInputs,
            IPlonkVerifier.PlonkProof[] memory proofs,
            bytes[] memory extraTranscriptInitMsgs
        ) = abi.decode(
            result,
            (IPlonkVerifier.VerifyingKey[], uint256[][], IPlonkVerifier.PlonkProof[], bytes[])
        );

        publicInputs[0] = badPublicInput;
        assert(!V.batchVerify(verifyingKeys, publicInputs, proofs, extraTranscriptInitMsgs));
    }

    /// @dev Test when bad proofs are supplied, the verification should fail
    function testFuzz_badProofs_fails(uint64 seed) external {
        IPlonkVerifier.PlonkProof memory badProof = dummyProof(seed);

        string[] memory cmds = new string[](3);
        cmds[0] = "diff-test";
        cmds[1] = "plonk-batch-verify";
        cmds[2] = vm.toString(uint32(1));

        bytes memory result = vm.ffi(cmds);
        (
            IPlonkVerifier.VerifyingKey[] memory verifyingKeys,
            uint256[][] memory publicInputs,
            IPlonkVerifier.PlonkProof[] memory proofs,
            bytes[] memory extraTranscriptInitMsgs
        ) = abi.decode(
            result,
            (IPlonkVerifier.VerifyingKey[], uint256[][], IPlonkVerifier.PlonkProof[], bytes[])
        );

        proofs[0] = badProof;
        assert(!V.batchVerify(verifyingKeys, publicInputs, proofs, extraTranscriptInitMsgs));
    }

    /// @dev Test when bad extraTranscriptInitMsgs are supplied, the verification should fail
    function testFuzz_badExtraTranscriptInitMsgs_fails(bytes calldata badMsg) external {
        string[] memory cmds = new string[](3);
        cmds[0] = "diff-test";
        cmds[1] = "plonk-batch-verify";
        cmds[2] = vm.toString(uint32(1));

        bytes memory result = vm.ffi(cmds);
        (
            IPlonkVerifier.VerifyingKey[] memory verifyingKeys,
            uint256[][] memory publicInputs,
            IPlonkVerifier.PlonkProof[] memory proofs,
            bytes[] memory extraTranscriptInitMsgs
        ) = abi.decode(
            result,
            (IPlonkVerifier.VerifyingKey[], uint256[][], IPlonkVerifier.PlonkProof[], bytes[])
        );

        extraTranscriptInitMsgs[0] = badMsg;
        assert(!V.batchVerify(verifyingKeys, publicInputs, proofs, extraTranscriptInitMsgs));
    }
}

contract PlonkVerifier_validateProof_Test is PlonkVerifierCommonTest {
    /// @dev Test that a valid proof shouldn't revert
    function test_validProof_succeeds() external {
        // a valid proof
        IPlonkVerifier.PlonkProof memory proof = dummyProof(42);

        V._validateProof(proof);
    }

    /// @dev Randomly pick a coordinate of a point among points in a proof
    /// mutate it to another value so that the point is no longer valid,
    /// test if our check will revert.
    function testFuzz_RevertIfProofContainsInvalidGroup(uint256 nthPoint, bool testX) external {
        // a valid proof
        IPlonkVerifier.PlonkProof memory proof = dummyProof(42);

        // we are testing the `nthPoint` in the `proof`,
        // only mutating a single field element (either x or y coordinate)
        // There are 13 points in total.
        nthPoint = bound(nthPoint, 0, 12);

        assembly {
            switch testX
            case true {
                // mutate the x coordinate
                mstore(mload(add(proof, mul(0x20, nthPoint))), 0x1234)
            }
            default {
                // else, mutate y coordinate
                mstore(add(mload(add(proof, mul(0x20, nthPoint))), 0x20), 0x1234)
            }
        }

        vm.expectRevert();
        V._validateProof(proof);
    }

    /// @dev Randomly pick field in a proof mutate it to invalid value
    /// test if our check will revert.
    function testFuzz_RevertIfProofContainsInvalidField(uint256 nthField) external {
        // a valid proof
        IPlonkVerifier.PlonkProof memory proof = dummyProof(42);
        uint256 invalidField = BN254.R_MOD;

        // we are testing the `nthField` in the `proof`,
        // There are 10 field elements in total (with 13 points in front)
        nthField = bound(nthField, 0, 9);

        assembly {
            let start := add(proof, mul(0x20, 13))
            mstore(add(start, mul(nthField, 0x20)), invalidField)
        }

        vm.expectRevert();
        V._validateProof(proof);
    }
}

contract PlonkVerifier_preparePcsInfo_Test is PlonkVerifierCommonTest {
    /// @dev Test `preparePcsInfo` matches that of Jellyfish
    function testFuzz_preparePcsInfo_matches(
        uint64 seed,
        uint256[] memory publicInput,
        bytes memory extraTranscriptInitMsg
    ) external {
        publicInput = sanitizeScalarFields(publicInput);
        IPlonkVerifier.VerifyingKey memory vk = sanitizeVk(VkTest.getVk(), publicInput.length);
        IPlonkVerifier.PlonkProof memory proof = dummyProof(seed);

        V.PcsInfo memory info = V._preparePcsInfo(vk, publicInput, proof, extraTranscriptInitMsg);

        string[] memory cmds = new string[](6);
        cmds[0] = "diff-test";
        cmds[1] = "plonk-prepare-pcs-info";
        cmds[2] = vm.toString(abi.encode(vk));
        cmds[3] = vm.toString(abi.encode(publicInput));
        cmds[4] = vm.toString(abi.encode(proof));
        cmds[5] = vm.toString(abi.encode(extraTranscriptInitMsg));

        bytes memory result = vm.ffi(cmds);
        (
            uint256 u,
            uint256 evalPoint,
            uint256 nextEvalPoint,
            uint256 eval,
            BN254.G1Point memory scalarsAndBasesProd,
            BN254.G1Point memory openingProof,
            BN254.G1Point memory shiftedOpeningProof
        ) = abi.decode(
            result,
            (uint256, uint256, uint256, uint256, BN254.G1Point, BN254.G1Point, BN254.G1Point)
        );

        assertEq(info.u, u);
        assertEq(info.evalPoint, evalPoint);
        assertEq(info.nextEvalPoint, nextEvalPoint);
        assertEq(info.eval, eval);
        // NOTE: since we cannot directly compare `struct ScalarsAndBases`, we compare their MSM
        uint256 l = info.commScalars.length;
        BN254.ScalarField[] memory infoCommScalars = new BN254.ScalarField[](l);
        for (uint256 i = 0; i < l; i++) {
            infoCommScalars[i] = BN254.ScalarField.wrap(info.commScalars[i]);
        }
        assertEq(
            abi.encode(BN254.multiScalarMul(info.commBases, infoCommScalars)),
            abi.encode(scalarsAndBasesProd)
        );
        assertEq(abi.encode(info.openingProof), abi.encode(openingProof));
        assertEq(abi.encode(info.shiftedOpeningProof), abi.encode(shiftedOpeningProof));
    }
}

contract PlonkVerifier_computeChallenges_Test is PlonkVerifierCommonTest {
    /// @dev Test `computeChallenges` matches that of Jellyfish
    function testFuzz_computeChallenges_matches(
        uint64 seed,
        uint256[] memory publicInput,
        bytes memory extraTranscriptInitMsg
    ) external {
        IPlonkVerifier.VerifyingKey memory vk = VkTest.getVk();
        IPlonkVerifier.PlonkProof memory proof = dummyProof(seed);
        publicInput = sanitizeScalarFields(publicInput);

        string[] memory cmds = new string[](6);
        cmds[0] = "diff-test";
        cmds[1] = "plonk-compute-chal";
        cmds[2] = vm.toString(abi.encode(vk));
        cmds[3] = vm.toString(abi.encode(publicInput));
        cmds[4] = vm.toString(abi.encode(proof));
        cmds[5] = vm.toString(abi.encode(extraTranscriptInitMsg));

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

contract PlonkVerifier_prepareEvaluations_Test is PlonkVerifierCommonTest {
    /// @dev Test if combining the polynomial evaluations into a single evaluation is done correctly
    /// is done correctly
    function testFuzz_prepareEvaluations_matches(
        uint64 seed,
        uint256 linPolyConstant,
        uint256[30] memory scalars
    ) external {
        IPlonkVerifier.PlonkProof memory proof = dummyProof(seed);
        linPolyConstant = sanitizeScalarField(linPolyConstant);
        uint256[] memory commScalars = sanitizeScalarFields(copyCommScalars(scalars));

        string[] memory cmds = new string[](5);
        cmds[0] = "diff-test";
        cmds[1] = "plonk-prepare-eval";
        cmds[2] = vm.toString(abi.encode(proof));
        cmds[3] = vm.toString(bytes32(linPolyConstant));
        cmds[4] = vm.toString(abi.encode(commScalars));

        bytes memory result = vm.ffi(cmds);
        (uint256 eval) = abi.decode(result, (uint256));

        assertEq(eval, V._prepareEvaluations(linPolyConstant, proof, commScalars));
    }
}

// NOTE: it's troublesome to convert `ScalarsAndBases` field of a proper `PcsInfo` from Jellyfish to
// Solidity due to the different data structure (vector v.s. map). Thus, we skip diff-test for
// `batchVerifyOpeningProofs()` for now, and only test the outer `batchVerify()` directly
