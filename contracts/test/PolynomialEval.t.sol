// SPDX-License-Identifier: Unlicensed

/* solhint-disable contract-name-camelcase, func-name-mixedcase */

pragma solidity ^0.8.0;

// Libraries
import "forge-std/Test.sol";

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
