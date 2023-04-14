// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../src/HotShot.sol";
import "../src/ExampleRollup.sol";

contract ExampleRollupTest is Test {
    HotShot public hotshot;
    ExampleRollup public rollup;

    event StateUpdate(uint256 blockHeight);

    function setUp() public {
        hotshot = new HotShot();
        rollup = new ExampleRollup(address(hotshot));
    }

    function testStateUpdate() public {
        // Add a commitment to hotshot
        uint256[] memory comms = new uint[](2);
        bytes[] memory qcs = new bytes[](2);

        comms[0] = 576467464341;
        qcs[0] = "0x3333";

        comms[1] = 234274238974;
        qcs[1] = "0x4444";

        hotshot.newBlocks(comms, qcs);

        // Send a state update to the rollup
        bytes[] memory proof = new bytes[](1);
        uint256 nextStateCommitment = 523123;
        uint256 nextBlockHeight = 1;

        vm.expectEmit(false, false, false, true, address(rollup));
        emit StateUpdate(nextBlockHeight);

        rollup.newBlock(nextStateCommitment, nextBlockHeight, proof);

        assertEq(rollup.stateCommitment(), nextStateCommitment);
        assertEq(rollup.blockHeight(), nextBlockHeight);

        // Check that contract enforces sequential updates
        vm.expectRevert();
        rollup.newBlock(nextStateCommitment, 5, proof);
    }
}
