pragma solidity ^0.8.13;

import "forge-std/console.sol";
import "./HotShot.sol";

contract ExampleRollup {
    HotShot hotshot;
    uint256 public stateCommitment;
    uint256 public verifiedBlocks;

    error InvalidProof(uint256 blockHeight, bytes proof);

    event StateUpdate(uint256 blockHeight);

    constructor(address hotshotAddress) {
        hotshot = HotShot(hotshotAddress);
        stateCommitment = 0;
        verifiedBlocks = 0;
    }

    // For demonstration purposes, this always returns true.
    // A real rollup would verify the state update proof against the current state commitment, the most recent hotshot block commitment,
    // and the next state commitment.
    function verifyProof(
        uint256, /* next block commitment*/
        uint256, /* next state commitment*/
        uint256, /* current state commitment*/
        bytes calldata /*proof*/
    ) private pure returns (bool) {
        return true;
    }

    function newBlock(uint256 nextStateCommitment, bytes calldata proof) external {
        uint256 nextBlockCommitment = hotshot.commitments(verifiedBlocks + 1);

        if (!verifyProof(nextBlockCommitment, nextStateCommitment, stateCommitment, proof)) {
            revert InvalidProof(verifiedBlocks + 1, proof);
        }

        verifiedBlocks += 1;
        stateCommitment = nextStateCommitment;

        emit StateUpdate(verifiedBlocks);
    }
}
