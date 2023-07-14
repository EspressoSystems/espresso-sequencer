pragma solidity ^0.8.0;

import {BN254} from "bn254/BN254.sol";
import {BLSSig} from "./libraries/BLSSig.sol";

import {BN256G2} from "./libraries//BN256G2.sol";

contract HotShot {
    event NewStakingKey(BN254.G2Point stakingKey, uint256 amount, uint256 index);

    uint256 public constant MAX_BLOCKS = 500;
    mapping(uint256 blockHeight => uint256 commitment) public commitments;
    uint256 public blockHeight;

    // Stake table related data structures
    mapping(uint256 index => uint256 amount) private _stakeAmounts;
    BN254.G2Point[] private _stakingKeys;

    event NewBlocks(uint256 firstBlockNumber, uint256 numBlocks);

    error WrongNumberOfQCs(uint256 numBlocks, uint256 numQCs);
    error TooManyBlocks(uint256 numBlocks);
    error InvalidQC(uint256 blockNumber);
    error NoKeySelected();
    error NotEnoughStake();

    function _verifyQC(uint256, /*blockNumber*/ uint256, /*commitment*/ bytes calldata /*qc*/ )
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
            if (!_verifyQC(blockHeight, newCommitments[i], qcs[i])) {
                revert InvalidQC(blockHeight);
            }

            commitments[blockHeight] = newCommitments[i];
            blockHeight += 1;
        }

        emit NewBlocks(firstBlockNumber, newCommitments.length);
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

    /// @dev Verify an aggregated signature against a bitmap (use to reconstruct
    ///      the aggregated public key) and some stake threshold. If the stake
    ///      involved by the signers is bigger than the threshold and the signature is
    ///      valid then the validation passes, otherwise the transaction
    ///      reverts.
    /// @param message message to check the signature against
    /// @param sig aggregated signature
    /// @param bitmap bit vector that corresponds to the public keys of the stake
    ///        table to take into account to build the aggregated public key
    /// @param minStakeThreshold total stake that must me matched by the
    ///        signers in order for the signature to be valid
    function verifyAggSig(
        bytes memory message,
        BN254.G1Point memory sig,
        bool[] memory bitmap,
        uint256 minStakeThreshold
    ) public view {
        require(bitmap.length <= _stakingKeys.length, "bitmap is too long");

        // Build aggregated public key

        // Loop until we find a one in the bitmap
        uint256 index = 0;
        while (!bitmap[index] && index < bitmap.length) {
            index++;
        }

        if (index >= bitmap.length) {
            revert NoKeySelected();
        }

        // Compute the stake corresponding to the signers and check if it is enough
        uint256 stake = 0;
        for (uint256 i = index; i < bitmap.length; i++) {
            if (bitmap[i]) {
                stake += _stakeAmounts[i]; // TODO check to avoid wrapping around?
            }
        }

        if (stake < minStakeThreshold) {
            revert NotEnoughStake();
        }

        BN254.G2Point memory aggPk = _stakingKeys[index];
        for (uint256 i = index + 1; i < bitmap.length; i++) {
            if (bitmap[i]) {
                BN254.G2Point memory pk = _stakingKeys[i];

                // Note: (x,y) coordinates for each field component must be inverted.
                uint256 p1xy = aggPk.x0;
                uint256 p1xx = aggPk.x1;
                uint256 p1yy = aggPk.y0;
                uint256 p1yx = aggPk.y1;
                uint256 p2xy = pk.x0;
                uint256 p2xx = pk.x1;
                uint256 p2yy = pk.y0;
                uint256 p2yx = pk.y1;

                (uint256 p3xx, uint256 p3xy, uint256 p3yx, uint256 p3yy) =
                    BN256G2.ECTwistAdd(p1xx, p1xy, p1yx, p1yy, p2xx, p2xy, p2yx, p2yy);
                aggPk = BN254.G2Point(p3xy, p3xx, p3yy, p3yx);
            }
        }

        BLSSig.verifyBlsSig(message, sig, aggPk);
    }
}
