// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../src/HotShot.sol";

contract HotShotTest is Test {
    HotShot public hotshot;

    event NewBlocks(uint256 firstBlockNumber);

    function setUp() public {
        hotshot = new HotShot();
    }

    function testPublishCommitments() public {
        uint256[] memory comms = new uint[](2);
        bytes[] memory qcs = new bytes[](2);

        comms[0] = 576467464341;
        qcs[0] = "0x3333";

        comms[1] = 234274238974;
        qcs[1] = "0x4444";

        uint256 block_number = 3;

        vm.expectEmit(false, false, false, true, address(hotshot));
        emit NewBlocks(block_number);

        hotshot.newBlocks(block_number, comms, qcs);

        assertEq(hotshot.commitments(3), comms[0]);
        assertEq(hotshot.commitments(4), comms[1]);
        assertEq(hotshot.commitments(5), 0);
    }
}
