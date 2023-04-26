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
        rollup = new ExampleRollup(address(hotshot), 0);
    }

    function testStateUpdate() public {
        // Add a commitment to hotshot
        uint256[] memory comms = new uint[](1);
        bytes[] memory qcs = new bytes[](1);
        comms[0] = 576467464341;
        qcs[0] = "0x3333";
        hotshot.newBlocks(comms, qcs);

        // Send a state update to the rollup
        ExampleRollup.BatchProof memory proof =
            ExampleRollup.BatchProof({firstBlock: comms[0], lastBlock: comms[0], oldState: 0, newState: 523123});
        vm.expectEmit(false, false, false, true, address(rollup));
        emit StateUpdate(1);
        rollup.verifyBlocks(1, proof.newState, proof);

        assertEq(rollup.stateCommitment(), proof.newState);
        assertEq(rollup.verifiedBlocks(), 1);
    }

    function testInvalidProof() public {
        // Add a commitment to hotshot
        uint256[] memory comms = new uint[](1);
        bytes[] memory qcs = new bytes[](1);
        comms[0] = 576467464341;
        qcs[0] = "0x3333";
        hotshot.newBlocks(comms, qcs);

        // Send an invalid state update to the rollup
        ExampleRollup.BatchProof memory proof =
            ExampleRollup.BatchProof({firstBlock: comms[0], lastBlock: 0, oldState: 0, newState: 523123});
        vm.expectRevert(
            abi.encodeWithSelector(
                ExampleRollup.InvalidProof.selector, comms[0], comms[0], proof.oldState, proof.newState, proof
            )
        );
        rollup.verifyBlocks(1, proof.newState, proof);
    }
}
