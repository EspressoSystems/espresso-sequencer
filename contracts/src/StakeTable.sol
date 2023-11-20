pragma solidity ^0.8.0;

import { BN254 } from "bn254/BN254.sol";
//import { BLSSig } from "./libraries/BLSSig.sol";
//import { BN256G2 } from "./libraries//BN256G2.sol";
import "./interfaces/IStakeTable.sol";

contract StakeTable is IStakeTable {
    mapping(bytes32 keyHash => Node node) private nodesTable;
    uint256[2] private totalStakeArr;
    uint32 private totalNumberOfKeys;
    uint256 private totalVotingStakeVal;
    uint64 private numRegistrations;
    uint64 private numPendingExits;

    // TODO check
    function hashBlsKey(BN254.G1Point calldata blsVK) private pure returns (bytes32) {
        uint256 x = blsVK.x;
        uint256 y = blsVK.y;
        bytes32 hash = keccak256(abi.encode(x, y));
        return hash;
    }

    function totalStake() external view returns (uint256, uint256) {
        return (totalStakeArr[0], totalStakeArr[1]);
    }

    function totalKeys() external view returns (uint32) {
        return totalNumberOfKeys;
    }

    function totalVotingStake() external view returns (uint256) {
        return totalVotingStakeVal;
    }

    function lookupStake(BN254.G1Point calldata blsVK) external view returns (uint64) {
        Node memory node = this.lookupNode(blsVK);
        return node.balance;
    }

    function lookupNode(BN254.G1Point calldata blsVK) external view returns (Node memory) {
        bytes32 hash = hashBlsKey(blsVK);
        return nodesTable[hash];
    }

    function nextRegistrationEpoch() external view returns (uint64) {
        if (numRegistrations == 0) {
            return 0;
        } else {
            return 1;
        }
    }

    function numPendingRegistrations() external view returns (uint64) {
        return numRegistrations;
    }

    function nextExitEpoch() external view returns (uint64) {
        if (numPendingExits == 0) {
            return 0;
        } else {
            return 1;
        }
    }

    function numPendingExit() external view returns (uint64) {
        return numPendingExits;
    }

    function register(
        BN254.G1Point calldata blsVK,
        EdOnBN254.EdOnBN254Point calldata schnorrVK,
        uint64 amount,
        StakeType stakeType,
        bytes calldata blsSig,
        uint64 validUntilEpoch
    ) external returns (bool) {
        uint64 thisEpoch = 0; // TODO
        Node memory node =
            Node(msg.sender, stakeType, amount, thisEpoch, validUntilEpoch, schnorrVK);
        // TODO Check blsSig
        if (blsSig[0] == 0) {
            return false;
        }
        bytes32 hash = hashBlsKey(blsVK);
        nodesTable[hash] = node;
        return true; // TODO
    }

    function deposit(BN254.G1Point calldata blsVK, uint64 amount)
        external
        returns (uint64, uint64)
    {
        bytes32 hash = hashBlsKey(blsVK);
        nodesTable[hash].balance += amount;
        return (0, 0);
    }

    function requestExit(BN254.G1Point calldata blsVK) external returns (bool) {
        bytes32 hash = hashBlsKey(blsVK);
        nodesTable[hash].exitEpoch = 0;
        return true;
    }

    function withdrawFunds(BN254.G1Point calldata blsVK) external returns (uint64) {
        bytes32 hash = hashBlsKey(blsVK);
        nodesTable[hash].balance = 0;
        return 0;
    }
}
