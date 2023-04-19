pragma solidity ^0.8.0;

import {BN254} from "./libraries/BN254.sol";
import {BLSSig} from "./libraries/BLSSig.sol";

contract HotShot {
    uint256 public constant MAX_BLOCKS = 1000;
    mapping(uint256 => uint256) public commitments;
    uint256 public blockHeight;

    event NewBlocks(uint256 firstBlockNumber, uint256 numBlocks);

    error WrongNumberOfQCs(uint256 numBlocks, uint256 numQCs);
    error TooManyBlocks(uint256 numBlocks);
    error InvalidQC(uint256 blockNumber);

    function verifyQC(uint256, /*blockNumber*/ uint256, /*commitment*/ bytes calldata /*qc*/ )
        private
        pure
        returns (bool)
    {
        // TODO Check the QC
        // TODO Check the block number
        return true;
    }

    function newBlocks(uint256[] calldata newCommitments, bytes[] calldata qcs) external {
        if (newCommitments.length != qcs.length) {
            revert WrongNumberOfQCs(newCommitments.length, qcs.length);
        }
        if (newCommitments.length > MAX_BLOCKS) {
            revert TooManyBlocks(newCommitments.length);
        }

        uint256 firstBlockNumber = blockHeight;
        for (uint256 i = 0; i < newCommitments.length; ++i) {
            if (!verifyQC(blockHeight, newCommitments[i], qcs[i])) {
                revert InvalidQC(blockHeight);
            }

            commitments[blockHeight] = newCommitments[i];
            blockHeight += 1;
        }

        emit NewBlocks(firstBlockNumber, newCommitments.length);
    }

    ////// BLS signature verification

    // For testing purposes only
    // TODO make another contract ?
    function hash_to_field(bytes memory message) public pure returns (uint256) {
        return BLSSig.hash_to_field(message);
    }

    // For testing purposes only
    // TODO make another contract ?
    function hash_to_curve(bytes memory input) public view returns (uint256, uint256) {
        return BLSSig.hash_to_curve(input);
    }

    // For testing purposes only
    // TODO make another contract ?
    function verify_bls_sig(bytes memory message, BN254.G1Point memory sig, BN254.G2Point memory pk)
        public
        view
        returns (bool)
    {
        return BLSSig.verify_bls_sig(message, sig, pk);
    }
}
