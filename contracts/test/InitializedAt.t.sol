// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import { InitializedAt } from "../src/InitializedAt.sol";

contract MockInitializedAt is InitializedAt {
    function initializeAtBlockPublic() public {
        initializeAtBlock();
    }
}

contract InitializedAtTest is Test {
    function testDeploymentStoresBlockNumber() public {
        InitializedAt init = new InitializedAt();
        assertEq(init.initializedAtBlock(), block.number);
    }

    function testCallingInitializerTwiceReverts() public {
        MockInitializedAt init = new MockInitializedAt();
        vm.expectRevert();
        init.initializeAtBlockPublic();
    }
}
