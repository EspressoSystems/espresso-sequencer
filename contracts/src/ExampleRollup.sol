pragma solidity ^0.8.13;

import "forge-std/console.sol";
import "./HotShot.sol";

contract ExampleRollup {
    HotShot hotshot;
    uint256 public stateCommitment;
    uint256 public verifiedBlocks;

    // Attempted to verify a proof of the blocks from `verifiedBlocks` to `verifiedBlocks + count`,
    // but the HotShot `blockHeight` is less than  `verifiedBlocks + count`.
    error NotYetSequenced(uint256 verifiedBlocks, uint64 count, uint256 blockHeight);
    // Attempted to verify an invalid proof of the blocks from `firstBlock` to `firstBlock + count`.
    error InvalidProof(uint256 firstBlock, uint64 count, bytes proof);

    event StateUpdate(uint256 blockHeight);

    constructor(address hotshotAddress) {
        hotshot = HotShot(hotshotAddress);
        stateCommitment = 0;
        verifiedBlocks = 0;
    }

    // Verify a batch proof of the execution of a chain of blocks.
    //
    // For demonstration purposes, this always returns true.
    //
    // A real rollup would verify the state update proof against
    // * the current state commitment
    // * the last verified hotshot block commitment
    // * the last hotshot block commitment in a chain of newly verified blocks extending from the
    //   last verified block
    // * the new state commitment after executing the chain of new blocks
    function verifyProof(
        uint256, /* firstBlock */
        uint256, /* lastBlock */
        uint256, /* oldState */
        uint256, /* newState */
        bytes calldata /* proof */
    ) private pure returns (bool) {
        return true;
    }

    function verifyBlocks(uint64 count, uint256 nextStateCommitment, bytes calldata proof) external {
        uint256 blockHeight = hotshot.blockHeight();
        if (verifiedBlocks + count >= blockHeight) {
            revert NotYetSequenced(verifiedBlocks, count, blockHeight);
        }

        uint256 firstBlock = hotshot.commitments(verifiedBlocks);
        uint256 lastBlock = hotshot.commitments(verifiedBlocks + count);
        if (!verifyProof(firstBlock, lastBlock, stateCommitment, nextStateCommitment, proof)) {
            revert InvalidProof(verifiedBlocks, count, proof);
        }

        verifiedBlocks += count;
        stateCommitment = nextStateCommitment;
        emit StateUpdate(verifiedBlocks);
    }
}
