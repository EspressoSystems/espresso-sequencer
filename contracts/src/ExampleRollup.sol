pragma solidity ^0.8.13;

import "./HotShot.sol";

contract ExampleRollup {
    HotShot public hotshot;
    uint256 public stateCommitment;
    uint256 public numVerifiedBlocks;

    // Attempted to verify a proof of the blocks from `numVerifiedBlocks` to
    // `numVerifiedBlocks + count`, but the HotShot `blockHeight` is less than
    // `numVerifiedBlocks + count`.
    error NotYetSequenced(uint256 numVerifiedBlocks, uint64 count, uint256 blockHeight);
    // Attempted to verify an invalid proof.
    error InvalidProof(uint256 firstBlock, uint256 lastBlock, uint256 oldState, uint256 newState, BatchProof proof);
    // Attempted to verify an empty chain of blocks;
    error NoBlocks();

    event StateUpdate(uint256 blockHeight, uint256 stateCommitment);

    constructor(address hotshotAddress, uint256 initialState) {
        hotshot = HotShot(hotshotAddress);
        stateCommitment = initialState;
        numVerifiedBlocks = 0;
    }

    // A batch proof of the execution of a chain of blocks.
    //
    // For demonstration purposes, this just contains the public parameters that the proof was
    // generated with, so that the contract can at least check that the prover is submitting the
    // intended proof. In a real rollup, this would contain a SNARK witness attesting to the state
    // update.
    struct BatchProof {
        uint256 firstBlock;
        uint256 lastBlock;
        uint256 oldState;
        uint256 newState;
    }

    // Verify a batch proof of the execution of a chain of blocks.
    //
    // For demonstration purposes, this merely checks that the public parameters with which `proof`
    // was generated match the claimed public parameters.
    //
    // A real rollup would verify the state update proof against
    // * the current state commitment
    // * the first unverified HotShot block commitment
    // * the last HotShot block commitment in the claimed chain of blocks
    // * the new state commitment after executing the chain of new blocks
    // The proof would constrain the VM execution semantics for each block and would enforce that
    // the executed blocks form a chain from `firstBlock` to `lastBlock`. The latter condition
    // forces the prover to execute the correct chain of blocks without explicitly taking this
    // entire chain as a public input (which would be expensive). This holds up to collision
    // resistance of the hash function used to link each HotShot block to its parent.
    //
    // The proof would also verify consistency between the NMT block roots stored in the
    // BatchProof and the committed hotshot blocks.
    function _verifyProof(
        uint256, /*firstBlock */
        uint256, /*lastBlock */
        uint256 oldState,
        uint256 newState,
        BatchProof calldata proof
    ) private pure returns (bool) {
        return oldState == proof.oldState && newState == proof.newState;
    }

    function verifyBlocks(uint64 count, uint256 nextStateCommitment, BatchProof calldata proof) external {
        if (count == 0) {
            revert NoBlocks();
        }

        uint256 blockHeight = hotshot.blockHeight();
        if (numVerifiedBlocks + count > blockHeight) {
            revert NotYetSequenced(numVerifiedBlocks, count, blockHeight);
        }

        uint256 firstBlock = hotshot.commitments(numVerifiedBlocks);
        uint256 lastBlock = hotshot.commitments(numVerifiedBlocks + count - 1);
        if (!_verifyProof(firstBlock, lastBlock, stateCommitment, nextStateCommitment, proof)) {
            revert InvalidProof(firstBlock, lastBlock, stateCommitment, nextStateCommitment, proof);
        }

        numVerifiedBlocks += count;
        stateCommitment = nextStateCommitment;
        emit StateUpdate(numVerifiedBlocks, stateCommitment);
    }
}
