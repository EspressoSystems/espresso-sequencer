// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import { InitializedAt } from "../src/InitializedAt.sol";

contract InitializedAtTest is Test {
    function test_Deployment_StoresBlockNumber() public {
        InitializedAt init = new InitializedAt();
        assertEq(init.initializedAtBlock(), block.number);
    }
}
