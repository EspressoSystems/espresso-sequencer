// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";
import { BN254 } from "bn254/BN254.sol";
import { EdOnBN254 } from "./libraries/EdOnBn254.sol";

/**
 * @title SimpleStakeTable
 * @dev An stake table mapping with owner-only access control.
 */
contract SimpleStakeTable is Ownable {
    event Added(NodeInfo[]);
    event Removed(NodeInfo[]);

    error StakerAlreadyExists(BN254.G2Point);
    error StakerNotFound(BN254.G2Point);

    struct NodeInfo {
        BN254.G2Point blsVK;
        EdOnBN254.EdOnBN254Point schnorrVK;
    }

    // State mapping from staker IDs to their staking status
    mapping(bytes32 nodeID => bool isStaker) private stakers;

    constructor(address initialOwner) Ownable(initialOwner) { }

    function insert(NodeInfo[] memory newStakers) public onlyOwner {
        // TODO: revert if array empty
        for (uint256 i = 0; i < newStakers.length; i++) {
            bytes32 stakerID = _hashBlsKey(newStakers[i].blsVK);
            if (stakers[stakerID]) {
                revert StakerAlreadyExists(newStakers[i].blsVK);
            }
            stakers[stakerID] = true;
        }
        emit Added(newStakers);
    }

    function remove(NodeInfo[] memory stakersToRemove) external onlyOwner {
        // TODO: revert if array empty
        for (uint256 i = 0; i < stakersToRemove.length; i++) {
            bytes32 stakerID = _hashBlsKey(stakersToRemove[i].blsVK);
            if (!stakers[stakerID]) {
                revert StakerNotFound(stakersToRemove[i].blsVK);
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
