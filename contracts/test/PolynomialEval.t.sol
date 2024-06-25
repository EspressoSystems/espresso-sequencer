// SPDX-License-Identifier: Unlicensed

/* solhint-disable contract-name-camelcase, func-name-mixedcase, one-contract-per-file */

pragma solidity ^0.8.0;

// Libraries
import "forge-std/Test.sol";
import { BN254 } from "bn254/BN254.sol";

// Target contract
import { PolynomialEval as Poly } from "../src/libraries/PolynomialEval.sol";

contract PolynomialEval_newEvalDomain_Test is Test {
    /// @dev diff-test with Rust when `domainSize` is in {2^16 ~ 2^20, 2^5}
    function test_supportedDomainSize_matches() external {
        uint256[6] memory logSizes = [uint256(5), 16, 17, 18, 19, 20];
        for (uint256 i = 0; i < 6; i++) {
            string[] memory cmds = new string[](3);
            cmds[0] = "diff-test";
            cmds[1] = "new-poly-eval-domain";
            cmds[2] = vm.toString(logSizes[i]);

            bytes memory result = vm.ffi(cmds);
            (uint256 sizeInv, uint256 groupGen, uint256 groupGenInv) =
                abi.decode(result, (uint256, uint256, uint256));

            Poly.EvalDomain memory domain = Poly.newEvalDomain(2 ** logSizes[i]);
            assertEq(sizeInv, domain.sizeInv);
            assertEq(groupGen, domain.groupGen);
            assertEq(groupGenInv, domain.groupGenInv);
        }
    }

    /// @dev Test revert if domainSize is not among {2^16 ~ 2^20, 2^5}
    function testFuzz_unsupportedDomainSize_reverts(uint256 domainSize) external {
        vm.assume(
            domainSize != 2 ** 16 && domainSize != 2 ** 17 && domainSize != 2 ** 18
                && domainSize != 2 ** 19 && domainSize != 2 ** 20 && domainSize != 2 ** 5
        );

        vm.expectRevert(Poly.UnsupportedDegree.selector);
        Poly.newEvalDomain(domainSize);
    }
}

contract PolynomialEval_domainElements_Test is Test {
    /// @dev Test if the domain elements are generated correctly
    function testFuzz_domainElements_matches(uint256 logSize, uint256 length) external {
        logSize = bound(logSize, 16, 20);
        length = bound(length, 0, 10000);
        Poly.EvalDomain memory domain = Poly.newEvalDomain(2 ** logSize);

        if (length > domain.size) {
            vm.expectRevert(Poly.InvalidPolyEvalArgs.selector);
            Poly.domainElements(domain, length);
        } else {
            string[] memory cmds = new string[](4);
            cmds[0] = "diff-test";
            cmds[1] = "eval-domain-elements";
            cmds[2] = vm.toString(logSize);
            cmds[3] = vm.toString(length);

            bytes memory result = vm.ffi(cmds);
            (uint256[] memory elems) = abi.decode(result, (uint256[]));

            assertEq(elems, Poly.domainElements(domain, length));
        }
    }
}

contract PolynomialEval_evalDataGen_Test is Test {
    /// @dev Test if evaluations on the vanishing poly, the lagrange one poly, and the public input
    /// poly are correct.
    function testFuzz_evalDataGen_matches(
        uint256 logSize,
        uint256 zeta,
        uint256[] memory publicInput
    ) external {
        logSize = bound(logSize, 16, 20);
        zeta = bound(zeta, 0, BN254.R_MOD - 1);
        BN254.validateScalarField(BN254.ScalarField.wrap(zeta));
        // Since these user-provided `publicInputs` were checked outside before passing in via
        // `BN254.validateScalarField()`, it suffices to assume they are proper for our test here.
        for (uint256 i = 0; i < publicInput.length; i++) {
            publicInput[i] = bound(publicInput[i], 0, BN254.R_MOD - 1);
            BN254.validateScalarField(BN254.ScalarField.wrap(publicInput[i]));
        }

        Poly.EvalDomain memory domain = Poly.newEvalDomain(2 ** logSize);
        // NOTE: since zeta comes from CRH via `computeChallenges`, we can assume it's hard to
        // be specifically preimage attacked, or accidentally collide with these values.
        vm.assume(zeta != 1); // otherwise divisor of lagrange_1_poly would be zero
        vm.assume(zeta != domain.groupGenInv); // otherwise divisor of lagrange_n_poly would be zero

        string[] memory cmds = new string[](5);
        cmds[0] = "diff-test";
        cmds[1] = "eval-data-gen";
        cmds[2] = vm.toString(logSize);
        cmds[3] = vm.toString(bytes32(zeta));
        cmds[4] = vm.toString(abi.encode(publicInput));

        bytes memory result = vm.ffi(cmds);
        (uint256 vanishEval, uint256 lagrangeOne, uint256 piEval) =
            abi.decode(result, (uint256, uint256, uint256));

        Poly.EvalData memory evalData = Poly.evalDataGen(domain, zeta, publicInput);
        assertEq(vanishEval, BN254.ScalarField.unwrap(evalData.vanishEval));
        assertEq(lagrangeOne, BN254.ScalarField.unwrap(evalData.lagrangeOne));
        assertEq(piEval, BN254.ScalarField.unwrap(evalData.piEval));
    }

    /// @dev Test edge cases when zeta is one of the elements in the evaluation domain.
    /// The random evaluation point case (most likely outside evalDomain) is already
    /// tested in `testFuzz_evalDataGen_matches()`
    function test_lagrangeOneCoeffForDomainElements() external view {
        uint256 size = 2 ** 5;
        Poly.EvalDomain memory domain = Poly.newEvalDomain(size);

        uint256[] memory elements = Poly.domainElements(domain, size);
        uint256 vanishEval = Poly.evaluateVanishingPoly(domain, elements[0]);

        // L_0(g^0) = 1
        assertEq(
            BN254.ScalarField.unwrap(
                Poly.evaluateLagrangeOne(
                    domain, BN254.ScalarField.wrap(elements[0]), BN254.ScalarField.wrap(vanishEval)
                )
            ),
            1
        );

        // L_i(g^0) = 0 for i \in [size]
        for (uint256 i = 1; i < size; i++) {
            vanishEval = Poly.evaluateVanishingPoly(domain, elements[i]);
            assertEq(
                BN254.ScalarField.unwrap(
                    Poly.evaluateLagrangeOne(
                        domain,
                        BN254.ScalarField.wrap(elements[i]),
                        BN254.ScalarField.wrap(vanishEval)
                    )
                ),
                0
            );
        }
    }
}
