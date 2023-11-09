// SPDX-License-Identifier: Unlicensed

/* solhint-disable contract-name-camelcase, func-name-mixedcase, one-contract-per-file */

// NOTE: For developers and auditors: we mainly test the consistency between the outputs between
// Solidity and Jellyfish library, with the help of fuzzer-generated inputs from Forge Testing.
// Inside the logic of `batchVerify()`, variables values only need to be consistent and valid
// (i.e. valid group or field elements) and don't need to be from a correct proof/public input.
// Only the last step `_batchVerifyOpeningProof` will test *correctness* of these parameters.
// Therefore, we employ more randomly generated dummy inputs for most tests for robustness,
// and only relie on Rust-code to generate correct inputs for the `_batchVerifyOpeningProof`.

pragma solidity ^0.8.0;

// Libraries
import "forge-std/Test.sol";
import { BN254 } from "bn254/BN254.sol";
import { IPlonkVerifier } from "../src/interfaces/IPlonkVerifier.sol";
import { VkTest } from "./stubs/Transfer1In2Out24DepthVk.sol";
import { PolynomialEval as Poly } from "../src/libraries/PolynomialEval.sol";

// Target contract
import { PlonkVerifier as V } from "../src/libraries/PlonkVerifier.sol";

/// @dev Common helpers/utils for PlonkVerifier tests
contract PlonkVerifierCommonTest is Test {
    /// @dev Sanitize a single value to be valid scalar field Bn254::Fr.
    function sanitizeScalarField(uint256 a) public view returns (uint256) {
        a = bound(a, 0, BN254.R_MOD - 1);
        BN254.validateScalarField(a);
        return a;
    }

    /// @dev Sanitize all values in `a` to be valid scalar fields Bn254::Fr.
    /// This is helpful to sanitize fuzzer-generated random `uint[]` values.
    function sanitizeScalarFields(uint256[] memory a) public view returns (uint256[] memory) {
        for (uint256 i = 0; i < a.length; i++) {
            a[i] = bound(a[i], 0, BN254.R_MOD - 1);
            BN254.validateScalarField(a[i]);
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

    /// Thin wrapper to ensure two G1 points are the same
    function assertEqG1Point(BN254.G1Point memory a, BN254.G1Point memory b) public {
        assertEq(a.x, b.x);
        assertEq(a.y, b.y);
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
    /// forge-config: default.fuzz.runs = 30
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

    /// @dev Test happy and unhappy path of `batchVerify`.
    function test_batchVerify_succeeds() external {
        // TODO: change to i<6
        for (uint32 i = 1; i < 2; i++) {
            string[] memory cmds = new string[](6);
            cmds[0] = "cargo";
            cmds[1] = "run";
            cmds[2] = "--bin";
            cmds[3] = "diff-test";
            cmds[4] = "plonk-batch-verify";
            cmds[5] = vm.toString(i);

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

            // happy path
            assert(V.batchVerify(verifyingKeys, publicInputs, proofs, extraTranscriptInitMsgs));

            // unhappy path
            // wrong vk
            IPlonkVerifier.VerifyingKey[] memory badVks = verifyingKeys;
            badVks[0].q1 = BN254.negate(verifyingKeys[0].q1);
            badVks[0].qEcc = BN254.negate(verifyingKeys[0].qEcc);
            assert(!V.batchVerify(badVks, publicInputs, proofs, extraTranscriptInitMsgs));

            // wrong public inputs
            uint256[][] memory badPis = publicInputs;
            badPis[0][0] = 0x1234;
            assert(!V.batchVerify(verifyingKeys, badPis, proofs, extraTranscriptInitMsgs));

            // wrong proofs
            IPlonkVerifier.PlonkProof[] memory badProofs;
            badProofs[0].wireEval0 = 0x12;
            badProofs[0].sigmaEval0 = 0x34;
            assert(!V.batchVerify(verifyingKeys, publicInputs, badProofs, extraTranscriptInitMsgs));

            // wrong extraMsgs
            bytes[] memory badMsgs = extraTranscriptInitMsgs;
            badMsgs[0] = bytes("hi");
            assert(!V.batchVerify(verifyingKeys, publicInputs, proofs, badMsgs));
        }
    }
}

contract PlonkVerifier_validateProof_Test is PlonkVerifierCommonTest {
    /// forge-config: default.fuzz.runs = 5
    /// @dev Test `_validateProof` correct catches invalidly form proof
    function testFuzz_validateProof_succeeds(IPlonkVerifier.PlonkProof memory proof) external {
        // w.o.p. this fuzzer-generated input proof is invalid, either one of the G1Points
        // or one of the scalar fields is invalid.
        vm.expectRevert();
        V._validateProof(proof);

        // happy path
        proof = dummyProof(42); // give any seed
        V._validateProof(proof); // valid proof should pass without any assertion

        // unhappy path
        BN254.G1Point memory invalidPoint = BN254.G1Point(2, 3);
        uint256 invalidScalar = BN254.R_MOD;

        proof.wire0 = invalidPoint;
        vm.expectRevert();
        V._validateProof(proof);
        proof.wire0 = BN254.P1(); // restore to a valid point

        proof.wire1 = invalidPoint;
        vm.expectRevert();
        V._validateProof(proof);
        proof.wire1 = BN254.P1(); // restore to a valid point

        proof.wire2 = invalidPoint;
        vm.expectRevert();
        V._validateProof(proof);
        proof.wire2 = BN254.P1(); // restore to a valid point

        proof.wire3 = invalidPoint;
        vm.expectRevert();
        V._validateProof(proof);
        proof.wire3 = BN254.P1(); // restore to a valid point

        proof.wire4 = invalidPoint;
        vm.expectRevert();
        V._validateProof(proof);
        proof.wire4 = BN254.P1(); // restore to a valid point

        proof.prodPerm = invalidPoint;
        vm.expectRevert();
        V._validateProof(proof);
        proof.prodPerm = BN254.P1(); // restore to a valid point

        proof.split0 = invalidPoint;
        vm.expectRevert();
        V._validateProof(proof);
        proof.split0 = BN254.P1(); // restore to a valid point

        proof.split1 = invalidPoint;
        vm.expectRevert();
        V._validateProof(proof);
        proof.split1 = BN254.P1(); // restore to a valid point

        proof.split2 = invalidPoint;
        vm.expectRevert();
        V._validateProof(proof);
        proof.split2 = BN254.P1(); // restore to a valid point

        proof.split3 = invalidPoint;
        vm.expectRevert();
        V._validateProof(proof);
        proof.split3 = BN254.P1(); // restore to a valid point

        proof.split4 = invalidPoint;
        vm.expectRevert();
        V._validateProof(proof);
        proof.split4 = BN254.P1(); // restore to a valid point

        proof.zeta = invalidPoint;
        vm.expectRevert();
        V._validateProof(proof);
        proof.zeta = BN254.P1(); // restore to a valid point

        proof.wireEval0 = invalidScalar;
        vm.expectRevert();
        V._validateProof(proof);
        proof.wireEval0 = 1; // restore to a valid scalar

        proof.wireEval1 = invalidScalar;
        vm.expectRevert();
        V._validateProof(proof);
        proof.wireEval1 = 1; // restore to a valid scalar

        proof.wireEval2 = invalidScalar;
        vm.expectRevert();
        V._validateProof(proof);
        proof.wireEval2 = 1; // restore to a valid scalar

        proof.wireEval3 = invalidScalar;
        vm.expectRevert();
        V._validateProof(proof);
        proof.wireEval3 = 1; // restore to a valid scalar

        proof.wireEval4 = invalidScalar;
        vm.expectRevert();
        V._validateProof(proof);
        proof.wireEval4 = 1; // restore to a valid scalar

        proof.sigmaEval0 = invalidScalar;
        vm.expectRevert();
        V._validateProof(proof);
        proof.sigmaEval0 = 1; // restore to a valid scalar

        proof.sigmaEval1 = invalidScalar;
        vm.expectRevert();
        V._validateProof(proof);
        proof.sigmaEval1 = 1; // restore to a valid scalar

        proof.sigmaEval2 = invalidScalar;
        vm.expectRevert();
        V._validateProof(proof);
        proof.sigmaEval2 = 1; // restore to a valid scalar

        proof.sigmaEval3 = invalidScalar;
        vm.expectRevert();
        V._validateProof(proof);
        proof.sigmaEval3 = 1; // restore to a valid scalar

        proof.prodPermZetaOmegaEval = invalidScalar;
        vm.expectRevert();
        V._validateProof(proof);
        proof.prodPermZetaOmegaEval = 1; // restore to a valid scalar

        V._validateProof(proof);
    }
}

contract PlonkVerifier_preparePcsInfo_Test is PlonkVerifierCommonTest {
    /// forge-config: default.fuzz.runs = 5
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

        string[] memory cmds = new string[](9);
        cmds[0] = "cargo";
        cmds[1] = "run";
        cmds[2] = "--bin";
        cmds[3] = "diff-test";
        cmds[4] = "plonk-prepare-pcs-info";
        cmds[5] = vm.toString(abi.encode(vk));
        cmds[6] = vm.toString(abi.encode(publicInput));
        cmds[7] = vm.toString(abi.encode(proof));
        cmds[8] = vm.toString(abi.encode(extraTranscriptInitMsg));

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
        assertEqG1Point(BN254.multiScalarMul(info.commBases, info.commScalars), scalarsAndBasesProd);
        assertEqG1Point(info.openingProof, openingProof);
        assertEqG1Point(info.shiftedOpeningProof, shiftedOpeningProof);
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

contract PlonkVerifier_prepareEvaluations_Test is PlonkVerifierCommonTest {
    /// forge-config: default.fuzz.runs = 5
    /// @dev Test if combinng the polynomial evaluations into a single evaluation is done correctly
    /// is done correctly
    function testFuzz_prepareEvaluations_matches(
        uint64 seed,
        uint256 linPolyConstant,
        uint256[30] memory scalars
    ) external {
        IPlonkVerifier.PlonkProof memory proof = dummyProof(seed);
        linPolyConstant = sanitizeScalarField(linPolyConstant);
        uint256[] memory commScalars = sanitizeScalarFields(copyCommScalars(scalars));

        string[] memory cmds = new string[](8);
        cmds[0] = "cargo";
        cmds[1] = "run";
        cmds[2] = "--bin";
        cmds[3] = "diff-test";
        cmds[4] = "plonk-prepare-eval";
        cmds[5] = vm.toString(abi.encode(proof));
        cmds[6] = vm.toString(bytes32(linPolyConstant));
        cmds[7] = vm.toString(abi.encode(commScalars));

        bytes memory result = vm.ffi(cmds);
        (uint256 eval) = abi.decode(result, (uint256));

        assertEq(eval, V._prepareEvaluations(linPolyConstant, proof, commScalars));
    }
}

// NOTE: it's troublesome to convert `ScalarsAndBases` field of a proper `PcsInfo` from Jellyfish to
// Solidity due to the different data structure (vector v.s. map). Thus, we skip diff-test for
// `batchVerifyOpeningProofs()` for now, and only test the outer `batchVerify()` directly
