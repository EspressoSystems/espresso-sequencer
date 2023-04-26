pragma solidity ^0.8.0;

import {BN254} from "bn254/BN254.sol";
import {BLSSig} from "./libraries/BLSSig.sol";

// TODO use forge install with a fork of the library that supports solidity ^0.8.0.
import {BN256G2} from "./libraries//BN256G2.sol";

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

    // TODO document
    function verify_agg_sig(
        bytes memory message,
        BN254.G1Point memory sig,
        bool[] memory bitmap,
        uint256 min_stake_threshold
    ) public view {
        // Build aggregated public key

        // Loop until we find a one in the bitmap
        uint256 index = 0;
        while (!bitmap[index] && index < bitmap.length) {
            index++;
        }

        // TODO missing: compute the total amount of weight from the bitmap and check it against some value t passed as argument

        // TODO test
        require(index < bitmap.length, "At least one key must be selected.");

        // TODO test
        // Compute the stake corresponding to the signers and check if it is enough
        uint256 stake = 0;
        for (uint256 i = 0; i < bitmap.length; i++) {
            if (bitmap[i]) {
                stake += stakeAmounts[i]; // TODO check to avoid wrapping around?
            }
        }
        // TODO test
        require(stake >= min_stake_threshold, "Not enough stake is available for validating the signature.");

        BN254.G2Point memory agg_pk = stakingKeys[index];
        for (uint256 i = index + 1; i < bitmap.length; i++) {
            // Compute the group multiplication of the two keys

            if (bitmap[i]) {
                BN254.G2Point memory pk = stakingKeys[i];

                // Note: (x,y) coordinates for each field component must be inverted!
                uint256 p1xy = agg_pk.x0;
                uint256 p1xx = agg_pk.x1;
                uint256 p1yy = agg_pk.y0;
                uint256 p1yx = agg_pk.y1;
                uint256 p2xy = pk.x0;
                uint256 p2xx = pk.x1;
                uint256 p2yy = pk.y0;
                uint256 p2yx = pk.y1;

                (uint256 p3xx, uint256 p3xy, uint256 p3yx, uint256 p3yy) =
                    BN256G2.ECTwistAdd(p1xx, p1xy, p1yx, p1yy, p2xx, p2xy, p2yx, p2yy);
                agg_pk = BN254.G2Point(p3xy, p3xx, p3yy, p3yx);
            }
        }

        BLSSig.verify_bls_sig(message, sig, agg_pk);
    }
}
