pragma solidity ^0.8.0;

import "solmate/utils/SafeTransferLib.sol";

import { BN254 } from "bn254/BN254.sol";
import { BLSSig } from "./libraries/BLSSig.sol";
import "./interfaces/IStakeTable.sol";
import { ExampleToken } from "../src/ExampleToken.sol";

contract StakeTable is IStakeTable {
    error RestakingNotImplemented();
    error InvalidNextRegistrationEpoch(uint64, uint64);

    mapping(bytes32 keyHash => Node node) public nodes;
    uint256 public totalNativeStake;
    uint256 public totalRestakedStake;
    uint32 public totalKeysVar;
    uint256 public totalVotingStakeVar;
    uint64 public numRegistrations;
    uint64 public numPendingExits;
    uint64 private constant BLOCKS_PER_EPOCH = 10; // TODO make an argument of the constructor?
    uint256 private creationBlock;
    address public tokenAddress;

    constructor(address _tokenAddress) {
        creationBlock = block.number;
        tokenAddress = _tokenAddress;
    }

    /// @dev Computes a hash value of some G2 point
    /// @param blsVK BLS verification key in G2
    /// @return keccak256(blsVK)
    function _hashBlsKey(BN254.G2Point memory blsVK) private pure returns (bytes32) {
        return keccak256(abi.encode(blsVK.x0, blsVK.x1, blsVK.y0, blsVK.y1));
    }

    function currentEpoch() private view returns (uint64) {
        return uint64((block.number - creationBlock) / BLOCKS_PER_EPOCH);
    }

    function totalStake() external view returns (uint256, uint256) {
        return (totalNativeStake, totalRestakedStake);
    }

    function totalKeys() external view returns (uint32) {
        return totalKeysVar;
    }

    function totalVotingStake() external view returns (uint256) {
        return totalVotingStakeVar;
    }

    function lookupStake(BN254.G2Point memory blsVK) external view returns (uint64) {
        Node memory node = this.lookupNode(blsVK);
        return node.balance;
    }

    function lookupNode(BN254.G2Point memory blsVK) external view returns (Node memory) {
        return nodes[_hashBlsKey(blsVK)];
    }

    function nextRegistrationEpoch() external view returns (uint64) {
        // TODO implement queue logic
        return currentEpoch() + 1;
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
        BN254.G2Point memory blsVK,
        EdOnBN254.EdOnBN254Point memory schnorrVK,
        uint64 amount,
        StakeType stakeType,
        BN254.G1Point memory blsSig,
        uint64 validUntilEpoch
    ) external returns (bool) {
        bytes32 key = _hashBlsKey(blsVK);
        Node memory node = nodes[key];

        // The node must not already be registered.
        require(node.account == address(0x0), "The node has already been registered");

        if (stakeType != StakeType.Native) {
            revert RestakingNotImplemented();
        }

        bytes memory message = abi.encode(msg.sender);
        BLSSig.verifyBlsSig(message, blsSig, blsVK);

        // Find the earliest epoch at which this node can register. Usually, this will be
        // currentEpoch() + 1 (the start of the next full epoch), but in periods of high churn the
        // queue may fill up and it may be later. If the queue is so full that the wait time exceeds
        // the caller's desired maximum wait, abort.
        uint64 registerEpoch = this.nextRegistrationEpoch();
        if (registerEpoch > validUntilEpoch) {
            revert InvalidNextRegistrationEpoch(registerEpoch, validUntilEpoch);
        }

        // Create an entry for the node.
        node.account = msg.sender;
        node.balance = amount;
        node.stakeType = stakeType;
        node.schnorrVK = schnorrVK;
        node.registerEpoch = registerEpoch;

        nodes[key] = node;

        // Lock the deposited tokens in this contract.
        if (stakeType == StakeType.Native) {
            totalNativeStake += amount;
            SafeTransferLib.safeTransferFrom(ERC20(tokenAddress), msg.sender, address(this), amount);
        } // Other case will be implemented when we support restaking

        emit Registered(key, registerEpoch, stakeType, amount);

        return true;
    }

    function deposit(BN254.G2Point memory blsVK, uint64 amount) external returns (uint64, uint64) {
        bytes32 hash = _hashBlsKey(blsVK);
        nodes[hash].balance += amount;
        return (0, 0);
    }

    function requestExit(BN254.G2Point memory blsVK) external returns (bool) {
        bytes32 hash = _hashBlsKey(blsVK);
        nodes[hash].exitEpoch = 0;
        return true;
    }

    function withdrawFunds(BN254.G2Point memory blsVK) external returns (uint64) {
        bytes32 hash = _hashBlsKey(blsVK);
        nodes[hash].balance = 0;
        return 0;
    }
}
