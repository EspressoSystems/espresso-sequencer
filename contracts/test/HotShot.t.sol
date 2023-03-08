// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../src/HotShot.sol";

contract HotShotTest is Test {
    HotShot public hotshot;

    function setUp() public {
        hotshot = new HotShot();
    }

    function testPublishCommitment() public {
        uint256 comm = 576467464341;
        bytes memory qc = "0x3333";
        uint block_number = 3;
        hotshot.publishHotShotBlockCommitment(block_number,comm,qc);

        assertEq(hotshot.getHotShotBlockCommitment(12),0);
        assertEq(hotshot.getHotShotBlockCommitment(3),comm);
    }
}
