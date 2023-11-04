// SPDX-License-Identifier: Unlicensed

/* solhint-disable contract-name-camelcase, func-name-mixedcase, one-contract-per-file */

pragma solidity ^0.8.0;

// Libraries
import "forge-std/Test.sol";
import { BN254 } from "bn254/BN254.sol";

// Target contract
import { PolynomialEval as Poly } from "../src/libraries/PolynomialEval.sol";

contract PolynomialEval_newEvalDomain_Test is Test {
    /// @dev diff-test with Rust when `domainSize` is in {2^14, 2^15, 2^16, 2^17}
    function test_supportedDomainSize_matches() external {
        for (uint256 logSize = 14; logSize < 18; logSize++) {
            string[] memory cmds = new string[](6);
            cmds[0] = "cargo";
            cmds[1] = "run";
            cmds[2] = "--bin";
            cmds[3] = "diff-test";
            cmds[4] = "new-poly-eval-domain";
            cmds[5] = vm.toString(logSize);

            bytes memory result = vm.ffi(cmds);
            (uint256 sizeInv, uint256 groupGen, uint256 groupGenInv) =
                abi.decode(result, (uint256, uint256, uint256));

            Poly.EvalDomain memory domain = Poly.newEvalDomain(2 ** logSize);
            assertEq(sizeInv, domain.sizeInv);
            assertEq(groupGen, domain.groupGen);
            assertEq(groupGenInv, domain.groupGenInv);
        }
    }

    /// forge-config: default.fuzz.runs = 20
    /// @dev Test revert if domainSize is not among {2^14, 2^15, 2^16, 2^17}
    function testFuzz_unsupportedDomainSize_reverts(uint256 domainSize) external {
        vm.assume(
            domainSize != 2 ** 14 && domainSize != 2 ** 15 && domainSize != 2 ** 16
                && domainSize != 2 ** 17
        );

        vm.expectRevert(Poly.UnsupportedDegree.selector);
        Poly.newEvalDomain(domainSize);
    }
}

contract PolynomialEval_domainElements_Test is Test {
    /// @dev Test if the domain elements are generated correctly
    /// forge-config: default.fuzz.runs = 25
    function testFuzz_domainElements_matches(uint256 logSize, uint256 length) external {
        logSize = bound(logSize, 14, 17);
        Poly.EvalDomain memory domain = Poly.newEvalDomain(2 ** logSize);

        if (length > domain.size || length == 0) {
            vm.expectRevert(Poly.InvalidPolyEvalArgs.selector);
            Poly.domainElements(domain, length);
        } else {
            string[] memory cmds = new string[](7);
            cmds[0] = "cargo";
            cmds[1] = "run";
            cmds[2] = "--bin";
            cmds[3] = "diff-test";
            cmds[4] = "eval-domain-elements";
            cmds[5] = vm.toString(logSize);
            cmds[6] = vm.toString(length);

            bytes memory result = vm.ffi(cmds);
            (uint256[] memory elems) = abi.decode(result, (uint256[]));

            assertEq(elems, Poly.domainElements(domain, length));
        }
    }
}

contract PolynomialEval_evalDataGen_Test is Test {
    /// @dev Test if evaluations on the vanishing poly, the lagrange one poly, and the public input
    /// poly are correct.
    /// forge-config: default.fuzz.runs = 10
    function testFuzz_evalDataGen_matches(
        uint256 logSize,
        uint256 zeta,
        uint256[] memory publicInput
    ) external {
        logSize = bound(logSize, 14, 17);
        zeta = bound(zeta, 0, BN254.R_MOD - 1);
        vm.assume(zeta != 0); // otherwise divisor of lagrange_one_poly would be zero
        BN254.validateScalarField(zeta);
        vm.assume(publicInput.length > 0);
        // Since these user-provided `publicInputs` were checked outside before passing in via
        // `BN254.validateScalarField()`, it suffices to assume they are proper for our test here.
        for (uint256 i = 0; i < publicInput.length; i++) {
            publicInput[i] = bound(publicInput[i], 0, BN254.R_MOD - 1);
            BN254.validateScalarField(publicInput[i]);
        }

        Poly.EvalDomain memory domain = Poly.newEvalDomain(2 ** logSize);

        string[] memory cmds = new string[](8);
        cmds[0] = "cargo";
        cmds[1] = "run";
        cmds[2] = "--bin";
        cmds[3] = "diff-test";
        cmds[4] = "eval-data-gen";
        cmds[5] = vm.toString(logSize);
        cmds[6] = vm.toString(bytes32(zeta));
        cmds[7] = vm.toString(abi.encode(publicInput));

        bytes memory result = vm.ffi(cmds);
        (uint256 vanishEval, uint256 lagrangeOne, uint256 piEval) =
            abi.decode(result, (uint256, uint256, uint256));

        Poly.EvalData memory evalData = Poly.evalDataGen(domain, zeta, publicInput);
        assertEq(vanishEval, evalData.vanishEval);
        assertEq(lagrangeOne, evalData.lagrangeOne);
        assertEq(piEval, evalData.piEval);
    }
}
