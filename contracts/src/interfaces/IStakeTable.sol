// SPDX-License-Identifier: Unlicensed

pragma solidity ^0.8.0;

import { BN254 } from "bn254/BN254.sol";
import { EdOnBN254 } from "../libraries/EdOnBn254.sol";

/// @title Interface for stake table that keep track of validators' external key and staked amount.
/// @dev Stake delegation happens in a separate DelegationPool contract, and specific to
/// instantiation thus not part of this interface
/// @dev Stake table contract should store a reference to the `LightClient.sol` to query
/// "epoch-related" info
interface IStakeTable {
    /// @notice Supported stake type, either using native token or re-staked using ETH
    enum StakeType {
        Native,
        Restake
    }

    event Register(BN254.G2Point, Node);

    /// @dev (sadly, Solidity doesn't support type alias on non-primitive types)
    // We avoid declaring another struct even if the type info helps with readability,
    // extra layer of struct introduces overhead and more gas cost.
    //
    /// @dev BLS verification key (used for Consensus voting in HotShot) is `BN254.G2Point`.
    // `type BlsVerKey is BN254.G2Point;`
    //
    /// @dev Verification key of a Schnorr signature is just a `EdOnBN254.EdOnBN254Point`
    // `type SchnorrVerKey is EdOnBN254.EdOnBN254Point;`

    /// @notice Represents a HotShot validator node
    /// In the dual-staking model, a HotShot validator could have multiple `Node` entries.
    /// @param account The Ethereum account of the validator.
    /// @param stakeType The type of token staked.
    /// @param balance The amount of token staked.
    /// @param registerEpoch The starting epoch for the validator.
    /// @param exitEpoch The ending epoch for the validator.
    /// @param schnorrVK The Schnorr verification key associated.
    struct Node {
        address account;
        StakeType stakeType;
        uint64 balance;
        uint64 registerEpoch;
        uint64 exitEpoch;
        EdOnBN254.EdOnBN254Point schnorrVK;
    }

    // === Table State & Stats ===

    /// @notice Get the total stakes of the registered keys in the latest stake table (Head).
    /// @return The total stake for native token and restaked token respectively.
    function totalStake() external view returns (uint256, uint256);
    /// @notice Get the total number of stakers, uniquely identified by their `blsVK`
    function totalKeys() external view returns (uint32);
    /// @notice Get the total stake of the registered keys in the voting stake table
    /// (LastEpochStart)
    function totalVotingStake() external view returns (uint256);

    /// @notice Look up the balance of `blsVK`
    function lookupStake(BN254.G2Point calldata blsVK) external view returns (uint64);
    /// @notice Look up the full `Node` state associated with `blsVK`
    function lookupNode(BN254.G2Point calldata blsVK) external view returns (Node memory);

    // === Queuing Stats ===

    /// @notice Get the next available epoch for new registration
    function nextRegistrationEpoch() external view returns (uint64);
    /// @notice Get the number of pending registration requests in the waiting queue
    function numPendingRegistrations() external view returns (uint64);
    /// @notice Get the next available epoch for exit
    function nextExitEpoch() external view returns (uint64);
    /// @notice Get the number of pending exit requests in the waiting queue
    function numPendingExit() external view returns (uint64);

    // === Write APIs ===

    /// @notice Register a validator in the stake table, transfer of tokens incurred!
    ///
    /// @param blsVK The BLS verification key
    /// @param schnorrVK The Schnorr verification key (as the auxiliary info)
    /// @param amount The amount to register
    /// @param stakeType The type of staking (native or restaking)
    /// @param blsSig The BLS signature that authenticates the `blsVK` field
    /// @param validUntilEpoch The maximum epoch the sender is willing to wait to be included
    /// (cannot be smaller than the current epoch)
    ///
    /// @return success status
    ///
    /// @dev No validity check on `schnorrVK`, as it's assumed to be sender's responsibility,
    /// the contract only treat it as auxiliary info submitted by `blsVK`.
    /// @dev `blsSig` field is necessary to prevent "rogue public-key attack".
    function register(
        BN254.G2Point calldata blsVK,
        EdOnBN254.EdOnBN254Point calldata schnorrVK,
        uint64 amount,
        StakeType stakeType,
        BN254.G1Point calldata blsSig,
        uint64 validUntilEpoch
    ) external returns (bool);

    /// @notice Deposit more stakes to registered keys
    ///
    /// @param blsVK The BLS verification key
    /// @param amount The amount to deposit
    /// @return (newBalance, effectiveEpoch) the new balance effective at a future epoch
    function deposit(BN254.G2Point calldata blsVK, uint64 amount)
        external
        returns (uint64, uint64);

    /// @notice Request to exit from the stake table, not immediately withdrawable!
    ///
    /// @param blsVK The BLS verification key to exit
    /// @return success status
    function requestExit(BN254.G2Point calldata blsVK) external returns (bool);

    /// @notice Withdraw from the staking pool. Transfers occur! Only successfully exited keys can
    /// withdraw past their `exitEpoch`.
    ///
    /// @param blsVK The BLS verification key to withdraw
    /// @return The total amount withdrawn, equal to `Node.balance` associated with `blsVK`
    function withdrawFunds(BN254.G2Point calldata blsVK) external returns (uint64);
}
