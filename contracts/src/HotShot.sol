pragma solidity ^0.8.13;

import "forge-std/console.sol";

contract HotShot {
    uint256 public constant MAX_BLOCKS = 1000;
    mapping(uint256 => uint256) public commitments;

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

    function newBlocks(uint256 firstBlockNumber, uint256[] calldata newCommitments, bytes[] calldata qcs) external {
        if (newCommitments.length != qcs.length) {
            revert WrongNumberOfQCs(newCommitments.length, qcs.length);
        }
        if (newCommitments.length > MAX_BLOCKS) {
            revert TooManyBlocks(newCommitments.length);
        }

        for (uint256 i = 0; i < newCommitments.length; ++i) {
            uint256 blockNumber = firstBlockNumber + i;

            if (!verifyQC(blockNumber, newCommitments[i], qcs[i])) {
                revert InvalidQC(blockNumber);
            }

            commitments[blockNumber] = newCommitments[i];
        }

        emit NewBlocks(firstBlockNumber, newCommitments.length);
    }
}
