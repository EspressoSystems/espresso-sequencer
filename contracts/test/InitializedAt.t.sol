// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import { InitializedAt } from "../src/InitializedAt.sol";

contract InitializedAtTest is Test {
    function testEventEmitted() public {
        vm.expectEmit();
        emit InitializedAt.Initialized(1);
        new InitializedAt();
    }
}
