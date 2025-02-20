// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import { InitializedAt } from "../src/InitializedAt.sol";

contract InitializedAtTest is Test {
    function testDeploymentStoresBlockNumber() public {
        InitializedAt init = new InitializedAt();
        assertEq(init.initializedAtBlock(), block.number);
    }

    function testCallingInitializerTwiceReverts() public {
        InitializedAt init = new InitializedAt();
        vm.expectRevert();
        init.initialize();
    }
}
