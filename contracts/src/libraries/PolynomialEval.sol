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
        if (BN254.ScalarField.unwrap(vanishEval) == 0) {
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

    /// @dev Compute prefix and suffix product arrays for the given local domain elements.
    function _computePrefixAndSuffixProduct(
        uint256[] memory localDomainElements,
        uint256 zeta,
        uint256 p
    ) internal pure returns (uint256[] memory prefix, uint256[] memory suffix) {
        // Assume we have [a, b, c, d] where a = zeta - g^0, b = zeta - g^1, ...

        // prefix[i] = prefix[i - 1] * (zeta - g^(i - 1)) and prefix[0] = 1
        // prefix = [1, a, ab, abc]

        // suffix[length - i - 1] = suffix[length - i] * (zeta - g^(length - i)) and
        // suffix[length - 1] = 1
        // suffix = [dcb, dc, d, 1]

        // fullProduct = abcd can be computed by suffix[0] * prefix[1]

        uint256 length = localDomainElements.length;

        prefix = new uint256[](length);
        suffix = new uint256[](length);

        assembly {
            let prefixPtr := add(prefix, 0x20)
            let suffixPtr := add(suffix, mul(length, 0x20))
            let localDomainElementsPrefixPtr := add(localDomainElements, 0x20)
            let localDomainElementsSuffixPtr := add(localDomainElements, mul(length, 0x20))

            let currentElementPrefix := 1
            let currentElementSuffix := 1

            // First element of prefix is set to 1
            mstore(prefixPtr, currentElementPrefix)
            // Last element of suffix is set to 1
            mstore(suffixPtr, currentElementSuffix)

            // Calculate prefix and suffix products
            for { let i := 1 } lt(i, length) { i := add(i, 1) } {
                // move prefix pointer
                prefixPtr := add(prefixPtr, 0x20)
                // move suffix pointer
                suffixPtr := sub(suffixPtr, 0x20)

                // prefix[i] = prefix[i - 1] * (zeta - g^(i - 1))
                currentElementPrefix :=
                    mulmod(
                        currentElementPrefix,
                        addmod(sub(p, mload(localDomainElementsPrefixPtr)), zeta, p),
                        p
                    )
                mstore(prefixPtr, currentElementPrefix)

                // suffix[length - i - 1] = suffix[length - i] * (zeta - g^(length - i))
                currentElementSuffix :=
                    mulmod(
                        currentElementSuffix,
                        addmod(sub(p, mload(localDomainElementsSuffixPtr)), zeta, p),
                        p
                    )
                mstore(suffixPtr, currentElementSuffix)

                // move localDomainElements pointers
                localDomainElementsPrefixPtr := add(localDomainElementsPrefixPtr, 0x20)
                localDomainElementsSuffixPtr := sub(localDomainElementsSuffixPtr, 0x20)
            }
        }
    }

    /// @dev Evaluate public input polynomial at point `zeta`.
    function evaluatePiPoly(
        EvalDomain memory self,
        uint256[] memory pi,
        uint256 zeta,
        uint256 vanishingPolyEval
    ) internal view returns (uint256 res) {
        if (vanishingPolyEval == 0) {
            return 0;
        }

        uint256[] memory localDomainElements = domainElements(self, pi.length);

        uint256 length = pi.length;
        uint256 p = BN254.R_MOD;

        // In order to compute PiPoly(zeta) in an efficient way, we can do the following derivation:

        // PiPoly(zeta) = \sum_{i=0}^{length} pi[i] * L_i(zeta) where
        // L_i(zeta) = (Z_H(zeta) * g^i) / (n * (zeta - g^i))
        // PiPoly(zeta) = (Z_H(zeta) / n) * \sum_{i=0}^{length} pi[i] * g^i * (\prod_{i neq j}
        // (zeta - g^j)) / (\prod_{j=0}^{length} (zeta - g^j))

        // Since the denominator (\prod_{j=0}^{length} (zeta - g^j)) is the total product and
        // doesn't depend on i, we can take it out of the sum and compute it once.

        // PiPoly(zeta) = vanishingPolyEval / (n * fullProduct) * \sum_{i=0}^{length} pi[i] * g^i *
        // (\prod_{i != j} (zeta - g^j))

        // where fullProduct = \prod_{j=0}^{length} (zeta - g^j) or suffix[0] * prefix[1]

        // Another optimization we can do is instead of computing the product where i != j, we can
        // precompute the prefix and suffix products and just compute prefix[i] * suffix[i] to get
        // the product (\prod_{i != j} (zeta - g^j)).

        // compute prefix and suffix product arrays as described in the function
        // _computePrefixAndSuffixProduct
        // this helps optimize the PiPoly computation by using the following formula:
        // PiPoly(zeta) = vanishingPolyEval / (n * fullProduct) * \sum_{i=0}^{length}
        // (prefix[i] * suffix[i] * pi[i] * g^i)

        // Compute prefix and suffix products
        // This optimization keeps the 1 inversion but reduces the number of multiplications from
        // n(n - 1) to 3n
        (uint256[] memory prefix, uint256[] memory suffix) =
            _computePrefixAndSuffixProduct(localDomainElements, zeta, p);

        // 1 / n
        uint256 nInverted = self.sizeInv;

        // if length is 1, then fullProduct = zeta - 1
        uint256 fullProduct = zeta == 0 ? p - 1 : zeta - 1;

        if (length > 1) {
            assembly {
                // multiply suffix[0] and prefix[1] to obtain the fullProduct
                fullProduct := mulmod(mload(add(suffix, 0x20)), mload(add(prefix, 0x40)), p)
            }
        }

        uint256 invertedProduct =
            BN254.ScalarField.unwrap(BN254.invert(BN254.ScalarField.wrap(fullProduct)));

        assembly {
            let sum := 0
            let prefixPtr := add(prefix, 0x20)
            let suffixPtr := add(suffix, 0x20)
            let piPtr := add(pi, 0x20)
            let localDomainElementsPtr := add(localDomainElements, 0x20)
            // Compute the sum term \sum_{i=0}^{length} prefix[i] * suffix[i] * pi[i] * g^i
            for { let i := 0 } lt(i, length) { i := add(i, 1) } {
                // sum += prefix[i] * suffix[i] * pi[i] * g^i
                let currentTerm :=
                    mulmod(
                        mulmod(mulmod(mload(prefixPtr), mload(suffixPtr), p), mload(piPtr), p),
                        mload(localDomainElementsPtr),
                        p
                    )
                sum := addmod(sum, currentTerm, p)

                // move the pointers
                prefixPtr := add(prefixPtr, 0x20)
                suffixPtr := add(suffixPtr, 0x20)
                piPtr := add(piPtr, 0x20)
                localDomainElementsPtr := add(localDomainElementsPtr, 0x20)
            }

            // Final computation
            // vanishingPolyEval / ( n * fullProduct ) * sum
            res := mulmod(vanishingPolyEval, nInverted, p)
            res := mulmod(res, invertedProduct, p)
            res := mulmod(res, sum, p)
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
    function evalDataGen(EvalDomain memory self, uint256 zeta, uint256[] memory publicInput)
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
