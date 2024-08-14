// SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.8.0;

import { BN254 } from "bn254/BN254.sol";
import { PolynomialEval as Poly } from "./PolynomialEval.sol";
import { IPlonkVerifier } from "../interfaces/IPlonkVerifier.sol";

library PlonkVerifier2 {
    // use notation from https://datatracker.ietf.org/doc/draft-irtf-cfrg-pairing-friendly-curves/
    //
    // Elliptic curve is defined over a prime field GF(p), with embedding degree k.
    // Short Weierstrass (SW form) is, for a, b \in GF(p^n) for some natural number n > 0:
    //   E: y^2 = x^3 + a * x + b
    //
    // Pairing is defined over cyclic subgroups G1, G2, both of which are of order r.
    // G1 is a subgroup of E(GF(p)), G2 is a subgroup of E(GF(p^k)).
    //
    // BN family are parameterized curves with well-chosen t,
    //   p = 36 * t^4 + 36 * t^3 + 24 * t^2 + 6 * t + 1
    //   r = 36 * t^4 + 36 * t^3 + 18 * t^2 + 6 * t + 1
    // for some integer t.
    // E has the equation:
    //   E: y^2 = x^3 + b
    // where b is a primitive element of multiplicative group (GF(p))^* of order (p-1).
    // A pairing e is defined by taking G1 as a subgroup of E(GF(p)) of order r,
    // G2 as a subgroup of E'(GF(p^2)),
    // and G_T as a subgroup of a multiplicative group (GF(p^12))^* of order r.
    //
    // BN254 is defined over a 254-bit prime order p, embedding degree k = 12.
    uint256 public constant P_MOD =
        21888242871839275222246405745257275088696311157297823662689037894645226208583;
    uint256 public constant R_MOD =
        21888242871839275222246405745257275088548364400416034343698204186575808495617;

    // _COSET_K0 = 1, has no effect during multiplication, thus avoid declaring it here.
    uint256 private constant COSET_K1 =
        0x2f8dd1f1a7583c42c4e12a44e110404c73ca6c94813f85835da4fb7bb1301d4a;
    uint256 private constant COSET_K2 =
        0x1ee678a0470a75a6eaa8fe837060498ba828a3703b311d0f77f010424afeb025;
    uint256 private constant COSET_K3 =
        0x2042a587a90c187b0a087c03e29c968b950b1db26d5c82d666905a6895790c0a;
    uint256 private constant COSET_K4 =
        0x2e2b91456103698adf57b799969dea1c8f739da5d8d40dd3eb9222db7c81e881;

    // Parsed from Aztec's Ignition CRS,
    // `beta_h` \in G2 where \beta is the trapdoor, h is G2 generator `BN254.P2()`
    // See parsing code: https://github.com/alxiong/crs
    uint256 private constant BETA_H_X0 =
        0x260e01b251f6f1c7e7ff4e580791dee8ea51d87a358e038b4efe30fac09383c1;
    uint256 private constant BETA_H_X1 =
        0x0118c4d5b837bcc2bc89b5b398b5974e9f5944073b32078b7e231fec938883b0;
    uint256 private constant BETA_H_Y0 =
        0x04fc6369f7110fe3d25156c1bb9a72859cf2a04641f99ba4ee413c80da6a5fe4;
    uint256 private constant BETA_H_Y1 =
        0x22febda3c0c0632a56475b4214e5615e11e6dd3f96e6cea2854a87d4dacc5e55;

    // BN254.P2
    uint256 private constant H_X0 =
        0x198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c2;
    uint256 private constant H_X1 =
        0x1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed;
    uint256 private constant H_Y0 =
        0x090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b;
    uint256 private constant H_Y1 =
        0x12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa;

    /// The number of wire types of the circuit, TurboPlonk has 5.
    uint256 private constant NUM_WIRE_TYPES = 5;

    uint256 private constant ERROR_STRING_ID =
        0x08c379a000000000000000000000000000000000000000000000000000000000; // selector for function
        // Error(string)

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
                perm :=
                    mulmod(
                        perm,
                        addmod(addmod(w0, gamma, R_MOD), mulmod(beta, sigma0, R_MOD), R_MOD),
                        R_MOD
                    )
            }
            {
                let w1 := mload(add(proof, 0x1c0))
                let sigma1 := mload(add(proof, 0x260))
                perm :=
                    mulmod(
                        perm,
                        addmod(addmod(w1, gamma, R_MOD), mulmod(beta, sigma1, R_MOD), R_MOD),
                        R_MOD
                    )
            }
            {
                let w2 := mload(add(proof, 0x1e0))
                let sigma2 := mload(add(proof, 0x280))
                perm :=
                    mulmod(
                        perm,
                        addmod(addmod(w2, gamma, R_MOD), mulmod(beta, sigma2, R_MOD), R_MOD),
                        R_MOD
                    )
            }
            {
                let w3 := mload(add(proof, 0x200))
                let sigma3 := mload(add(proof, 0x2a0))
                perm :=
                    mulmod(
                        perm,
                        addmod(addmod(w3, gamma, R_MOD), mulmod(beta, sigma3, R_MOD), R_MOD),
                        R_MOD
                    )
            }

            // \prod_i=1..m-1 (w_i + beta * sigma_i + gamma) * (w_m + gamma) * z(xw)
            {
                let w4 := mload(add(proof, 0x220))
                let permNextEval := mload(add(proof, 0x2c0))
                perm := mulmod(perm, mulmod(addmod(w4, gamma, R_MOD), permNextEval, R_MOD), R_MOD)
            }

            let alpha := mload(chal)
            let alpha2 := mload(add(chal, 0x20))
            // PI - L1(x) * alpha^2 - alpha * \prod_i=1..m-1 (w_i + beta * sigma_i + gamma) * (w_m +
            // gamma) * z(xw)
            res := addmod(piEval, sub(R_MOD, mulmod(alpha2, lagrangeOneEval, R_MOD)), R_MOD)
            res := addmod(res, sub(R_MOD, mulmod(alpha, perm, R_MOD)), R_MOD)
        }
    }

    function _computeChallenges(
        IPlonkVerifier.VerifyingKey memory vk,
        uint256[8] memory pi,
        IPlonkVerifier.PlonkProof memory proof
    ) internal pure returns (Challenges memory res) {
        assembly {
            // use free memory space for scratch pad, 0x40: free memory ptr
            let statePtr := mload(0x40)
            let dataPtr := add(statePtr, 0x20)

            // Start of transcript (unit: bytes)
            // sizeInBits (4)  | domainSize (8) | numInputs (8) | pad (12)
            mstore(dataPtr, 0) // initialize to 0 first
            mstore(dataPtr, shl(224, 254)) // sizeInBits
            mstore(add(dataPtr, 4), shl(192, mload(vk))) // domainSize
            mstore(add(dataPtr, 12), shl(192, mload(add(vk, 0x20)))) // numInputs

            // G2 from SRS
            mstore(add(dataPtr, 0x20), mload(add(vk, 0x280))) // g2LSB (32)
            mstore(add(dataPtr, 0x40), mload(add(vk, 0x2a0))) // g2MSB (32)

            // k0 ~ k4
            mstore(add(dataPtr, 0x60), 0x1)
            mstore(
                add(dataPtr, 0x80),
                0x2f8dd1f1a7583c42c4e12a44e110404c73ca6c94813f85835da4fb7bb1301d4a
            )
            mstore(
                add(dataPtr, 0xa0),
                0x1ee678a0470a75a6eaa8fe837060498ba828a3703b311d0f77f010424afeb025
            )
            mstore(
                add(dataPtr, 0xc0),
                0x2042a587a90c187b0a087c03e29c968b950b1db26d5c82d666905a6895790c0a
            )
            mstore(
                add(dataPtr, 0xe0),
                0x2e2b91456103698adf57b799969dea1c8f739da5d8d40dd3eb9222db7c81e881
            )

            // selectors
            let q1Ptr := mload(add(vk, 0xe0))
            mstore(add(dataPtr, 0x100), mload(q1Ptr)) // q1.x
            mstore(add(dataPtr, 0x120), mload(add(q1Ptr, 0x20))) // q1.y
            let q2Ptr := mload(add(vk, 0x100))
            mstore(add(dataPtr, 0x140), mload(q2Ptr)) // q2.x
            mstore(add(dataPtr, 0x160), mload(add(q2Ptr, 0x20))) // q2.y
            let q3Ptr := mload(add(vk, 0x120))
            mstore(add(dataPtr, 0x180), mload(q3Ptr)) // q3.x
            mstore(add(dataPtr, 0x1a0), mload(add(q3Ptr, 0x20))) // q3.y
            let q4Ptr := mload(add(vk, 0x140))
            mstore(add(dataPtr, 0x1c0), mload(q4Ptr)) // q4.x
            mstore(add(dataPtr, 0x1e0), mload(add(q4Ptr, 0x20))) // q4.y
            let qM12Ptr := mload(add(vk, 0x160))
            mstore(add(dataPtr, 0x200), mload(qM12Ptr)) // qM12.x
            mstore(add(dataPtr, 0x220), mload(add(qM12Ptr, 0x20))) // qM12.y
            let qM34Ptr := mload(add(vk, 0x180))
            mstore(add(dataPtr, 0x240), mload(qM34Ptr)) // qM34.x
            mstore(add(dataPtr, 0x260), mload(add(qM34Ptr, 0x20))) // qM34.y
            let qH1Ptr := mload(add(vk, 0x1e0))
            mstore(add(dataPtr, 0x280), mload(qH1Ptr)) // qH1.x
            mstore(add(dataPtr, 0x2a0), mload(add(qH1Ptr, 0x20))) // qH1.y
            let qH2Ptr := mload(add(vk, 0x200))
            mstore(add(dataPtr, 0x2c0), mload(qH2Ptr)) // qH2.x
            mstore(add(dataPtr, 0x2e0), mload(add(qH2Ptr, 0x20))) // qH2.y
            let qH3Ptr := mload(add(vk, 0x220))
            mstore(add(dataPtr, 0x300), mload(qH3Ptr)) // qH3.x
            mstore(add(dataPtr, 0x320), mload(add(qH3Ptr, 0x20))) // qH3.y
            let qH4Ptr := mload(add(vk, 0x240))
            mstore(add(dataPtr, 0x340), mload(qH4Ptr)) // qH4.x
            mstore(add(dataPtr, 0x360), mload(add(qH4Ptr, 0x20))) // qH4.y
            let qOPtr := mload(add(vk, 0x1a0))
            mstore(add(dataPtr, 0x380), mload(qOPtr)) // qO.x
            mstore(add(dataPtr, 0x3a0), mload(add(qOPtr, 0x20))) // qO.y
            let qCPtr := mload(add(vk, 0x1c0))
            mstore(add(dataPtr, 0x3c0), mload(qCPtr)) // qC.x
            mstore(add(dataPtr, 0x3e0), mload(add(qCPtr, 0x20))) // qC.y
            let qECCPtr := mload(add(vk, 0x260))
            mstore(add(dataPtr, 0x400), mload(qECCPtr)) // qECC.x
            mstore(add(dataPtr, 0x420), mload(add(qECCPtr, 0x20))) // qECC.y

            // sigmas
            let sigma0Ptr := mload(add(vk, 0x40))
            mstore(add(dataPtr, 0x440), mload(sigma0Ptr)) // sigma0.x
            mstore(add(dataPtr, 0x460), mload(add(sigma0Ptr, 0x20))) // sigma0.y
            let sigma1Ptr := mload(add(vk, 0x60))
            mstore(add(dataPtr, 0x480), mload(sigma1Ptr)) // sigma1.x
            mstore(add(dataPtr, 0x4a0), mload(add(sigma1Ptr, 0x20))) // sigma1.y
            let sigma2Ptr := mload(add(vk, 0x80))
            mstore(add(dataPtr, 0x4c0), mload(sigma2Ptr)) // sigma2.x
            mstore(add(dataPtr, 0x4e0), mload(add(sigma2Ptr, 0x20))) // sigma2.y
            let sigma3Ptr := mload(add(vk, 0xa0))
            mstore(add(dataPtr, 0x500), mload(sigma3Ptr)) // sigma3.x
            mstore(add(dataPtr, 0x520), mload(add(sigma3Ptr, 0x20))) // sigma3.y
            let sigma4Ptr := mload(add(vk, 0xc0))
            mstore(add(dataPtr, 0x540), mload(sigma4Ptr)) // sigma4.x
            mstore(add(dataPtr, 0x560), mload(add(sigma4Ptr, 0x20))) // sigma4.y

            // public inputs
            mstore(add(dataPtr, 0x580), mload(pi)) // PI[0]
            mstore(add(dataPtr, 0x5a0), mload(add(pi, 0x20))) // PI[1]
            mstore(add(dataPtr, 0x5c0), mload(add(pi, 0x40))) // PI[2]
            mstore(add(dataPtr, 0x5e0), mload(add(pi, 0x60))) // PI[3]
            mstore(add(dataPtr, 0x600), mload(add(pi, 0x80))) // PI[4]
            mstore(add(dataPtr, 0x620), mload(add(pi, 0xa0))) // PI[5]
            mstore(add(dataPtr, 0x640), mload(add(pi, 0xc0))) // PI[6]
            mstore(add(dataPtr, 0x660), mload(add(pi, 0xe0))) // PI[7]

            // proof
            let wire0Ptr := mload(proof)
            mstore(add(dataPtr, 0x680), mload(wire0Ptr)) // wire0.x
            mstore(add(dataPtr, 0x6a0), mload(add(wire0Ptr, 0x20))) // wire0.y
            let wire1Ptr := mload(add(proof, 0x20))
            mstore(add(dataPtr, 0x6c0), mload(wire1Ptr)) // wire1.x
            mstore(add(dataPtr, 0x6e0), mload(add(wire1Ptr, 0x20))) // wire1.y
            let wire2Ptr := mload(add(proof, 0x40))
            mstore(add(dataPtr, 0x700), mload(wire2Ptr)) // wire2.x
            mstore(add(dataPtr, 0x720), mload(add(wire2Ptr, 0x20))) // wire2.y
            let wire3Ptr := mload(add(proof, 0x60))
            mstore(add(dataPtr, 0x740), mload(wire3Ptr)) // wire3.x
            mstore(add(dataPtr, 0x760), mload(add(wire3Ptr, 0x20))) // wire3.y
            let wire4Ptr := mload(add(proof, 0x80))
            mstore(add(dataPtr, 0x780), mload(wire4Ptr)) // wire4.x
            mstore(add(dataPtr, 0x7a0), mload(add(wire4Ptr, 0x20))) // wire4.y

            // challenge: beta
            {
                mstore(statePtr, 0x0) // init state
                // preimage len: state(0x20) + transcript(0x7c0)
                mstore(add(dataPtr, 0x7c0), keccak256(statePtr, 0x7e0))
                // update new state (by updating state pointer)
                statePtr := add(dataPtr, 0x7c0)
                // empty transcript
                dataPtr := add(statePtr, 0x20)
                // (mod R_MOD) to get beta
                mstore(add(res, 0x60), mod(mload(statePtr), R_MOD))
            }

            // challenge: gamma
            {
                // preimage len: state(0x20) + transcript(0x0)
                mstore(dataPtr, keccak256(statePtr, 0x20))
                // update new state (by updating state pointer)
                statePtr := dataPtr
                // empty transcript
                dataPtr := add(statePtr, 0x20)
                // (mod R_MOD) to get gamma
                mstore(add(res, 0x80), mod(mload(statePtr), R_MOD))
            }

            let prodPermPtr := mload(add(proof, 0xa0))
            mstore(dataPtr, mload(prodPermPtr)) // prodPerm.x
            mstore(add(dataPtr, 0x20), mload(add(prodPermPtr, 0x20))) // prodPerm.y

            // challenge: alpha, alpha2, alpha3
            {
                // preimage len: state(0x20) + transcript(0x40)
                let alpha := keccak256(statePtr, 0x60)
                // update new state (by updating state pointer)
                statePtr := add(dataPtr, 0x40)
                mstore(statePtr, alpha)
                // empty transcript
                dataPtr := add(statePtr, 0x20)
                // (mod R_MOD) to get challenge
                mstore(res, mod(alpha, R_MOD))

                let alpha2 := mulmod(alpha, alpha, R_MOD)
                let alpha3 := mulmod(alpha2, alpha, R_MOD)
                mstore(add(res, 0x20), alpha2)
                mstore(add(res, 0x40), alpha3)
            }

            let split0Ptr := mload(add(proof, 0xc0))
            mstore(dataPtr, mload(split0Ptr)) // split0.x
            mstore(add(dataPtr, 0x20), mload(add(split0Ptr, 0x20))) // split0.y
            let split1Ptr := mload(add(proof, 0xe0))
            mstore(add(dataPtr, 0x40), mload(split1Ptr)) // split1.x
            mstore(add(dataPtr, 0x60), mload(add(split1Ptr, 0x20))) // split1.y
            let split2Ptr := mload(add(proof, 0x100))
            mstore(add(dataPtr, 0x80), mload(split2Ptr)) // split2.x
            mstore(add(dataPtr, 0xa0), mload(add(split2Ptr, 0x20))) // split2.y
            let split3Ptr := mload(add(proof, 0x120))
            mstore(add(dataPtr, 0xc0), mload(split3Ptr)) // split3.x
            mstore(add(dataPtr, 0xe0), mload(add(split3Ptr, 0x20))) // split3.y
            let split4Ptr := mload(add(proof, 0x140))
            mstore(add(dataPtr, 0x100), mload(split4Ptr)) // split4.x
            mstore(add(dataPtr, 0x120), mload(add(split4Ptr, 0x20))) // split4.y

            // challenge: zeta
            {
                // preimage len: state(0x20) + transcript(0x140)
                mstore(add(dataPtr, 0x140), keccak256(statePtr, 0x160))
                // update new state (by updating state pointer)
                statePtr := add(dataPtr, 0x140)
                // empty transcript
                dataPtr := add(statePtr, 0x20)
                // (mod R_MOD) to get challenge
                mstore(add(res, 0xa0), mod(mload(statePtr), R_MOD))
            }

            mstore(dataPtr, mload(add(proof, 0x1a0))) // wireEval0
            mstore(add(dataPtr, 0x20), mload(add(proof, 0x1c0))) // wireEval1
            mstore(add(dataPtr, 0x40), mload(add(proof, 0x1e0))) // wireEval2
            mstore(add(dataPtr, 0x60), mload(add(proof, 0x200))) // wireEval3
            mstore(add(dataPtr, 0x80), mload(add(proof, 0x220))) // wireEval4
            mstore(add(dataPtr, 0xa0), mload(add(proof, 0x240))) // sigmaEval0
            mstore(add(dataPtr, 0xc0), mload(add(proof, 0x260))) // sigmaEval1
            mstore(add(dataPtr, 0xe0), mload(add(proof, 0x280))) // sigmaEval2
            mstore(add(dataPtr, 0x100), mload(add(proof, 0x2a0))) // sigmaEval3
            mstore(add(dataPtr, 0x120), mload(add(proof, 0x2c0))) // prodPermZetaOmegaEval

            // challenge: v
            {
                // preimage len: state(0x20) + transcript(0x140)
                mstore(add(dataPtr, 0x140), keccak256(statePtr, 0x160))
                // update new state (by updating state pointer)
                statePtr := add(dataPtr, 0x140)
                // empty transcript
                dataPtr := add(statePtr, 0x20)
                // (mod R_MOD) to get challenge
                mstore(add(res, 0xc0), mod(mload(statePtr), R_MOD))
            }

            let zetaPtr := mload(add(proof, 0x160))
            mstore(dataPtr, mload(zetaPtr)) // zeta.x
            mstore(add(dataPtr, 0x20), mload(add(zetaPtr, 0x20))) // zeta.y
            let zetaOmegaPtr := mload(add(proof, 0x180))
            mstore(add(dataPtr, 0x40), mload(zetaOmegaPtr)) // zetaOmega.x
            mstore(add(dataPtr, 0x60), mload(add(zetaOmegaPtr, 0x20))) // zetaOmega.y

            // challenge: u
            {
                // preimage len: state(0x20) + transcript(0x80)
                let hash := keccak256(statePtr, 0xa0)
                // (mod R_MOD) to get challenge
                mstore(add(res, 0xe0), mod(hash, R_MOD))
            }
        }
    }

    function verify(
        IPlonkVerifier.VerifyingKey memory vk,
        uint256[8] memory publicInput,
        IPlonkVerifier.PlonkProof memory proof
    ) external view returns (bool success) {
        _validateProof(proof);

        BN254.validateScalarField(BN254.ScalarField.wrap(publicInput[0]));
        BN254.validateScalarField(BN254.ScalarField.wrap(publicInput[1]));
        BN254.validateScalarField(BN254.ScalarField.wrap(publicInput[2]));
        BN254.validateScalarField(BN254.ScalarField.wrap(publicInput[3]));
        BN254.validateScalarField(BN254.ScalarField.wrap(publicInput[4]));
        BN254.validateScalarField(BN254.ScalarField.wrap(publicInput[5]));
        BN254.validateScalarField(BN254.ScalarField.wrap(publicInput[6]));
        BN254.validateScalarField(BN254.ScalarField.wrap(publicInput[7]));

        Challenges memory chal = _computeChallenges(vk, publicInput, proof);
        Poly.EvalDomain memory domain = Poly.newEvalDomain(vk.domainSize);
        Poly.EvalData memory evalData = Poly.evalDataGen(domain, chal.zeta, publicInput);

        uint256 linPolyConstant = _computeLinPolyConstantTerm(chal, proof, evalData);

        assembly {
            // credit: error function from Succinct's SP1 code
            function error_verify() {
                let ptError := mload(0x40)
                mstore(ptError, ERROR_STRING_ID) // selector for function Error(string)
                mstore(add(ptError, 0x4), 0x20)
                mstore(add(ptError, 0x24), 0xc)
                mstore(add(ptError, 0x44), "error verify")
                revert(ptError, 0x64)
            }
            function error_pairing() {
                let ptError := mload(0x40)
                mstore(ptError, ERROR_STRING_ID) // selector for function Error(string)
                mstore(add(ptError, 0x4), 0x20)
                mstore(add(ptError, 0x24), 0xd)
                mstore(add(ptError, 0x44), "error pairing")
                revert(ptError, 0x64)
            }
            // allocate 64 bytes in memory to store a G1 point
            function mallocG1() -> ptr {
                ptr := mload(0x40)
                mstore(0x40, add(ptr, 0x40))
            }

            // base: &uint256[2], scalar: uint256
            // result will be written to 0x00 (as there are 64 bytes of scratch space)
            function scalarMul(base, scalar) {
                let ptr := mload(0x40) // free memory pointer
                // layout:
                // 0x00: base.x
                // 0x20: base.y
                // 0x40: scalar
                mstore(ptr, mload(base))
                mstore(add(ptr, 0x20), mload(add(base, 0x20)))
                mstore(add(ptr, 0x40), scalar)
                let l_success := staticcall(gas(), 7, ptr, 0x60, 0x00, 0x40)
                if iszero(l_success) { error_verify() }
            }

            // self: &uint256[2], rhs: &uint256[2]
            // self += rhs (G1 point addition)
            function g1AddAssign(self, rhs) {
                let ptr := mload(0x40) // free memory pointer
                // layout:
                // 0x00: self.x
                // 0x20: self.y
                // 0x40: rhs.x
                // 0x60: rhs.y
                mstore(ptr, mload(self))
                mstore(add(ptr, 0x20), mload(add(self, 0x20)))
                mstore(add(ptr, 0x40), mload(rhs))
                mstore(add(ptr, 0x60), mload(add(rhs, 0x20)))
                let l_success := staticcall(gas(), 6, ptr, 0x80, self, 0x40)
                if iszero(l_success) { error_verify() }
            }

            /// @dev self: &uint256[2]
            /// self = -self
            function g1NegateAssign(self) {
                let y := mload(add(self, 0x20))
                mstore(add(self, 0x20), sub(P_MOD, y))
            }

            // layout for final pairing check (total memory size: 0x180):
            //     e(a, [x]_2) =?= e(b, [1]_2)
            // Definitions:
            // - x \in Fp2 = c0 + c1 * X
            // 0x00: a.x
            // 0x20: a.y
            // 0x40: [x]_2.x.c0
            // 0x60: [x]_2.x.c1
            // 0x80: [x]_2.y.c0
            // 0xa0: [x]_2.y.c1
            // 0xc0: b.x
            // 0xe0: b.y
            // 0x100: [1]_2.x.c1
            // 0x120: [1]_2.x.c0
            // 0x140: [1]_2.y.c1
            // 0x160: [1]_2.y.c0

            let aPtr := mload(0x40) // free memory ptr
            let bPtr := add(aPtr, 0xc0)
            // load beta_h (i.e. [x]_2) from SRS
            mstore(add(aPtr, 0x40), BETA_H_X0)
            mstore(add(aPtr, 0x60), BETA_H_X1)
            mstore(add(aPtr, 0x80), BETA_H_Y0)
            mstore(add(aPtr, 0xa0), BETA_H_Y1)
            // load BN254.P2 (i.e. [1]_2) from SRS
            mstore(add(aPtr, 0x100), H_X0)
            mstore(add(aPtr, 0x120), H_X1)
            mstore(add(aPtr, 0x140), H_Y0)
            mstore(add(aPtr, 0x160), H_Y1)
            // reserve space for inputs to the pairing check
            mstore(0x40, add(aPtr, 0x180))

            let base := 0x00
            let scalar := 0
            let tmp := 0
            let tmp2 := 0
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
            {
                // firstScalar = alpha^2 * L1(zeta)
                scalar := mulmod(mload(add(chal, 0x20)), mload(add(evalData, 0x20)), R_MOD)
                // rhs = alpha
                let rhs := mload(chal)
                // tmp = beta * zeta
                tmp := mulmod(mload(add(chal, 0x60)), mload(add(chal, 0xA0)), R_MOD)
                // =================================
                // k0 (which is 1) component
                // (beta * zeta + wireEval0 + gamma)
                // =================================
                tmp2 := addmod(tmp, mload(add(proof, 0x1A0)), R_MOD)
                tmp2 := addmod(tmp2, mload(add(chal, 0x80)), R_MOD)
                rhs := mulmod(tmp2, rhs, R_MOD)

                // =================================
                // k1 component
                // (beta * zeta * k1 + wireEval1 + gamma)
                // =================================
                tmp2 := mulmod(tmp, COSET_K1, R_MOD)
                tmp2 := addmod(tmp2, mload(add(proof, 0x1C0)), R_MOD)
                tmp2 := addmod(tmp2, mload(add(chal, 0x80)), R_MOD)
                rhs := mulmod(tmp2, rhs, R_MOD)

                // =================================
                // k2 component
                // (beta * zeta * k2 + wireEval2 + gamma)
                // =================================
                tmp2 := mulmod(tmp, COSET_K2, R_MOD)
                tmp2 := addmod(tmp2, mload(add(proof, 0x1E0)), R_MOD)
                tmp2 := addmod(tmp2, mload(add(chal, 0x80)), R_MOD)
                rhs := mulmod(tmp2, rhs, R_MOD)

                // =================================
                // k3 component
                // (beta * zeta * k3 + wireEval3 + gamma)
                // =================================
                tmp2 := mulmod(tmp, COSET_K3, R_MOD)
                tmp2 := addmod(tmp2, mload(add(proof, 0x200)), R_MOD)
                tmp2 := addmod(tmp2, mload(add(chal, 0x80)), R_MOD)
                rhs := mulmod(tmp2, rhs, R_MOD)

                // =================================
                // k4 component
                // (beta * zeta * k4 + wireEval4 + gamma)
                // =================================
                tmp2 := mulmod(tmp, COSET_K4, R_MOD)
                tmp2 := addmod(tmp2, mload(add(proof, 0x220)), R_MOD)
                tmp2 := addmod(tmp2, mload(add(chal, 0x80)), R_MOD)
                rhs := mulmod(tmp2, rhs, R_MOD)

                scalar := addmod(scalar, rhs, R_MOD) // firstScalar
            }
            base := mload(add(proof, 0xa0)) // proof.prodPerm
            scalarMul(base, scalar)
            // update G1 point `b` with result from scalarMul in scratch pad
            mstore(bPtr, mload(0x00))
            mstore(add(bPtr, 0x20), mload(0x20))

            // ============================================
            // Compute coefficient for the last wire sigma polynomial commitment.
            // secondScalar = - alpha * beta * z_w
            //              * (wireEval0 + gamma + beta * sigmaEval0)
            //              * (wireEval1 + gamma + beta * sigmaEval1)
            //              * ...
            // ============================================
            // secondScalar = alpha * beta * z_w
            scalar := mulmod(mload(chal), mload(add(chal, 0x60)), R_MOD)
            scalar := mulmod(scalar, mload(add(proof, 0x2C0)), R_MOD)

            // (wireEval0 + gamma + beta * sigmaEval0)
            tmp := mulmod(mload(add(chal, 0x60)), mload(add(proof, 0x240)), R_MOD)
            tmp := addmod(tmp, mload(add(proof, 0x1A0)), R_MOD)
            tmp := addmod(tmp, mload(add(chal, 0x80)), R_MOD)
            scalar := mulmod(scalar, tmp, R_MOD)

            // (wireEval1 + gamma + beta * sigmaEval1)
            tmp := mulmod(mload(add(chal, 0x60)), mload(add(proof, 0x260)), R_MOD)
            tmp := addmod(tmp, mload(add(proof, 0x1C0)), R_MOD)
            tmp := addmod(tmp, mload(add(chal, 0x80)), R_MOD)
            scalar := mulmod(scalar, tmp, R_MOD)

            // (wireEval2 + gamma + beta * sigmaEval2)
            tmp := mulmod(mload(add(chal, 0x60)), mload(add(proof, 0x280)), R_MOD)
            tmp := addmod(tmp, mload(add(proof, 0x1E0)), R_MOD)
            tmp := addmod(tmp, mload(add(chal, 0x80)), R_MOD)
            scalar := mulmod(scalar, tmp, R_MOD)

            // (wireEval3 + gamma + beta * sigmaEval3)
            tmp := mulmod(mload(add(chal, 0x60)), mload(add(proof, 0x2A0)), R_MOD)
            tmp := addmod(tmp, mload(add(proof, 0x200)), R_MOD)
            tmp := addmod(tmp, mload(add(chal, 0x80)), R_MOD)

            scalar := sub(R_MOD, mulmod(scalar, tmp, R_MOD)) // secondScalar
            base := mload(add(vk, 0xc0)) // vk.sigma4
            scalarMul(base, scalar) // base^scalar (res stored at 0x00)
            g1AddAssign(bPtr, 0x00) // b += base^scalar

            // ============
            // q_lc: linear combination selectors
            // ============
            scalar := mload(add(proof, 0x1a0)) // proof.wireEval0
            base := mload(add(vk, 0xe0)) // vk.q1
            scalarMul(base, scalar) // q1^wireEval0 (result stored at 0x00)
            g1AddAssign(bPtr, 0x00) // b += q1^wireEval0

            scalar := mload(add(proof, 0x1c0)) // proof.wireEval1
            base := mload(add(vk, 0x100)) // vk.q2
            scalarMul(base, scalar) // q2^wireEval1 (result stored at 0x00)
            g1AddAssign(bPtr, 0x00) // b += q2^wireEval1

            scalar := mload(add(proof, 0x1e0)) // proof.wireEval2
            base := mload(add(vk, 0x120)) // vk.q3
            scalarMul(base, scalar) // q3^wireEval2 (result stored at 0x00)
            g1AddAssign(bPtr, 0x00) // b += q3^wireEval2

            scalar := mload(add(proof, 0x200)) // proof.wireEval3
            base := mload(add(vk, 0x140)) // vk.q4
            scalarMul(base, scalar) // q4^wireEval3 (result stored at 0x00)
            g1AddAssign(bPtr, 0x00) // b += q4^wireEval3

            // ============
            // q_M: multiplication selectors
            // ============
            // w_evals[0] * w_evals[1]
            scalar := mulmod(mload(add(proof, 0x1a0)), mload(add(proof, 0x1c0)), R_MOD)
            base := mload(add(vk, 0x160)) // vk.qM12
            scalarMul(base, scalar) // qM12^(wireEval0 * wireEval1)
            g1AddAssign(bPtr, 0x00) // b += qM12^(wireEval0 * wireEval1)

            // w_evals[2] * w_evals[3]
            scalar := mulmod(mload(add(proof, 0x1e0)), mload(add(proof, 0x200)), R_MOD)
            base := mload(add(vk, 0x180)) // vk.qM34
            scalarMul(base, scalar) // qM34^(wireEval2 * wireEval3)
            g1AddAssign(bPtr, 0x00) // b += qM34^(wireEval2 * wireEval3)

            // ============
            // q_H: hash (rescue-friendly) selectors
            // ============
            // w_evals[0].pow(5);
            tmp := mload(add(proof, 0x1a0)) // proof.wireEval0
            tmp2 := mulmod(tmp, tmp, R_MOD)
            tmp2 := mulmod(tmp2, tmp2, R_MOD)
            scalar := mulmod(tmp, tmp2, R_MOD)
            base := mload(add(vk, 0x1e0)) // vk.qH1
            scalarMul(base, scalar) // qH1^(wireEval0.pow(5))
            g1AddAssign(bPtr, 0x00) // b += qH1^(wireEval0.pow(5))

            // w_evals[1].pow(5);
            tmp := mload(add(proof, 0x1c0)) // proof.wireEval1
            tmp2 := mulmod(tmp, tmp, R_MOD)
            tmp2 := mulmod(tmp2, tmp2, R_MOD)
            scalar := mulmod(tmp, tmp2, R_MOD)
            base := mload(add(vk, 0x200)) // vk.qH2
            scalarMul(base, scalar) // qH2^(wireEval2.pow(5))
            g1AddAssign(bPtr, 0x00) // b += qH2^(wireEval1.pow(5))

            // w_evals[2].pow(5);
            tmp := mload(add(proof, 0x1e0)) // proof.wireEval2
            tmp2 := mulmod(tmp, tmp, R_MOD)
            tmp2 := mulmod(tmp2, tmp2, R_MOD)
            scalar := mulmod(tmp, tmp2, R_MOD)
            base := mload(add(vk, 0x220)) // vk.qH3
            scalarMul(base, scalar) // qH3^(wireEval2.pow(5))
            g1AddAssign(bPtr, 0x00) // b += qH3^(wireEval2.pow(5))

            // w_evals[3].pow(5);
            tmp := mload(add(proof, 0x200)) // proof.wireEval3
            tmp2 := mulmod(tmp, tmp, R_MOD)
            tmp2 := mulmod(tmp2, tmp2, R_MOD)
            scalar := mulmod(tmp, tmp2, R_MOD)
            base := mload(add(vk, 0x240)) // vk.qH4
            scalarMul(base, scalar) // qH4^(wireEval3.pow(5))
            g1AddAssign(bPtr, 0x00) // b += qH4^(wireEval3.pow(5))

            // ============
            // q_o and q_c: output and constant selectors
            // ============
            scalar := sub(R_MOD, mload(add(proof, 0x220))) // - proof.wireEval4
            base := mload(add(vk, 0x1a0)) // vk.qO
            scalarMul(base, scalar) // qO^(-w_evals[4])
            g1AddAssign(bPtr, 0x00) // b += qO^(-w_evals[4])

            scalar := 1
            base := mload(add(vk, 0x1c0)) // vk.qC
            scalarMul(base, scalar) // qC
            g1AddAssign(bPtr, 0x00) // b += qC

            // ============
            // q_Ecc: Elliptic Curve Operation selector
            // ============
            // q_Ecc = w_evals[0] * w_evals[1] * w_evals[2] * w_evals[3] * w_evals[4];
            tmp := mulmod(mload(add(proof, 0x1a0)), mload(add(proof, 0x1c0)), R_MOD)
            tmp := mulmod(tmp, mload(add(proof, 0x1E0)), R_MOD)
            tmp := mulmod(tmp, mload(add(proof, 0x200)), R_MOD)
            scalar := mulmod(tmp, mload(add(proof, 0x220)), R_MOD)
            base := mload(add(vk, 0x260)) // vk.qEcc
            scalarMul(base, scalar) // qEcc^(\prod{w_evals})
            g1AddAssign(bPtr, 0x00) // b += qEcc^(\prod{w_evals})

            // ============================================
            // splitting quotient commitments
            // ============================================
            // 1 - zeta^n = - (zeta^n - 1)
            scalar := sub(R_MOD, mload(evalData)) // - vanishEval
            base := mload(add(proof, 0xc0)) // proof.split0
            scalarMul(base, scalar) // split0^(-vanishEval)
            g1AddAssign(bPtr, 0x00) // b += split0^(-vanishEval)

            // (1-zeta^n) * zeta^(n+2)
            tmp := addmod(mload(evalData), 1, R_MOD) // zeta^n
            tmp2 := mload(add(chal, 0xa0)) // zeta
            tmp2 := mulmod(tmp2, tmp2, R_MOD) // zeta^2
            tmp := mulmod(tmp, tmp2, R_MOD) // zeta^(n+2)
            scalar := mulmod(scalar, tmp, R_MOD)
            base := mload(add(proof, 0xe0)) // proof.split1
            scalarMul(base, scalar)
            g1AddAssign(bPtr, 0x00) // b += split1^[(1-zeta^n) * zeta^(n+2)]

            // (1-zeta^n) * zeta^2(n+2)
            scalar := mulmod(scalar, tmp, R_MOD)
            base := mload(add(proof, 0x100)) // proof.split2
            scalarMul(base, scalar)
            g1AddAssign(bPtr, 0x00) // b += split2^[(1-zeta^n) * zeta^2(n+2)]

            // (1-zeta^n) * zeta^3(n+2)
            scalar := mulmod(scalar, tmp, R_MOD)
            base := mload(add(proof, 0x120)) // proof.split3
            scalarMul(base, scalar)
            g1AddAssign(bPtr, 0x00) // b += split3^[(1-zeta^n) * zeta^3(n+2)]

            // (1-zeta^n) * zeta^4(n+2)
            scalar := mulmod(scalar, tmp, R_MOD)
            base := mload(add(proof, 0x140)) // proof.split4
            scalarMul(base, scalar)
            g1AddAssign(bPtr, 0x00) // b += split4^[(1-zeta^n) * zeta^4(n+2)]

            // ============================================
            // Add wire witness poly commitments
            // Meanwhile, _prepareEvaluation (`[E]1` in Sec 8.4, step 11 of Plonk)
            // ============================================
            let eval := sub(R_MOD, linPolyConstant) // - r0
            tmp := mload(add(chal, 0xc0)) // chal.v

            scalar := tmp // chal.v
            base := mload(proof) // proof.wire0
            scalarMul(base, scalar)
            g1AddAssign(bPtr, 0x00) // b += wire0^v
            // eval += v * proof.wireEval0
            eval := addmod(eval, mulmod(scalar, mload(add(proof, 0x1a0)), R_MOD), R_MOD)

            scalar := mulmod(scalar, tmp, R_MOD) // v^2
            base := mload(add(proof, 0x20)) // proof.wire1
            scalarMul(base, scalar)
            g1AddAssign(bPtr, 0x00) // b += wire1^(v^2)
            // eval += v^2 * proof.wireEval1
            eval := addmod(eval, mulmod(scalar, mload(add(proof, 0x1c0)), R_MOD), R_MOD)

            scalar := mulmod(scalar, tmp, R_MOD) // v^3
            base := mload(add(proof, 0x40)) // proof.wire2
            scalarMul(base, scalar)
            g1AddAssign(bPtr, 0x00) // b += wire2^(v^3)
            // eval += v^3 * proof.wireEval2
            eval := addmod(eval, mulmod(scalar, mload(add(proof, 0x1e0)), R_MOD), R_MOD)

            scalar := mulmod(scalar, tmp, R_MOD) // v^4
            base := mload(add(proof, 0x60)) // proof.wire3
            scalarMul(base, scalar)
            g1AddAssign(bPtr, 0x00) // b += wire3^(v^4)
            // eval += v^4 * proof.wireEval3
            eval := addmod(eval, mulmod(scalar, mload(add(proof, 0x200)), R_MOD), R_MOD)

            scalar := mulmod(scalar, tmp, R_MOD) // v^5
            base := mload(add(proof, 0x80)) // proof.wire4
            scalarMul(base, scalar)
            g1AddAssign(bPtr, 0x00) // b += wire4^(v^5)
            // eval += v^5 * proof.wireEval4
            eval := addmod(eval, mulmod(scalar, mload(add(proof, 0x220)), R_MOD), R_MOD)

            // ============================================
            // Add sigma poly commitments
            // The last sigma commitment is excluded.
            // ============================================
            scalar := mulmod(scalar, tmp, R_MOD) // v^6
            base := mload(add(vk, 0x40)) // vk.sigma0
            scalarMul(base, scalar)
            g1AddAssign(bPtr, 0x00) // b += sigma0^(v^6)
            // eval += v^6 * proof.sigmaEval0
            eval := addmod(eval, mulmod(scalar, mload(add(proof, 0x240)), R_MOD), R_MOD)

            scalar := mulmod(scalar, tmp, R_MOD) // v^7
            base := mload(add(vk, 0x60)) // vk.sigma1
            scalarMul(base, scalar)
            g1AddAssign(bPtr, 0x00) // b += sigma1^(v^7)
            // eval += v^7 * proof.sigmaEval1
            eval := addmod(eval, mulmod(scalar, mload(add(proof, 0x260)), R_MOD), R_MOD)

            scalar := mulmod(scalar, tmp, R_MOD) // v^8
            base := mload(add(vk, 0x80)) // vk.sigma2
            scalarMul(base, scalar)
            g1AddAssign(bPtr, 0x00) // b += sigma2^(v^8)
            // eval += v^8 * proof.sigmaEval2
            eval := addmod(eval, mulmod(scalar, mload(add(proof, 0x280)), R_MOD), R_MOD)

            scalar := mulmod(scalar, tmp, R_MOD) // v^9
            base := mload(add(vk, 0xa0)) // vk.sigma3
            scalarMul(base, scalar)
            g1AddAssign(bPtr, 0x00) // b += sigma3^(v^9)
            // eval += v^9 * proof.sigmaEval3
            eval := addmod(eval, mulmod(scalar, mload(add(proof, 0x2a0)), R_MOD), R_MOD)

            // ============================================
            // Add poly comm evaluated at point `zeta * g`
            // ============================================
            scalar := mload(add(chal, 0xe0)) // chal.u
            base := mload(add(proof, 0xa0)) // proof.prodPerm
            scalarMul(base, scalar)
            g1AddAssign(bPtr, 0x00) // b += prodPerm^u
            // eval += u * proof.prodPermZetaOmegaEval
            eval := addmod(eval, mulmod(scalar, mload(add(proof, 0x2c0)), R_MOD), R_MOD)

            scalar := mload(add(chal, 0xa0)) // chal.zeta or evalPoint
            base := mload(add(proof, 0x160)) // proof.zeta or openingProof
            scalarMul(base, scalar)
            g1AddAssign(bPtr, 0x00) // b += proof.zeta^chal.zeta

            // chal.zeta * groupGen or nextEvalPoint
            tmp := mulmod(scalar, mload(add(mload(add(domain, 0x40)), 0x20)), R_MOD)
            scalar := mulmod(tmp, mload(add(chal, 0xe0)), R_MOD) // u * nextEvalPoint
            base := mload(add(proof, 0x180)) // shiftedOpeningProof or proof.zetaOmega
            scalarMul(base, scalar)
            g1AddAssign(bPtr, 0x00) // b += proof.zetaOmega^(u * chal.zeta * groupGen)

            base := mallocG1()
            mstore(base, 1) // BN254.P1.x
            mstore(add(base, 0x20), 2) // BN254.P1.y
            scalarMul(base, sub(R_MOD, eval)) // P1^-eval or [E]1 in paper
            g1AddAssign(bPtr, 0x00) // b += P1^-eval

            // b = -b
            g1NegateAssign(bPtr)

            // a = openingProof + shiftedOpeningProof^u
            // first store openingProof to aPtr
            tmp := mload(add(proof, 0x160)) // ptr to proof.zeta
            mstore(aPtr, mload(tmp))
            mstore(add(aPtr, 0x20), mload(add(tmp, 0x20)))

            scalar := mload(add(chal, 0xe0)) // chal.u
            base := mload(add(proof, 0x180)) // shiftedOpeningProof or proof.zetaOmega
            scalarMul(base, scalar)
            g1AddAssign(aPtr, 0x00) // a += shiftedOpeningProof^u

            // ============================================
            // Final pairing check
            // ============================================
            let l_success := staticcall(gas(), 8, aPtr, 0x180, 0x00, 0x20)
            if iszero(l_success) { error_pairing() }

            success := mload(0x00)
        }
    }
}
