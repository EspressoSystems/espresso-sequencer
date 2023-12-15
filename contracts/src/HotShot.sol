pragma solidity ^0.8.0;

import { BN254 } from "bn254/BN254.sol";
import { BLSSig } from "./libraries/BLSSig.sol";

contract HotShot {
    event NewStakingKey(BN254.G2Point stakingKey, uint256 amount, uint256 index);

    uint256 public constant MAX_BLOCKS = 500;
    mapping(uint256 blockHeight => uint256 commitment) public commitments;
    uint256 public blockHeight;

    // Stake table related data structures
    mapping(uint256 index => uint256 amount) private _stakeAmounts;
    BN254.G2Point[] private _stakingKeys;

    event NewBlocks(uint256 firstBlockNumber, uint256 numBlocks);

    error TooManyBlocks(uint256 numBlocks);
    error InvalidQC(uint256 blockNumber);
    error IncorrectBlockNumber(uint256 blockNumber, uint256 expectedBlockNumber);
    error NoKeySelected();
    error NotEnoughStake();

    struct QC {
        uint256 height;
        uint256 blockCommitment;
        // QC validation is currently mocked out, so the rest of the QC data isn't used, and its
        // format is not finalized. For realism of gas usage, we want something of the correct size.
        // The plan for on-chain QC validation is for the contract to only take a few 32-byte words
        // of the QC, with the rest replaced by a short commitment, since the contract doesn't need
        // all the fields of the QC and storing the whole QC in calldata can be expensive (or even
        // run into RPC size limits).
        uint256 pad1;
        uint256 pad2;
    }

    function _verifyQC(QC calldata /* qc */ ) private pure returns (bool) {
        // TODO Check the QC
        // TODO Check the block number
        return true;
    }

    function newBlocks(QC[] calldata qcs) external {
        if (qcs.length > MAX_BLOCKS) {
            revert TooManyBlocks(qcs.length);
        }

        uint256 firstBlockNumber = blockHeight;
        for (uint256 i = 0; i < qcs.length; ++i) {
            if (qcs[i].height != blockHeight) {
                // Fail quickly if this QC is for the wrong block. In particular, this saves the
                // caller some gas in the race condition where two clients try to sequence the same
                // block at the same time, and the first one wins.
                revert IncorrectBlockNumber(qcs[i].height, blockHeight);
            }

            // Check that QC is signed and well-formed.
            if (!_verifyQC(qcs[i])) {
                revert InvalidQC(blockHeight);
            }

            commitments[blockHeight] = qcs[i].blockCommitment;
            blockHeight += 1;
        }

        emit NewBlocks(firstBlockNumber, qcs.length);
    }

    /// @dev Stake table related functions
    /// @notice This function is for testing purposes only. The real sequencer
    ///         contract will perform several checks before adding a new key (e.g.
    ///         validate deposits).
    /// @param stakingKey public key for the BLS scheme
    /// @param amount stake corresponding to the staking key
    function addNewStakingKey(BN254.G2Point memory stakingKey, uint256 amount) public {
        uint256 index = _stakingKeys.length;
        _stakeAmounts[index] = amount;
        _stakingKeys.push(stakingKey);
        emit NewStakingKey(stakingKey, amount, index);
    }

    function getStakingKey(uint256 index) public view returns (BN254.G2Point memory, uint256) {
        return (_stakingKeys[index], _stakeAmounts[index]);
    }
}
