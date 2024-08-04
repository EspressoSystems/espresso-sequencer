// SPDX-License-Identifier: Unlicensed

pragma solidity ^0.8.0;

import { BN254 } from "bn254/BN254.sol";

/* solhint-disable no-inline-assembly */

library PolynomialEval {
    /// Unsupported polynomial degree, currently size must in 2^{14~17}.
    error UnsupportedDegree();
    /// Unexpected input arguments, some precondition assumptions violated.
    error InvalidPolyEvalArgs();

    /// @dev a Radix 2 Evaluation Domain
    struct EvalDomain {
        uint256 logSize; // log_2(self.size)
        uint256 size; // Size of the domain as a field element
        uint256 sizeInv; // Inverse of the size in the field
        uint256 groupGen; // A generator of the subgroup
        uint256 groupGenInv; // Inverse of the generator of the subgroup
    }

    /// @dev stores vanishing poly, lagrange at 1, and Public input poly
    struct EvalData {
        BN254.ScalarField vanishEval;
        BN254.ScalarField lagrangeOne;
        BN254.ScalarField piEval;
    }

    /// @dev Create a new Radix2EvalDomain with `domainSize` which should be power of 2.
    /// @dev Will revert if domainSize is not among {2^5, 2^16, 2^17, 2^18, 2^19, 2^20}
    function newEvalDomain(uint256 domainSize) internal pure returns (EvalDomain memory) {
        if (domainSize == 65536) {
            return EvalDomain(
                16,
                domainSize,
                0x30641e0e92bebef818268d663bcad6dbcfd6c0149170f6d7d350b1b1fa6c1001,
                0x00eeb2cb5981ed45649abebde081dcff16c8601de4347e7dd1628ba2daac43b7,
                0x0b5d56b77fe704e8e92338c0082f37e091126414c830e4c6922d5ac802d842d4
            );
        } else if (domainSize == 131072) {
            return EvalDomain(
                17,
                domainSize,
                0x30643640b9f82f90e83b698e5ea6179c7c05542e859533b48b9953a2f5360801,
                0x1bf82deba7d74902c3708cc6e70e61f30512eca95655210e276e5858ce8f58e5,
                0x244cf010c43ca87237d8b00bf9dd50c4c01c7f086bd4e8c920e75251d96f0d22
            );
        } else if (domainSize == 262144) {
            return EvalDomain(
                18,
                domainSize,
                0x30644259cd94e7dd5045d7a27013b7fcd21c9e3b7fa75222e7bda49b729b0401,
                0x19ddbcaf3a8d46c15c0176fbb5b95e4dc57088ff13f4d1bd84c6bfa57dcdc0e0,
                0x36853f083780e87f8d7c71d111119c57dbe118c22d5ad707a82317466c5174c
            );
        } else if (domainSize == 524288) {
            return EvalDomain(
                19,
                domainSize,
                0x3064486657634403844b0eac78ca882cfd284341fcb0615a15cfcd17b14d8201,
                0x2260e724844bca5251829353968e4915305258418357473a5c1d597f613f6cbd,
                0x6e402c0a314fb67a15cf806664ae1b722dbc0efe66e6c81d98f9924ca535321
            );
        } else if (domainSize == 1048576) {
            return EvalDomain(
                20,
                domainSize,
                0x30644b6c9c4a72169e4daa317d25f04512ae15c53b34e8f5acd8e155d0a6c101,
                0x26125da10a0ed06327508aba06d1e303ac616632dbed349f53422da953337857,
                0x100c332d2100895fab6473bc2c51bfca521f45cb3baca6260852a8fde26c91f3
            );
        }
        if (domainSize == 32) {
            // useful for small-size test, in practice unlikely to be used.
            return EvalDomain(
                5,
                domainSize,
                0x2ee12bff4a2813286a8dc388cd754d9a3ef2490635eba50cb9c2e5e750800001,
                0x9c532c6306b93d29678200d47c0b2a99c18d51b838eeb1d3eed4c533bb512d0,
                0x2724713603bfbd790aeaf3e7df25d8e7ef8f311334905b4d8c99980cf210979d
            );
        } else {
            revert UnsupportedDegree();
        }
    }

    // This evaluates the vanishing polynomial for this domain at zeta.
    // For multiplicative subgroups, this polynomial is
    // `z(X) = X^self.size - 1`.
    function evaluateVanishingPoly(EvalDomain memory self, uint256 zeta)
        internal
        pure
        returns (uint256 res)
    {
        uint256 p = BN254.R_MOD;
        uint256 logSize = self.logSize;

        assembly {
            switch zeta
            case 0 { res := sub(p, 1) }
            default {
                res := zeta
                for { let i := 0 } lt(i, logSize) { i := add(i, 1) } { res := mulmod(res, res, p) }
                // since zeta != 0 we know that res is not 0
                // so we can safely do a subtraction
                res := sub(res, 1)
            }
        }
    }

    /// @dev Evaluate the lagrange polynomial at point `zeta` given the vanishing polynomial
    /// evaluation `vanish_eval`.
    function evaluateLagrangeOne(
        EvalDomain memory self,
        BN254.ScalarField zeta,
        BN254.ScalarField vanishEval
    ) internal view returns (BN254.ScalarField res) {
        if (BN254.ScalarField.unwrap(zeta) == 1) {
            // when zeta is first element in the eval domain
            return BN254.ScalarField.wrap(1);
        }
        if (BN254.ScalarField.unwrap(vanishEval) == 0) {
            // else, if zeta is other elements in the eval domain
            return BN254.ScalarField.wrap(0);
        }

        uint256 p = BN254.R_MOD;
        uint256 divisor;
        uint256 vanishEvalMulSizeInv = self.sizeInv;

        // =========================
        // lagrange_1_eval = vanish_eval / self.size / (zeta - 1)
        // =========================
        assembly {
            vanishEvalMulSizeInv := mulmod(vanishEval, vanishEvalMulSizeInv, p)

            switch zeta
            case 0 { divisor := sub(p, 1) }
            default { divisor := sub(zeta, 1) }
        }
        divisor = BN254.ScalarField.unwrap((BN254.invert(BN254.ScalarField.wrap(divisor))));
        assembly {
            res := mulmod(vanishEvalMulSizeInv, divisor, p)
        }
    }

    /// @dev Evaluate public input polynomial at point `zeta`.
    function evaluatePiPoly(
        EvalDomain memory self,
        uint256[8] memory pi,
        uint256 zeta,
        uint256 vanishEval
    ) internal view returns (uint256 res) {
        uint256 p = BN254.R_MOD;

        // TODO Philippe if we don't do this the test fails...
        uint256[] memory piDyn = new uint256[](8);
        for (uint256 i = 0; i < 8; i++) {
            piDyn[i] = pi[i];
        }

        uint256 length = piDyn.length;

        // when zeta is one of the eval domain, vanishEval = 0
        // lagrange coeffs for all but the one corresponding to zeta are zero
        // NOTE: since this happens with negligible probability, we avoid writing in assembly
        if (vanishEval == 0) {
            uint256 group = 1;
            for (uint256 i = 0; i < length; i++) {
                if (zeta == group) {
                    return piDyn[i];
                }
                group = mulmod(group, self.groupGen, p);
            }
            return 0;
        }

        uint256 ithLagrange;
        uint256 ithDivisor;
        uint256 tmp;
        uint256 vanishEvalDivN = self.sizeInv;
        uint256 divisorProd;
        uint256[] memory localDomainElements = domainElements(self, length);
        uint256[] memory divisors = new uint256[](length);

        assembly {
            // vanish_eval_div_n = (zeta^n-1)/n
            vanishEvalDivN := mulmod(vanishEvalDivN, vanishEval, p)

            // Now we need to compute
            //  \sum_{i=0..l} L_{i,H}(zeta) * pub_input[i]
            // where
            // - L_{i,H}(zeta)
            //      = Z_H(zeta) * v_i / (zeta - g^i)
            //      = vanish_eval_div_n * g^i / (zeta - g^i)
            // - v_i = g^i / n
            //
            // we want to use batch inversion method where we compute
            //
            //      divisorProd = 1 / \prod (zeta - g^i)
            //
            // and then each 1 / (zeta - g^i) can be computed via (length - 1)
            // multiplications:
            //
            //      1 / (zeta - g^i) = divisorProd * \prod_{j!=i} (zeta - g^j)
            //
            // In total this takes n(n-1) multiplications and 1 inversion,
            // instead of doing n inversions.
            divisorProd := 1

            for { let i := 0 } lt(i, length) { i := add(i, 1) } {
                // tmp points to g^i
                // first 32 bytes of reference is the length of an array
                tmp := mload(add(add(localDomainElements, 0x20), mul(i, 0x20)))
                // compute (zeta - g^i)
                ithDivisor := addmod(sub(p, tmp), zeta, p)
                // accumulate (zeta - g^i) to the divisorProd
                divisorProd := mulmod(divisorProd, ithDivisor, p)
                // store ithDivisor in the array
                mstore(add(add(divisors, 0x20), mul(i, 0x20)), ithDivisor)
            }
        }

        // compute 1 / \prod_{i=0}^length (zeta - g^i)
        divisorProd = BN254.ScalarField.unwrap(BN254.invert(BN254.ScalarField.wrap(divisorProd)));

        assembly {
            for { let i := 0 } lt(i, length) { i := add(i, 1) } {
                // tmp points to g^i
                // first 32 bytes of reference is the length of an array
                tmp := mload(add(add(localDomainElements, 0x20), mul(i, 0x20)))
                // vanish_eval_div_n * g^i
                ithLagrange := mulmod(vanishEvalDivN, tmp, p)

                // now we compute vanish_eval_div_n * g^i / (zeta - g^i) via
                // vanish_eval_div_n * g^i * divisorProd * \prod_{j!=i} (zeta - g^j)
                ithLagrange := mulmod(ithLagrange, divisorProd, p)
                for { let j := 0 } lt(j, length) { j := add(j, 1) } {
                    if iszero(eq(i, j)) {
                        ithDivisor := mload(add(add(divisors, 0x20), mul(j, 0x20)))
                        ithLagrange := mulmod(ithLagrange, ithDivisor, p)
                    }
                }

                // multiply by pub_input[i] and update res
                // tmp points to public input
                tmp := mload(add(add(piDyn, 0x20), mul(i, 0x20)))
                ithLagrange := mulmod(ithLagrange, tmp, p)
                res := addmod(res, ithLagrange, p)
            }
        }
    }

    /// @dev Generate the domain elements for indexes 0..length
    /// which are essentially g^0, g^1, ..., g^{length-1}
    function domainElements(EvalDomain memory self, uint256 length)
        internal
        pure
        returns (uint256[] memory elements)
    {
        if (length > self.size) revert InvalidPolyEvalArgs();
        uint256 groupGen = self.groupGen;
        uint256 tmp = 1;
        uint256 p = BN254.R_MOD;
        elements = new uint256[](length);
        assembly {
            if not(iszero(length)) {
                let ptr := add(elements, 0x20)
                let end := add(ptr, mul(0x20, length))
                mstore(ptr, 1)
                ptr := add(ptr, 0x20)
                // for (; ptr < end; ptr += 32) loop through the memory of `elements`
                for { } lt(ptr, end) { ptr := add(ptr, 0x20) } {
                    tmp := mulmod(tmp, groupGen, p)
                    mstore(ptr, tmp)
                }
            }
        }
    }

    /// @dev compute the EvalData for a given domain and a challenge zeta
    function evalDataGen(EvalDomain memory self, uint256 zeta, uint256[8] memory publicInput)
        internal
        view
        returns (EvalData memory evalData)
    {
        evalData.vanishEval = BN254.ScalarField.wrap(evaluateVanishingPoly(self, zeta));
        evalData.lagrangeOne =
            evaluateLagrangeOne(self, BN254.ScalarField.wrap(zeta), evalData.vanishEval);
        evalData.piEval = BN254.ScalarField.wrap(
            evaluatePiPoly(self, publicInput, zeta, BN254.ScalarField.unwrap(evalData.vanishEval))
        );
    }
}
