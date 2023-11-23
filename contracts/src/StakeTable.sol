pragma solidity ^0.8.0;

import { BN254 } from "bn254/BN254.sol";
import { BLSSig } from "./libraries/BLSSig.sol";
import "./interfaces/IStakeTable.sol";

contract StakeTable is IStakeTable {
    mapping(bytes32 keyHash => Node node) private nodesTable;
    uint256[2] private totalStakeArr;
    uint32 private totalNumberOfKeys;
    uint256 private totalVotingStakeVal;
    uint64 private numRegistrations;
    uint64 private numPendingExits;

    // TODO check
    function hashBlsKey(BN254.G2Point calldata blsVK) private pure returns (bytes32) {
        uint256 x0 = blsVK.x0;
        uint256 x1 = blsVK.x1;
        uint256 y0 = blsVK.y0;
        uint256 y1 = blsVK.y1;
        bytes32 hash = keccak256(abi.encode(x0, x1, y0, y1));
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

    function lookupStake(BN254.G2Point calldata blsVK) external view returns (uint64) {
        Node memory node = this.lookupNode(blsVK);
        return node.balance;
    }

    function lookupNode(BN254.G2Point calldata blsVK) external view returns (Node memory) {
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
        BN254.G2Point calldata blsVK,
        EdOnBN254.EdOnBN254Point calldata schnorrVK,
        uint64 amount,
        StakeType stakeType,
        BN254.G1Point calldata blsSig,
        uint64 validUntilEpoch
    ) external returns (bool) {
        bytes32 key = hashBlsKey(blsVK);
        Node memory node = nodesTable[key];

        // The node must not already be registered.
        require(node.account == address(0x0));

        bytes memory message = abi.encode(msg.sender);
        BLSSig.verifyBlsSig(message, blsSig, blsVK);

        // Find the earliest epoch at which this node can register. Usually, this will be
        // currentEpoch() + 1 (the start of the next full epoch), but in periods of high churn the
        // queue may fill up and it may be later. If the queue is so full that the wait time exceeds
        // the caller's desired maximum wait, abort.
        uint64 registerEpoch = this.nextRegistrationEpoch();
        if (registerEpoch > validUntilEpoch) {
            revert();
        }

        // Create an entry for the node.
        node.account = msg.sender;
        node.balance = amount;
        node.stakeType = stakeType;
        node.schnorrVK = schnorrVK;
        node.registerEpoch = registerEpoch;

        emit Register(blsVK, node);

        // Lock the deposited tokens in this contract.
        // TODO take the BEANS tokens

        return true;
    }

    function deposit(BN254.G2Point calldata blsVK, uint64 amount)
        external
        returns (uint64, uint64)
    {
        bytes32 hash = hashBlsKey(blsVK);
        nodesTable[hash].balance += amount;
        return (0, 0);
    }

    function requestExit(BN254.G2Point calldata blsVK) external returns (bool) {
        bytes32 hash = hashBlsKey(blsVK);
        nodesTable[hash].exitEpoch = 0;
        return true;
    }

    function withdrawFunds(BN254.G2Point calldata blsVK) external returns (uint64) {
        bytes32 hash = hashBlsKey(blsVK);
        nodesTable[hash].balance = 0;
        return 0;
    }
}
