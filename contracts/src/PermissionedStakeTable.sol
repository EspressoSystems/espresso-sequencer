// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";
import { BN254 } from "bn254/BN254.sol";
import { EdOnBN254 } from "./libraries/EdOnBn254.sol";

/**
 * @title PermissionedStakeTable
 * @notice Manages a mapping of stakers with owner-only access control.
 * @dev This contract allows for adding and removing stakers and ensures that only the owner can perform these operations.
 */
contract PermissionedStakeTable is Ownable {
    /**
     * @dev Emitted when stakers are updated (removed or added).
     * @param removed An array of stakers that were removed.
     * @param added An array of new stakers that were added.
     */
    event StakersUpdated(BN254.G2Point[] removed, NodeInfo[] added);

    /**
     * @dev Error thrown when attempting to add a staker that already exists.
     * @param staker The staker that already exists.
     */
    error StakerAlreadyExists(BN254.G2Point staker);

    /**
     * @dev Error thrown when attempting to remove a staker that does not exist.
     * @param staker The staker that was not found.
     */
    error StakerNotFound(BN254.G2Point staker);

    /**
     * @notice Represents information about a node in the network.
     * @dev Contains consensus signing keys and a flag indicating if the node is a DA node.
     */
    struct NodeInfo {
        /// The consensus signing key (BLS verification key).
        BN254.G2Point blsVK;
        /// The Schnorr verification key (used for storage only).
        EdOnBN254.EdOnBN254Point schnorrVK;
        /// Indicates whether the node is a DA Node (used for storage only).
        bool isDA;
    }

    // State mapping from staker IDs to their staking status
    mapping(bytes32 => bool) private stakers;

    /**
     * @notice Initializes the contract with an initial set of stakers.
     * @param initialStakers An array of `NodeInfo` representing the initial set of stakers.
     */
    constructor(NodeInfo[] memory initialStakers) Ownable(msg.sender) {
        insert(initialStakers);
    }

    /**
     * @notice Updates the list of stakers by removing some and adding new ones.
     * @dev Only callable by the owner of the contract.
     * @param stakersToRemove An array of BLS public keys representing stakers to be removed.
     * @param newStakers An array of `NodeInfo` representing new stakers to be added.
     */
    function update(BN254.G2Point[] memory stakersToRemove, NodeInfo[] memory newStakers)
        public
        onlyOwner
    {
        remove(stakersToRemove);
        insert(newStakers);
        emit StakersUpdated(stakersToRemove, newStakers);
    }

    /**
     * @notice Inserts new stakers into the mapping.
     * @dev Reverts if any of the new stakers already exist or if the input array is empty.
     * @param newStakers An array of `NodeInfo` representing new stakers to be added.
     */
    function insert(NodeInfo[] memory newStakers) internal {
        require(newStakers.length > 0, "Input array cannot be empty");
        for (uint256 i = 0; i < newStakers.length; i++) {
            bytes32 stakerID = _hashBlsKey(newStakers[i].blsVK);
            if (stakers[stakerID]) {
                revert StakerAlreadyExists(newStakers[i].blsVK);
            }
            stakers[stakerID] = true;
        }
    }

    /**
     * @notice Removes existing stakers from the mapping.
     * @dev Reverts if any of the specified stakers do not exist or if the input array is empty.
     * @param stakersToRemove An array of BLS public keys representing stakers to be removed.
     */
    function remove(BN254.G2Point[] memory stakersToRemove) internal {
        require(stakersToRemove.length > 0, "Input array cannot be empty");
        for (uint256 i = 0; i < stakersToRemove.length; i++) {
            bytes32 stakerID = _hashBlsKey(stakersToRemove[i]);
            if (!stakers[stakerID]) {
                revert StakerNotFound(stakersToRemove[i]);
            }
            stakers[stakerID] = false;
        }
    }

    /**
     * @notice Checks if a given BLS public key belongs to an active staker.
     * @param staker The BLS public key to check.
     * @return True if the given key belongs to an active staker, false otherwise.
     */
    function isStaker(BN254.G2Point memory staker) external view returns (bool) {
        return stakers[_hashBlsKey(staker)];
    }

    /**
     * @notice Computes a unique hash for a given BLS public key.
     * @param blsVK The BLS public key to hash.
     * @return A unique hash representing the given BLS public key.
     */
    function _hashBlsKey(BN254.G2Point memory blsVK) public pure returns (bytes32) {
        return keccak256(abi.encode(blsVK.x0, blsVK.x1, blsVK.y0, blsVK.y1));
    }
}
