pragma solidity ^0.8.0;

import {BN254} from "bn254/BN254.sol";

contract HotShot {
    uint256 public constant MAX_BLOCKS = 1000;
    mapping(uint256 => uint256) public commitments;
    uint256 public blockHeight;

    // Stake table related data structures
    mapping(uint256 => uint256) private stakeAmounts;
    BN254.G2Point[] private stakingKeys;

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

    // Stake table related functions
    function addNewStakingKey(BN254.G2Point memory staking_key, uint256 amount) public {
        uint256 index = stakingKeys.length;
        stakeAmounts[index] = amount;
        stakingKeys.push(staking_key);
    }

    function getStakingKey(uint256 index) public view returns (BN254.G2Point memory, uint256) {
        return (stakingKeys[index], stakeAmounts[index]);
    }
}
