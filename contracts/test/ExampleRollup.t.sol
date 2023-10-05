// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../src/HotShot.sol";
import "../src/ExampleRollup.sol";

contract ExampleRollupTest is Test {
    HotShot public hotshot;
    ExampleRollup public rollup;

    event StateUpdate(uint256 blockHeight, uint256 stateCommitment);

    function setUp() public {
        hotshot = new HotShot();
        rollup = new ExampleRollup(address(hotshot), 0);
    }

    function testStateUpdate() public {
        // Add a commitment to hotshot
        HotShot.QC[] memory qcs = new HotShot.QC[](1);
        qcs[0].blockCommitment = 576467464341;
        qcs[0].height = 0;
        hotshot.newBlocks(qcs);

        // Send a state update to the rollup
        ExampleRollup.BatchProof memory proof = ExampleRollup.BatchProof({
            firstBlock: qcs[0].blockCommitment,
            lastBlock: qcs[0].blockCommitment,
            oldState: 0,
            newState: 523123
        });
        vm.expectEmit(false, false, false, true, address(rollup));
        emit StateUpdate(1, proof.newState);
        rollup.verifyBlocks(1, proof.newState, proof);

        assertEq(rollup.stateCommitment(), proof.newState);
        assertEq(rollup.numVerifiedBlocks(), 1);
    }

    function testInvalidProof() public {
        // Add a commitment to hotshot
        HotShot.QC[] memory qcs = new HotShot.QC[](1);
        qcs[0].blockCommitment = 576467464341;
        qcs[0].height = 0;
        hotshot.newBlocks(qcs);
        uint256 invalidState = 523124;

        // Send an invalid state update to the rollup
        ExampleRollup.BatchProof memory proof = ExampleRollup.BatchProof({
            firstBlock: qcs[0].blockCommitment,
            lastBlock: 0,
            oldState: 0,
            newState: 523123
        });
        vm.expectRevert(
            abi.encodeWithSelector(
                ExampleRollup.InvalidProof.selector,
                qcs[0].blockCommitment,
                qcs[0].blockCommitment,
                proof.oldState,
                invalidState,
                proof
            )
        );
        rollup.verifyBlocks(1, invalidState, proof);
    }
}
