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
        HotShot.QC[] memory qcs = new HotShot.QC[](2);

        qcs[0].blockCommitment = 576467464341;
        qcs[0].height = 0;

        qcs[1].blockCommitment = 234274238974;
        qcs[1].height = 1;

        vm.expectEmit(false, false, false, true, address(hotshot));
        emit NewBlocks(0, 2);

        hotshot.newBlocks(qcs);

        assertEq(hotshot.commitments(0), qcs[0].blockCommitment);
        assertEq(hotshot.commitments(1), qcs[1].blockCommitment);
        assertEq(hotshot.commitments(2), 0);

        // Test idempotency: sequencing the same blocks again should revert.
        vm.expectRevert(abi.encodeWithSelector(HotShot.IncorrectBlockNumber.selector, 0, 2));
        hotshot.newBlocks(qcs);
    }
}
