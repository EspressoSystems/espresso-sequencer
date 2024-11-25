// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";
import { BN254 } from "bn254/BN254.sol";

/**
 * @title SimpleStakeTable
 * @dev An stake table mapping with owner-only access control.
 */
contract SimpleStakeTable is Ownable {
    event Added(BN254.G2Point[]);
    event Removed(BN254.G2Point[]);

    error StakerAlreadyExists(BN254.G2Point);
    error StakerNotFound(BN254.G2Point);

    // State mapping from staker IDs to their staking status
    mapping(bytes32 => bool) private stakers;

    constructor(address initialOwner) Ownable(initialOwner) { }

    function insert(BN254.G2Point[] memory newStakers) external onlyOwner {
        // TODO: revert if array empty
        for (uint256 i = 0; i < newStakers.length; i++) {
            bytes32 stakerID = _hashBlsKey(newStakers[i]);
            if (stakers[stakerID]) {
                revert StakerAlreadyExists(newStakers[i]);
            }
            stakers[stakerID] = true;
        }
        emit Added(newStakers);
    }

    function remove(BN254.G2Point[] memory stakersToRemove) external onlyOwner {
        // TODO: revert if array empty
        for (uint256 i = 0; i < stakersToRemove.length; i++) {
            bytes32 stakerID = _hashBlsKey(stakersToRemove[i]);
            if (!stakers[stakerID]) {
                revert StakerNotFound(stakersToRemove[i]);
            }
            stakers[stakerID] = false;
        }
        emit Removed(stakersToRemove);
    }

    function isStaker(BN254.G2Point memory staker) external view returns (bool) {
        return stakers[_hashBlsKey(staker)];
    }

    function _hashBlsKey(BN254.G2Point memory blsVK) public pure returns (bytes32) {
        return keccak256(abi.encode(blsVK.x0, blsVK.x1, blsVK.y0, blsVK.y1));
    }
}
