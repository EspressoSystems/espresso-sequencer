// SPDX-License-Identifier: Unlicensed

/* solhint-disable contract-name-camelcase, func-name-mixedcase, one-contract-per-file */

pragma solidity ^0.8.0;

// Libraries
import { Test } from "forge-std/Test.sol";

contract FeeContractTest is Test {
    //function setUp() public { }

    function abiEncodePackedAssembly(bytes32 a, bytes32 b) public returns (bytes memory) {
        bytes memory result = new bytes(64); // Allocate memory for the result
        assembly {
            // Store the first bytes32 value
            mstore(result, a)

            // Store the second bytes32 value
            mstore(add(result, 32), b)
        }
        return result;
    }

    //    function testAbiDecodeAssembly() public payable {
    //        bytes32 a = 0x0000000000000000000000000000000000000000000000000000000000000001;
    //        bytes32 b = 0x0000000000000000000000000000000000000000000000000000000000000002;
    //
    //        bytes memory v = abiEncodePackedAssembly(a, b);
    //        bytes memory w = abi.encodePacked(a, b);
    //        assertEq(v, w);
    //    }
}
