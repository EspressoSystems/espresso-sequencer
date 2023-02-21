// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract Counter {
    uint256 public number;

    function setNumber(uint256 newNumber2) public {
        number = newNumber2;
    }

    function doNothing() public {

    }

    function doNothing3() public {

    }

    function increment() public {
        number++;
    }
}
