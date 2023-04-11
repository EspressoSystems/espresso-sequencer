// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../src/HotShot.sol";

contract HotShotTest is Test {
    HotShot public hotshot;

    event NewBlocks(uint256 firstBlockNumber, uint256 numBlocks);

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

        vm.expectEmit(false, false, false, true, address(hotshot));
        emit NewBlocks(0, 2);

        hotshot.newBlocks(comms, qcs);

        assertEq(hotshot.commitments(0), comms[0]);
        assertEq(hotshot.commitments(1), comms[1]);
        assertEq(hotshot.commitments(2), 0);
    }

    function testExpander() public {
        uint8[] memory expander_input = new uint8[](2);
        expander_input[0] = 1;
        expander_input[1] = 2;
        hotshot.expand(expander_input);
    }
}
