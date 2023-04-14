pragma solidity ^0.8.13;

import "forge-std/console.sol";
import "./HotShot.sol";

contract ExampleRollup {
    HotShot hotshot;
    uint256 public stateCommitment;
    uint256 public blockHeight;

    error IncorrectBlockHeight(uint256 blockHeight);
    error InvalidProof(uint256 blockHeight, bytes[] proof);

    event StateUpdate(uint256 blockHeight);

    constructor(address hotshotAddress) {
        hotshot = HotShot(hotshotAddress);
        stateCommitment = 0;
        blockHeight = 0;
    }

    // For demonstration purposes, this always returns true.
    // A real rollup would verify the state update proof against the current state commitment, the most recent hotshot block commitment,
    // and the next state commitment.
    function verifyProof(
        uint256, /* next block commitment*/
        uint256, /* next state commitment*/
        bytes[] calldata /*proof*/
    ) private pure returns (bool) {
        return true;
    }

    function newBlock(uint256 nextStateCommitment, uint256 nextBlockHeight, bytes[] calldata proof) external {
        // State updates must be calculated sequentially
        if (nextBlockHeight != blockHeight + 1) {
            revert IncorrectBlockHeight(nextBlockHeight);
        }

        uint256 nextBlockCommitment = hotshot.commitments(nextBlockHeight);

        if (!verifyProof(nextBlockCommitment, nextStateCommitment, proof)) {
            revert InvalidProof(nextBlockHeight, proof);
        }

        blockHeight = nextBlockHeight;
        stateCommitment = nextStateCommitment;

        emit StateUpdate(blockHeight);
    }
}
