pragma solidity ^0.8.0;

import { SafeTransferLib, ERC20 } from "solmate/utils/SafeTransferLib.sol";
import { BN254 } from "bn254/BN254.sol";
import { BLSSig } from "./libraries/BLSSig.sol";
import { IStakeTable } from "./interfaces/IStakeTable.sol";
import { ExampleToken } from "../src/ExampleToken.sol";
import { LightClient } from "../src/LightClient.sol";
import { EdOnBN254 } from "./libraries/EdOnBn254.sol";

contract StakeTable is IStakeTable {
    error RestakingNotImplemented();
    error InvalidNextRegistrationEpoch(uint64, uint64);
    error NodeAlreadyRegistered();

    mapping(bytes32 keyHash => Node node) public nodes;
    uint256 public totalNativeStake;
    uint256 public totalRestakedStake;
    uint64 public numRegistrations;
    uint64 public numPendingExits;
    address public tokenAddress;
    LightClient public lightClient;

    constructor(address _tokenAddress, address _lightClientAddress) {
        tokenAddress = _tokenAddress;
        lightClient = LightClient(_lightClientAddress);
    }

    /// @dev Computes a hash value of some G2 point
    /// @param blsVK BLS verification key in G2
    /// @return keccak256(blsVK)
    function _hashBlsKey(BN254.G2Point memory blsVK) public pure returns (bytes32) {
        return keccak256(abi.encode(blsVK.x0, blsVK.x1, blsVK.y0, blsVK.y1));
    }

    function currentEpoch() private view returns (uint64) {
        return lightClient.currentEpoch();
    }

    function totalStake() external view override returns (uint256, uint256) {
        return (totalNativeStake, totalRestakedStake);
    }

    function lookupStake(BN254.G2Point memory blsVK) external view override returns (uint64) {
        Node memory node = this.lookupNode(blsVK);
        return node.balance;
    }

    function lookupNode(BN254.G2Point memory blsVK) external view override returns (Node memory) {
        return nodes[_hashBlsKey(blsVK)];
    }

    function nextRegistrationEpoch() external view override returns (uint64) {
        // TODO implement queue logic
        return currentEpoch() + 1;
    }

    function numPendingRegistrations() external view override returns (uint64) {
        return numRegistrations;
    }

    function nextExitEpoch() external view override returns (uint64) {
        if (numPendingExits == 0) {
            return 0;
        } else {
            return 1;
        }
    }

    function numPendingExit() external view override returns (uint64) {
        return numPendingExits;
    }

    function register(
        BN254.G2Point memory blsVK,
        EdOnBN254.EdOnBN254Point memory schnorrVK,
        uint64 amount,
        StakeType stakeType,
        BN254.G1Point memory blsSig,
        uint64 validUntilEpoch
    ) external override returns (bool) {
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

        bytes32 key = _hashBlsKey(blsVK);
        Node memory node = nodes[key];

        // The node must not already be registered.
        if (node.account != address(0x0)) {
            revert NodeAlreadyRegistered();
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

    function deposit(BN254.G2Point memory blsVK, uint64 amount)
        external
        override
        returns (uint64, uint64)
    {
        bytes32 hash = _hashBlsKey(blsVK);
        nodes[hash].balance += amount;
        return (0, 0);
    }

    function requestExit(BN254.G2Point memory blsVK) external override returns (bool) {
        bytes32 hash = _hashBlsKey(blsVK);
        nodes[hash].exitEpoch = 0;
        return true;
    }

    function withdrawFunds(BN254.G2Point memory blsVK) external override returns (uint64) {
        bytes32 hash = _hashBlsKey(blsVK);
        nodes[hash].balance = 0;
        return 0;
    }
}
