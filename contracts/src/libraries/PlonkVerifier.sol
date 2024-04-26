// SPDX-License-Identifier: Unlicensed

pragma solidity ^0.8.0;

import { BN254, Utils } from "bn254/BN254.sol";
import { PolynomialEval as Poly } from "./PolynomialEval.sol";
import { IPlonkVerifier } from "../interfaces/IPlonkVerifier.sol";
import { Transcript } from "./Transcript.sol";

/* solhint-disable no-inline-assembly */

/// @dev The TurboPlonk formula is:
/// qo * wo = pub_input + q_c +
///           q_mul0 * w0 * w1 + q_mul1 * w2 * w3 +
///           q_lc0 * w0 + q_lc1 * w1 + q_lc2 * w2 + q_lc3 * w3 +
///           q_hash0 * w0 + q_hash1 * w1 + q_hash2 * w2 + q_hash3 * w3 +
///           q_ecc * w0 * w1 * w2 * w3 * wo
library PlonkVerifier {
    /// Plonk: invalid inputs, either mismatching lengths among input arguments
    /// or empty input.
    error InvalidPlonkArgs();
    /// Plonk: wrong verification key used.
    error WrongPlonkVK();

    using Transcript for Transcript.TranscriptData;

    // _COSET_K0 = 1, has no effect during multiplication, thus avoid declaring it here.
    uint256 internal constant COSET_K1 =
        0x2f8dd1f1a7583c42c4e12a44e110404c73ca6c94813f85835da4fb7bb1301d4a;
    uint256 internal constant COSET_K2 =
        0x1ee678a0470a75a6eaa8fe837060498ba828a3703b311d0f77f010424afeb025;
    uint256 internal constant COSET_K3 =
        0x2042a587a90c187b0a087c03e29c968b950b1db26d5c82d666905a6895790c0a;
    uint256 internal constant COSET_K4 =
        0x2e2b91456103698adf57b799969dea1c8f739da5d8d40dd3eb9222db7c81e881;

    // Parsed from Aztec's Ignition CRS,
    // `beta_h` \in G2 where \beta is the trapdoor, h is G2 generator `BN254.P2()`
    // See parsing code: https://github.com/alxiong/crs
    // @dev since library cannot have constant value of custom type, we break it
    // into individual field values.
    uint256 internal constant BETA_H_X0 =
        0x260e01b251f6f1c7e7ff4e580791dee8ea51d87a358e038b4efe30fac09383c1;
    uint256 internal constant BETA_H_X1 =
        0x0118c4d5b837bcc2bc89b5b398b5974e9f5944073b32078b7e231fec938883b0;
    uint256 internal constant BETA_H_Y0 =
        0x04fc6369f7110fe3d25156c1bb9a72859cf2a04641f99ba4ee413c80da6a5fe4;
    uint256 internal constant BETA_H_Y1 =
        0x22febda3c0c0632a56475b4214e5615e11e6dd3f96e6cea2854a87d4dacc5e55;

    /// The number of wire types of the circuit, TurboPlonk has 5.
    uint256 internal constant NUM_WIRE_TYPES = 5;

    /// @dev polynomial commitment evaluation info.
    struct PcsInfo {
        // a random combiner that was used to combine evaluations at point
        uint256 u; // 0x00
        // the point to be evaluated at
        uint256 evalPoint; // 0x20
        // the shifted point to be evaluated at
        uint256 nextEvalPoint; // 0x40
        // the polynomial evaluation value
        uint256 eval; // 0x60
        // scalars of poly comm for MSM
        uint256[] commScalars; // 0x80
        // bases of poly comm for MSM
        BN254.G1Point[] commBases; // 0xa0
        // proof of evaluations at point `eval_point`
        BN254.G1Point openingProof; // 0xc0
        // proof of evaluations at point `next_eval_point`
        BN254.G1Point shiftedOpeningProof; // 0xe0
    }

    /// @dev Plonk IOP verifier challenges.
    struct Challenges {
        uint256 alpha; // 0x00
        uint256 alpha2; // 0x20
        uint256 alpha3; // 0x40
        uint256 beta; // 0x60
        uint256 gamma; // 0x80
        uint256 zeta; // 0xA0
        uint256 v; // 0xC0
        uint256 u; // 0xE0
    }

    /// @dev Verify a single TurboPlonk proofs.
    /// @param verifyingKey The Plonk verification key
    /// @param publicInput The public input fields
    /// @param proof The TurboPlonk proof
    /// @return _ A boolean indicating successful verification, false otherwise
    function verify(
        IPlonkVerifier.VerifyingKey memory verifyingKey,
        uint256[] memory publicInput,
        IPlonkVerifier.PlonkProof memory proof
    ) internal view returns (bool) {
        _validateProof(proof);

        // This *if* statement is needed in order for some tests to pass.
        if (publicInput.length == 8) {
            BN254.validateScalarField(BN254.ScalarField.wrap(publicInput[0]));
            BN254.validateScalarField(BN254.ScalarField.wrap(publicInput[1]));
            BN254.validateScalarField(BN254.ScalarField.wrap(publicInput[2]));
            BN254.validateScalarField(BN254.ScalarField.wrap(publicInput[3]));
            BN254.validateScalarField(BN254.ScalarField.wrap(publicInput[4]));
            BN254.validateScalarField(BN254.ScalarField.wrap(publicInput[5]));
            BN254.validateScalarField(BN254.ScalarField.wrap(publicInput[6]));
            BN254.validateScalarField(BN254.ScalarField.wrap(publicInput[7]));
        } else {
            for (uint256 i = 0; i < publicInput.length; i++) {
                BN254.validateScalarField(BN254.ScalarField.wrap(publicInput[i]));
            }
        }

        PcsInfo memory pcsInfo = _preparePcsInfo(verifyingKey, publicInput, proof);
        return _verifyOpeningProofs(pcsInfo);
    }

    /// @dev Validate all group points and scalar fields. Revert if
    /// any are invalid.
    /// @param proof A Plonk proof
    function _validateProof(IPlonkVerifier.PlonkProof memory proof) internal pure {
        BN254.validateG1Point(proof.wire0);
        BN254.validateG1Point(proof.wire1);
        BN254.validateG1Point(proof.wire2);
        BN254.validateG1Point(proof.wire3);
        BN254.validateG1Point(proof.wire4);
        BN254.validateG1Point(proof.prodPerm);
        BN254.validateG1Point(proof.split0);
        BN254.validateG1Point(proof.split1);
        BN254.validateG1Point(proof.split2);
        BN254.validateG1Point(proof.split3);
        BN254.validateG1Point(proof.split4);
        BN254.validateG1Point(proof.zeta);
        BN254.validateG1Point(proof.zetaOmega);
        BN254.validateScalarField(proof.wireEval0);
        BN254.validateScalarField(proof.wireEval1);
        BN254.validateScalarField(proof.wireEval2);
        BN254.validateScalarField(proof.wireEval3);
        BN254.validateScalarField(proof.wireEval4);
        BN254.validateScalarField(proof.sigmaEval0);
        BN254.validateScalarField(proof.sigmaEval1);
        BN254.validateScalarField(proof.sigmaEval2);
        BN254.validateScalarField(proof.sigmaEval3);
        BN254.validateScalarField(proof.prodPermZetaOmegaEval);
    }

    function _preparePcsInfo(
        IPlonkVerifier.VerifyingKey memory verifyingKey,
        uint256[] memory publicInput,
        IPlonkVerifier.PlonkProof memory proof
    ) internal view returns (PcsInfo memory res) {
        if (publicInput.length != verifyingKey.numInputs) revert WrongPlonkVK();

        Challenges memory chal = _computeChallenges(verifyingKey, publicInput, proof);

        Poly.EvalDomain memory domain = Poly.newEvalDomain(verifyingKey.domainSize);
        // pre-compute evaluation data
        Poly.EvalData memory evalData = Poly.evalDataGen(domain, chal.zeta, publicInput);

        // compute opening proof in poly comm.
        uint256[] memory commScalars = new uint256[](30);
        BN254.G1Point[] memory commBases = new BN254.G1Point[](30);

        uint256 eval =
            _prepareOpeningProof(verifyingKey, evalData, proof, chal, commScalars, commBases);

        uint256 zeta = chal.zeta;
        uint256 omega = domain.groupGen;
        uint256 p = BN254.R_MOD;
        uint256 zetaOmega;
        assembly {
            zetaOmega := mulmod(zeta, omega, p)
        }

        res = PcsInfo(
            chal.u, zeta, zetaOmega, eval, commScalars, commBases, proof.zeta, proof.zetaOmega
        );
    }

    function _computeChallenges(
        IPlonkVerifier.VerifyingKey memory verifyingKey,
        uint256[] memory publicInput,
        IPlonkVerifier.PlonkProof memory proof
    ) internal pure returns (Challenges memory res) {
        Transcript.TranscriptData memory transcript;
        uint256 p = BN254.R_MOD;

        transcript.appendVkAndPubInput(verifyingKey, publicInput);

        transcript.transcript = abi.encodePacked(
            transcript.transcript,
            BN254.g1Serialize(proof.wire0),
            BN254.g1Serialize(proof.wire1),
            BN254.g1Serialize(proof.wire2),
            BN254.g1Serialize(proof.wire3),
            BN254.g1Serialize(proof.wire4)
        );

        // have to compute tau, but not really used anywhere
        // slither-disable-next-line unused-return
        transcript.getAndAppendChallenge();
        res.beta = transcript.getAndAppendChallenge();
        res.gamma = transcript.getAndAppendChallenge();

        transcript.transcript =
            abi.encodePacked(transcript.transcript, BN254.g1Serialize(proof.prodPerm));

        res.alpha = transcript.getAndAppendChallenge();

        transcript.transcript = abi.encodePacked(
            transcript.transcript,
            BN254.g1Serialize(proof.split0),
            BN254.g1Serialize(proof.split1),
            BN254.g1Serialize(proof.split2),
            BN254.g1Serialize(proof.split3),
            BN254.g1Serialize(proof.split4)
        );

        res.zeta = transcript.getAndAppendChallenge();

        // Append proof evaluations
        transcript.transcript = abi.encodePacked(
            transcript.transcript,
            Utils.reverseEndianness(BN254.ScalarField.unwrap(proof.wireEval0)),
            Utils.reverseEndianness(BN254.ScalarField.unwrap(proof.wireEval1)),
            Utils.reverseEndianness(BN254.ScalarField.unwrap(proof.wireEval2)),
            Utils.reverseEndianness(BN254.ScalarField.unwrap(proof.wireEval3)),
            Utils.reverseEndianness(BN254.ScalarField.unwrap(proof.wireEval4))
        );

        transcript.transcript = abi.encodePacked(
            transcript.transcript,
            Utils.reverseEndianness(BN254.ScalarField.unwrap(proof.sigmaEval0)),
            Utils.reverseEndianness(BN254.ScalarField.unwrap(proof.sigmaEval1)),
            Utils.reverseEndianness(BN254.ScalarField.unwrap(proof.sigmaEval2)),
            Utils.reverseEndianness(BN254.ScalarField.unwrap(proof.sigmaEval3)),
            Utils.reverseEndianness(BN254.ScalarField.unwrap(proof.prodPermZetaOmegaEval))
        );

        res.v = transcript.getAndAppendChallenge();

        transcript.transcript = abi.encodePacked(
            transcript.transcript, BN254.g1Serialize(proof.zeta), BN254.g1Serialize(proof.zetaOmega)
        );

        res.u = transcript.getAndAppendChallenge();

        assembly {
            let alpha := mload(res)
            let alpha2 := mulmod(alpha, alpha, p)
            let alpha3 := mulmod(alpha2, alpha, p)
            mstore(add(res, 0x20), alpha2)
            mstore(add(res, 0x40), alpha3)
        }
    }

    /// @dev Compute components in [E]1 and [F]1 used for PolyComm opening verification
    /// equivalent of JF's
    /// https://github.com/EspressoSystems/jellyfish/blob/main/plonk/src/proof_system/verifier.rs#L154-L170
    /// caller allocates the memory fr commScalars and commBases
    /// requires Arrays of size 30.
    /// @param verifyingKey A verifier key
    /// @param evalData A polynomial evaluation
    /// @param proof A Plonk proof
    /// @param chal A set of challenges
    /// @param commScalars Common scalars
    /// @param commBases Common bases
    // The returned commitment is a generalization of
    // `[F]1` described in Sec 8.4, step 10 of https://eprint.iacr.org/2019/953.pdf
    // Returned evaluation is the scalar in `[E]1` described in Sec 8.4, step 11 of
    // https://eprint.iacr.org/2019/953.pdf
    function _prepareOpeningProof(
        IPlonkVerifier.VerifyingKey memory verifyingKey,
        Poly.EvalData memory evalData,
        IPlonkVerifier.PlonkProof memory proof,
        Challenges memory chal,
        uint256[] memory commScalars,
        BN254.G1Point[] memory commBases
    ) internal pure returns (uint256 eval) {
        // compute the constant term of the linearization polynomial
        uint256 linPolyConstant = _computeLinPolyConstantTerm(chal, proof, evalData);

        _preparePolyCommitments(verifyingKey, chal, evalData, proof, commScalars, commBases);

        eval = _prepareEvaluations(linPolyConstant, proof, commScalars);
    }

    /// @dev Compute the constant term of the linearization polynomial.
    /// ```
    /// r_plonk = PI - L1(x) * alpha^2 - alpha * \prod_i=1..m-1 (w_i + beta * sigma_i + gamma) *
    /// (w_m + gamma) * z(xw)
    /// ```
    /// where m is the number of wire types.
    function _computeLinPolyConstantTerm(
        Challenges memory chal,
        IPlonkVerifier.PlonkProof memory proof,
        Poly.EvalData memory evalData
    ) internal pure returns (uint256 res) {
        uint256 p = BN254.R_MOD;
        uint256 lagrangeOneEval = BN254.ScalarField.unwrap(evalData.lagrangeOne);
        uint256 piEval = BN254.ScalarField.unwrap(evalData.piEval);
        uint256 perm = 1;

        assembly {
            let beta := mload(add(chal, 0x60))
            let gamma := mload(add(chal, 0x80))

            // \prod_i=1..m-1 (w_i + beta * sigma_i + gamma)
            {
                let w0 := mload(add(proof, 0x1a0))
                let sigma0 := mload(add(proof, 0x240))
                perm := mulmod(perm, addmod(add(w0, gamma), mulmod(beta, sigma0, p), p), p)
            }
            {
                let w1 := mload(add(proof, 0x1c0))
                let sigma1 := mload(add(proof, 0x260))
                perm := mulmod(perm, addmod(add(w1, gamma), mulmod(beta, sigma1, p), p), p)
            }
            {
                let w2 := mload(add(proof, 0x1e0))
                let sigma2 := mload(add(proof, 0x280))
                perm := mulmod(perm, addmod(add(w2, gamma), mulmod(beta, sigma2, p), p), p)
            }
            {
                let w3 := mload(add(proof, 0x200))
                let sigma3 := mload(add(proof, 0x2a0))
                perm := mulmod(perm, addmod(add(w3, gamma), mulmod(beta, sigma3, p), p), p)
            }

            // \prod_i=1..m-1 (w_i + beta * sigma_i + gamma) * (w_m + gamma) * z(xw)
            {
                let w4 := mload(add(proof, 0x220))
                let permNextEval := mload(add(proof, 0x2c0))
                perm := mulmod(perm, mulmod(addmod(w4, gamma, p), permNextEval, p), p)
            }

            let alpha := mload(chal)
            let alpha2 := mload(add(chal, 0x20))
            // PI - L1(x) * alpha^2 - alpha * \prod_i=1..m-1 (w_i + beta * sigma_i + gamma) * (w_m +
            // gamma) * z(xw)
            res := addmod(piEval, sub(p, mulmod(alpha2, lagrangeOneEval, p)), p)
            res := addmod(res, sub(p, mulmod(alpha, perm, p)), p)
        }
    }

    /// @dev Similar to `aggregate_poly_commitments()` in Jellyfish, but we are not aggregating
    /// multiple,
    /// but rather preparing for `[F]1` from a single proof.
    /// The caller allocates the memory fr commScalars and commBases.
    /// Requires Arrays of size 30.
    function _preparePolyCommitments(
        IPlonkVerifier.VerifyingKey memory verifyingKey,
        Challenges memory chal,
        Poly.EvalData memory evalData,
        IPlonkVerifier.PlonkProof memory proof,
        uint256[] memory commScalars,
        BN254.G1Point[] memory commBases
    ) internal pure {
        _linearizationScalarsAndBases(verifyingKey, chal, evalData, proof, commBases, commScalars);

        uint256 p = BN254.R_MOD;
        uint256 v = chal.v;
        uint256 vBase = v;

        // Add wire witness polynomial commitments.
        commScalars[20] = vBase;
        commBases[20] = proof.wire0;
        assembly {
            vBase := mulmod(vBase, v, p)
        }

        commScalars[21] = vBase;
        commBases[21] = proof.wire1;
        assembly {
            vBase := mulmod(vBase, v, p)
        }

        commScalars[22] = vBase;
        commBases[22] = proof.wire2;
        assembly {
            vBase := mulmod(vBase, v, p)
        }

        commScalars[23] = vBase;
        commBases[23] = proof.wire3;
        assembly {
            vBase := mulmod(vBase, v, p)
        }

        commScalars[24] = vBase;
        commBases[24] = proof.wire4;
        assembly {
            vBase := mulmod(vBase, v, p)
        }

        // Add wire sigma polynomial commitments. The last sigma commitment is excluded.
        commScalars[25] = vBase;
        commBases[25] = verifyingKey.sigma0;
        assembly {
            vBase := mulmod(vBase, v, p)
        }

        commScalars[26] = vBase;
        commBases[26] = verifyingKey.sigma1;
        assembly {
            vBase := mulmod(vBase, v, p)
        }

        commScalars[27] = vBase;
        commBases[27] = verifyingKey.sigma2;
        assembly {
            vBase := mulmod(vBase, v, p)
        }

        commScalars[28] = vBase;
        commBases[28] = verifyingKey.sigma3;
        assembly {
            vBase := mulmod(vBase, v, p)
        }

        // Add poly commitments to be evaluated at point `zeta * g`.
        commScalars[29] = chal.u;
        commBases[29] = proof.prodPerm;
    }

    /// @dev `aggregate_evaluations()` in Jellyfish, but since we are not aggregating multiple, but
    /// rather preparing `[E]1` from a single proof.
    /// @dev caller allocates the memory fr commScalars
    /// requires Arrays of size 30.
    /// @param linPolyConstant A linear polynomial constant
    /// @param proof A Plonk proof
    /// @param commScalars An array of common scalars
    /// The returned value is the scalar in `[E]1` described in Sec 8.4, step 11 of
    /// https://eprint.iacr.org/2019/953.pdf
    function _prepareEvaluations(
        uint256 linPolyConstant,
        IPlonkVerifier.PlonkProof memory proof,
        uint256[] memory commScalars
    ) internal pure returns (uint256 eval) {
        uint256 p = BN254.R_MOD;
        assembly {
            eval := sub(p, linPolyConstant)
            for { let i := 0 } lt(i, 10) { i := add(i, 1) } {
                // the first u256 stores the length of this array;
                // the next 20 elements are used to store the linearization of the scalars
                // the first free space starts from 21
                let combiner := mload(add(commScalars, mul(add(i, 21), 0x20)))
                let termEval := mload(add(proof, add(0x1a0, mul(i, 0x20))))
                eval := addmod(eval, mulmod(combiner, termEval, p), p)
            }
        }
    }

    /// @dev Verify opening proof
    /// `open_key` has been assembled from BN254.P1(), BN254.P2() and contract variable _betaH
    /// @param pcsInfo A single PcsInfo
    /// @dev Returns true if the pc opening verifies
    function _verifyOpeningProofs(PcsInfo memory pcsInfo) internal view returns (bool) {
        uint256 p = BN254.R_MOD;
        // Compute a pseudorandom challenge from the instances

        BN254.G1Point memory a1;
        BN254.G1Point memory b1;

        // Compute A
        {
            BN254.ScalarField[] memory scalars = new BN254.ScalarField[](2);
            BN254.G1Point[] memory bases = new BN254.G1Point[](2);
            uint256 rBase = 1;

            scalars[0] = BN254.ScalarField.wrap(rBase);
            bases[0] = pcsInfo.openingProof;

            scalars[1] = BN254.ScalarField.wrap(pcsInfo.u);

            bases[1] = pcsInfo.shiftedOpeningProof;

            a1 = BN254.multiScalarMul(bases, scalars);
        }

        // Compute B
        {
            BN254.ScalarField[] memory scalars;
            BN254.G1Point[] memory bases;
            {
                // variable scoping to avoid "Stack too deep"
                uint256 scalarsLenPerInfo = pcsInfo.commScalars.length;
                uint256 totalScalarsLen = (2 + scalarsLenPerInfo) + 1;
                scalars = new BN254.ScalarField[](totalScalarsLen);
                bases = new BN254.G1Point[](totalScalarsLen);
            }
            uint256 sumEvals = 0;
            uint256 idx = 0;

            for (uint256 j = 0; j < pcsInfo.commScalars.length; j++) {
                scalars[idx] = BN254.ScalarField.wrap(pcsInfo.commScalars[j]);

                bases[idx] = pcsInfo.commBases[j];
                idx += 1;
            }

            scalars[idx] = BN254.ScalarField.wrap(pcsInfo.evalPoint);

            bases[idx] = pcsInfo.openingProof;
            idx += 1;

            {
                uint256 u = pcsInfo.u;
                uint256 nextEvalPoint = pcsInfo.nextEvalPoint;
                uint256 tmp;
                assembly {
                    // slither-disable-next-line variable-scope
                    tmp := mulmod(u, nextEvalPoint, p)
                }
                scalars[idx] = BN254.ScalarField.wrap(tmp);
            }
            bases[idx] = pcsInfo.shiftedOpeningProof;
            idx += 1;

            {
                uint256 eval = pcsInfo.eval;
                assembly {
                    sumEvals := addmod(sumEvals, eval, p)
                }
            }

            scalars[idx] = BN254.negate(BN254.ScalarField.wrap(sumEvals));
            bases[idx] = BN254.P1();
            b1 = BN254.negate(BN254.multiScalarMul(bases, scalars));
        }

        // Check e(A, [x]2) ?= e(B, [1]2)
        BN254.G2Point memory betaH = BN254.G2Point({
            x0: BN254.BaseField.wrap(BETA_H_X1),
            x1: BN254.BaseField.wrap(BETA_H_X0),
            y0: BN254.BaseField.wrap(BETA_H_Y1),
            y1: BN254.BaseField.wrap(BETA_H_Y0)
        });

        return BN254.pairingProd2(a1, betaH, b1, BN254.P2());
    }

    /// @dev Batchly verify multiple PCS opening proofs.
    /// `open_key` has been assembled from BN254.P1(), BN254.P2() and contract variable _betaH
    /// @param pcsInfos An array of PcsInfo
    /// @dev Returns true if the entire batch verifiies and false otherwise.
    function _batchVerifyOpeningProofs(PcsInfo[] memory pcsInfos) internal view returns (bool) {
        uint256 pcsLen = pcsInfos.length;
        uint256 p = BN254.R_MOD;
        // Compute a pseudorandom challenge from the instances
        uint256 r = 1; // for a single proof, no need to use `r` (`r=1` has no effect)
        if (pcsLen > 1) {
            Transcript.TranscriptData memory transcript;
            for (uint256 i = 0; i < pcsLen; i++) {
                transcript.appendChallenge(pcsInfos[i].u);
            }
            r = transcript.getAndAppendChallenge();
        }

        BN254.G1Point memory a1;
        BN254.G1Point memory b1;

        // Compute A := A0 + r * A1 + ... + r^{m-1} * Am
        {
            BN254.ScalarField[] memory scalars = new BN254.ScalarField[](2 * pcsLen);
            BN254.G1Point[] memory bases = new BN254.G1Point[](2 * pcsLen);
            uint256 rBase = 1;
            for (uint256 i = 0; i < pcsLen; i++) {
                scalars[2 * i] = BN254.ScalarField.wrap(rBase);
                bases[2 * i] = pcsInfos[i].openingProof;

                {
                    // slither-disable-next-line write-after-write
                    uint256 tmp;
                    uint256 u = pcsInfos[i].u;
                    assembly {
                        tmp := mulmod(rBase, u, p)
                    }
                    scalars[2 * i + 1] = BN254.ScalarField.wrap(tmp);
                }
                bases[2 * i + 1] = pcsInfos[i].shiftedOpeningProof;

                assembly {
                    rBase := mulmod(rBase, r, p)
                }
            }
            a1 = BN254.multiScalarMul(bases, scalars);
        }

        // Compute B := B0 + r * B1 + ... + r^{m-1} * Bm
        {
            BN254.ScalarField[] memory scalars;
            BN254.G1Point[] memory bases;
            {
                // variable scoping to avoid "Stack too deep"
                uint256 scalarsLenPerInfo = pcsInfos[0].commScalars.length;
                uint256 totalScalarsLen = (2 + scalarsLenPerInfo) * pcsInfos.length + 1;
                scalars = new BN254.ScalarField[](totalScalarsLen);
                bases = new BN254.G1Point[](totalScalarsLen);
            }
            uint256 sumEvals = 0;
            uint256 idx = 0;
            uint256 rBase = 1;
            for (uint256 i = 0; i < pcsInfos.length; i++) {
                for (uint256 j = 0; j < pcsInfos[0].commScalars.length; j++) {
                    {
                        // scalars[idx] = (rBase * pcsInfos[i].commScalars[j]) % BN254.R_MOD;
                        uint256 s = pcsInfos[i].commScalars[j];
                        uint256 tmp;
                        assembly {
                            // slither-disable-next-line variable-scope
                            tmp := mulmod(rBase, s, p)
                        }
                        scalars[idx] = BN254.ScalarField.wrap(tmp);
                    }
                    bases[idx] = pcsInfos[i].commBases[j];
                    idx += 1;
                }

                {
                    // scalars[idx] = (rBase * pcsInfos[i].evalPoint) % BN254.R_MOD;
                    uint256 evalPoint = pcsInfos[i].evalPoint;
                    uint256 tmp;
                    assembly {
                        // slither-disable-next-line variable-scope
                        tmp := mulmod(rBase, evalPoint, p)
                    }
                    scalars[idx] = BN254.ScalarField.wrap(tmp);
                }
                bases[idx] = pcsInfos[i].openingProof;
                idx += 1;

                {
                    // scalars[idx] = (rBase * pcsInfos[i].u * pcsInfos[i].nextEvalPoint) %
                    // BN254.R_MOD;
                    uint256 u = pcsInfos[i].u;
                    uint256 nextEvalPoint = pcsInfos[i].nextEvalPoint;
                    uint256 tmp;
                    assembly {
                        // slither-disable-next-line variable-scope
                        tmp := mulmod(rBase, mulmod(u, nextEvalPoint, p), p)
                    }
                    scalars[idx] = BN254.ScalarField.wrap(tmp);
                }
                bases[idx] = pcsInfos[i].shiftedOpeningProof;
                idx += 1;

                {
                    // sumEvals = (sumEvals + rBase * pcsInfos[i].eval) % BN254.R_MOD;
                    // rBase = (rBase * r) % BN254.R_MOD;
                    uint256 eval = pcsInfos[i].eval;
                    assembly {
                        sumEvals := addmod(sumEvals, mulmod(rBase, eval, p), p)
                        rBase := mulmod(rBase, r, p)
                    }
                }
            }
            scalars[idx] = BN254.negate(BN254.ScalarField.wrap(sumEvals));
            bases[idx] = BN254.P1();
            b1 = BN254.negate(BN254.multiScalarMul(bases, scalars));
        }

        // Check e(A, [x]2) ?= e(B, [1]2)
        // TODO the tests pass but it feels wrong.
        BN254.G2Point memory betaH = BN254.G2Point({
            x0: BN254.BaseField.wrap(BETA_H_X1),
            x1: BN254.BaseField.wrap(BETA_H_X0),
            y0: BN254.BaseField.wrap(BETA_H_Y1),
            y1: BN254.BaseField.wrap(BETA_H_Y0)
        });

        return BN254.pairingProd2(a1, betaH, b1, BN254.P2());
    }

    /// @dev Compute the linearization of the scalars and bases.
    /// The caller allocates the memory from commScalars and commBases.
    /// Requires arrays of size 30.
    /// @param verifyingKey The verifying key
    /// @param challenge A set of challenges
    /// @param evalData Polynomial evaluation data
    /// @param proof A Plonk proof
    /// @param bases An array of BN254 G1 points
    /// @param scalars An array of scalars
    function _linearizationScalarsAndBases(
        IPlonkVerifier.VerifyingKey memory verifyingKey,
        Challenges memory challenge,
        Poly.EvalData memory evalData,
        IPlonkVerifier.PlonkProof memory proof,
        BN254.G1Point[] memory bases,
        uint256[] memory scalars
    ) internal pure {
        uint256 firstScalar;
        uint256 secondScalar;
        uint256 rhs;
        uint256 tmp;
        uint256 tmp2;
        uint256 p = BN254.R_MOD;

        // ============================================
        // Compute coefficient for the permutation product polynomial commitment.
        // firstScalar =
        //          L1(zeta) * alpha^2
        //          + alpha
        //              * (beta * zeta      + wireEval0 + gamma)
        //              * (beta * k1 * zeta + wireEval1 + gamma)
        //              * (beta * k2 * zeta + wireEval2 + gamma)
        //              * ...
        // where wireEval0, wireEval1, wireEval2, ... are in w_evals
        // ============================================
        // first base and scala:
        // - proof.prodPerm
        // - firstScalar
        assembly {
            // firstScalar = alpha^2 * L1(zeta)
            firstScalar := mulmod(mload(add(challenge, 0x20)), mload(add(evalData, 0x20)), p)

            // rhs = alpha
            rhs := mload(challenge)

            // tmp = beta * zeta
            tmp := mulmod(mload(add(challenge, 0x60)), mload(add(challenge, 0xA0)), p)

            // =================================
            // k0 (which is 1) component
            // (beta * zeta + wireEval0 + gamma)
            // =================================
            tmp2 := addmod(tmp, mload(add(proof, 0x1A0)), p)
            tmp2 := addmod(tmp2, mload(add(challenge, 0x80)), p)

            rhs := mulmod(tmp2, rhs, p)

            // =================================
            // k1 component
            // (beta * zeta * k1 + wireEval1 + gamma)
            // =================================
            tmp2 := mulmod(tmp, COSET_K1, p)
            tmp2 := addmod(tmp2, mload(add(proof, 0x1C0)), p)
            tmp2 := addmod(tmp2, mload(add(challenge, 0x80)), p)

            rhs := mulmod(tmp2, rhs, p)

            // =================================
            // k2 component
            // (beta * zeta * k2 + wireEval2 + gamma)
            // =================================
            tmp2 := mulmod(tmp, COSET_K2, p)
            tmp2 := addmod(tmp2, mload(add(proof, 0x1E0)), p)
            tmp2 := addmod(tmp2, mload(add(challenge, 0x80)), p)
            rhs := mulmod(tmp2, rhs, p)

            // =================================
            // k3 component
            // (beta * zeta * k3 + wireEval3 + gamma)
            // =================================
            tmp2 := mulmod(tmp, COSET_K3, p)
            tmp2 := addmod(tmp2, mload(add(proof, 0x200)), p)
            tmp2 := addmod(tmp2, mload(add(challenge, 0x80)), p)
            rhs := mulmod(tmp2, rhs, p)

            // =================================
            // k4 component
            // (beta * zeta * k4 + wireEval4 + gamma)
            // =================================
            tmp2 := mulmod(tmp, COSET_K4, p)
            tmp2 := addmod(tmp2, mload(add(proof, 0x220)), p)
            tmp2 := addmod(tmp2, mload(add(challenge, 0x80)), p)
            rhs := mulmod(tmp2, rhs, p)

            firstScalar := addmod(firstScalar, rhs, p)
        }
        bases[0] = proof.prodPerm;
        scalars[0] = firstScalar;

        // ============================================
        // Compute coefficient for the last wire sigma polynomial commitment.
        // secondScalar = alpha * beta * z_w * [s_sigma_3]_1
        //              * (wireEval0 + gamma + beta * sigmaEval0)
        //              * (wireEval1 + gamma + beta * sigmaEval1)
        //              * ...
        // ============================================
        // second base and scala:
        // - verifyingKey.sigma4
        // - secondScalar
        assembly {
            // secondScalar = alpha * beta * z_w
            secondScalar := mulmod(mload(challenge), mload(add(challenge, 0x60)), p)
            secondScalar := mulmod(secondScalar, mload(add(proof, 0x2C0)), p)

            // (wireEval0 + gamma + beta * sigmaEval0)
            tmp := mulmod(mload(add(challenge, 0x60)), mload(add(proof, 0x240)), p)
            tmp := addmod(tmp, mload(add(proof, 0x1A0)), p)
            tmp := addmod(tmp, mload(add(challenge, 0x80)), p)

            secondScalar := mulmod(secondScalar, tmp, p)

            // (wireEval1 + gamma + beta * sigmaEval1)
            tmp := mulmod(mload(add(challenge, 0x60)), mload(add(proof, 0x260)), p)
            tmp := addmod(tmp, mload(add(proof, 0x1C0)), p)
            tmp := addmod(tmp, mload(add(challenge, 0x80)), p)

            secondScalar := mulmod(secondScalar, tmp, p)

            // (wireEval2 + gamma + beta * sigmaEval2)
            tmp := mulmod(mload(add(challenge, 0x60)), mload(add(proof, 0x280)), p)
            tmp := addmod(tmp, mload(add(proof, 0x1E0)), p)
            tmp := addmod(tmp, mload(add(challenge, 0x80)), p)

            secondScalar := mulmod(secondScalar, tmp, p)

            // (wireEval3 + gamma + beta * sigmaEval3)
            tmp := mulmod(mload(add(challenge, 0x60)), mload(add(proof, 0x2A0)), p)
            tmp := addmod(tmp, mload(add(proof, 0x200)), p)
            tmp := addmod(tmp, mload(add(challenge, 0x80)), p)

            secondScalar := mulmod(secondScalar, tmp, p)
        }
        bases[1] = verifyingKey.sigma4;
        scalars[1] = p - secondScalar;

        // ============================================
        // next 13 are for selectors:
        //
        // the selectors are organized as
        // - q_lc
        // - q_mul
        // - q_hash
        // - q_o
        // - q_c
        // - q_ecc
        // ============================================

        // ============
        // q_lc
        // ============
        // q_1...q_4
        scalars[2] = BN254.ScalarField.unwrap(proof.wireEval0);
        scalars[3] = BN254.ScalarField.unwrap(proof.wireEval1);
        scalars[4] = BN254.ScalarField.unwrap(proof.wireEval2);
        scalars[5] = BN254.ScalarField.unwrap(proof.wireEval3);
        bases[2] = verifyingKey.q1;
        bases[3] = verifyingKey.q2;
        bases[4] = verifyingKey.q3;
        bases[5] = verifyingKey.q4;

        // ============
        // q_M
        // ============
        // q_M12 and q_M34
        // q_M12 = w_evals[0] * w_evals[1];
        assembly {
            tmp := mulmod(mload(add(proof, 0x1A0)), mload(add(proof, 0x1C0)), p)
        }
        scalars[6] = tmp;
        bases[6] = verifyingKey.qM12;

        assembly {
            tmp := mulmod(mload(add(proof, 0x1E0)), mload(add(proof, 0x200)), p)
        }
        scalars[7] = tmp;
        bases[7] = verifyingKey.qM34;

        // ============
        // q_H
        // ============
        // w_evals[0].pow([5]);
        assembly {
            tmp := mload(add(proof, 0x1A0))
            tmp2 := mulmod(tmp, tmp, p)
            tmp2 := mulmod(tmp2, tmp2, p)
            tmp := mulmod(tmp, tmp2, p)
        }
        scalars[8] = tmp;
        bases[8] = verifyingKey.qH1;

        // w_evals[1].pow([5]);
        assembly {
            tmp := mload(add(proof, 0x1C0))
            tmp2 := mulmod(tmp, tmp, p)
            tmp2 := mulmod(tmp2, tmp2, p)
            tmp := mulmod(tmp, tmp2, p)
        }
        scalars[9] = tmp;
        bases[9] = verifyingKey.qH2;

        // w_evals[2].pow([5]);
        assembly {
            tmp := mload(add(proof, 0x1E0))
            tmp2 := mulmod(tmp, tmp, p)
            tmp2 := mulmod(tmp2, tmp2, p)
            tmp := mulmod(tmp, tmp2, p)
        }
        scalars[10] = tmp;
        bases[10] = verifyingKey.qH3;

        // w_evals[3].pow([5]);
        assembly {
            tmp := mload(add(proof, 0x200))
            tmp2 := mulmod(tmp, tmp, p)
            tmp2 := mulmod(tmp2, tmp2, p)
            tmp := mulmod(tmp, tmp2, p)
        }
        scalars[11] = tmp;
        bases[11] = verifyingKey.qH4;

        // ============
        // q_o and q_c
        // ============
        // q_o
        scalars[12] = p - BN254.ScalarField.unwrap(proof.wireEval4);
        bases[12] = verifyingKey.qO;
        // q_c
        scalars[13] = 1;
        bases[13] = verifyingKey.qC;

        // ============
        // q_Ecc
        // ============
        // q_Ecc = w_evals[0] * w_evals[1] * w_evals[2] * w_evals[3] * w_evals[4];
        assembly {
            tmp := mulmod(mload(add(proof, 0x1A0)), mload(add(proof, 0x1C0)), p)
            tmp := mulmod(tmp, mload(add(proof, 0x1E0)), p)
            tmp := mulmod(tmp, mload(add(proof, 0x200)), p)
            tmp := mulmod(tmp, mload(add(proof, 0x220)), p)
        }
        scalars[14] = tmp;
        bases[14] = verifyingKey.qEcc;

        // ============================================
        // the last 5 are for splitting quotient commitments
        // ============================================

        // first one is 1-zeta^n
        scalars[15] = p - BN254.ScalarField.unwrap(evalData.vanishEval);
        bases[15] = proof.split0;
        assembly {
            // tmp = zeta^{n+2}
            tmp := addmod(mload(evalData), 1, p)
            // todo: use pre-computed zeta^2
            tmp2 := mulmod(mload(add(challenge, 0xA0)), mload(add(challenge, 0xA0)), p)
            tmp := mulmod(tmp, tmp2, p)
        }

        // second one is (1-zeta^n) zeta^(n+2)
        assembly {
            tmp2 := mulmod(mload(add(scalars, mul(16, 0x20))), tmp, p)
        }
        scalars[16] = tmp2;
        bases[16] = proof.split1;

        // third one is (1-zeta^n) zeta^2(n+2)
        assembly {
            tmp2 := mulmod(mload(add(scalars, mul(17, 0x20))), tmp, p)
        }
        scalars[17] = tmp2;
        bases[17] = proof.split2;

        // forth one is (1-zeta^n) zeta^3(n+2)
        assembly {
            tmp2 := mulmod(mload(add(scalars, mul(18, 0x20))), tmp, p)
        }
        scalars[18] = tmp2;
        bases[18] = proof.split3;

        // fifth one is (1-zeta^n) zeta^4(n+2)
        assembly {
            tmp2 := mulmod(mload(add(scalars, mul(19, 0x20))), tmp, p)
        }
        scalars[19] = tmp2;
        bases[19] = proof.split4;
    }
}
