// SPDX-License-Identifier: Unlicensed

pragma solidity ^0.8.0;

import { BN254 } from "bn254/BN254.sol";
import { EdOnBN254 } from "../libraries/EdOnBn254.sol";

/// @title Interface for stake table that keep track of validators' external key and staked amount.
/// @dev Stake delegation happens in a separate DelegationPool contract, and specific to
/// instantiation thus not part of this interface
/// @dev Stake table contract should store a reference to the `LightClient.sol` to query
/// "epoch-related" info
abstract contract AbstractStakeTable {
    /// @notice Supported stake type, either using native token or re-staked using ETH
    enum StakeType {
        Native,
        Restake
    }

    /// @notice Get the total number of stakers, uniquely identified by their `blsVK`
    uint32 public totalKeys;
    /// @notice Get the total stake of the registered keys in the voting stake table
    /// (LastEpochStart)
    uint256 public totalVotingStake;

    /// @notice Signals a registration of a BLS public key.
    /// @param blsVKhash hash of the BLS public key that is registered.
    /// @param registerEpoch epoch when the registration becomes effective.
    /// @param amountDeposited amount deposited when registering the new node.
    event Registered(bytes32 blsVKhash, uint64 registerEpoch, uint256 amountDeposited);

    /// @notice Signals an exit request has been granted.
    /// @param blsVKhash hash of the BLS public key owned by the user who requested to exit.
    /// @param exitEpoch epoch when the user will be allowed to withdraw its funds.
    event Exit(bytes32 blsVKhash, uint64 exitEpoch);

    /// @notice Signals a deposit to a BLS public key.
    /// @param blsVKhash hash of the BLS public key that received the deposit.
    /// @param amount amount of the deposit
    event Deposit(bytes32 blsVKhash, uint256 amount);

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
    /// @param balance The amount of token staked.
    /// @param registerEpoch The starting epoch for the validator.
    /// @param exitEpoch The ending epoch for the validator.
    /// @param schnorrVK The Schnorr verification key associated.
    struct Node {
        address account;
        uint256 balance;
        uint64 registerEpoch;
        uint64 exitEpoch;
        EdOnBN254.EdOnBN254Point schnorrVK;
    }

    // === Table State & Stats ===

    /// @notice Look up the balance of `blsVK`
    function lookupStake(BN254.G2Point memory blsVK) external view virtual returns (uint256);
    /// @notice Look up the full `Node` state associated with `blsVK`
    function lookupNode(BN254.G2Point memory blsVK) external view virtual returns (Node memory);

    // === Queuing Stats ===

    /// @notice Get the next available epoch and queue size in that epoch
    function nextRegistrationEpoch() external view virtual returns (uint64, uint64);
    /// @notice Get the number of pending registration requests in the waiting queue
    function numPendingRegistrations() external view virtual returns (uint64);
    /// @notice Get the next available epoch for exit and queue size in that epoch
    function nextExitEpoch() external view virtual returns (uint64, uint64);
    /// @notice Get the number of pending exit requests in the waiting queue
    function numPendingExits() external view virtual returns (uint64);

    // === Write APIs ===

    /// @notice Register a validator in the stake table, transfer of tokens incurred!
    ///
    /// @param blsVK The BLS verification key
    /// @param schnorrVK The Schnorr verification key (as the auxiliary info)
    /// @param amount The amount to register
    /// @param blsSig The BLS signature that authenticates the ethereum account this function is
    /// called from
    /// @param validUntilEpoch The maximum epoch the sender is willing to wait to be included
    /// (cannot be smaller than the current epoch)
    /// @dev No validity check on `schnorrVK`, as it's assumed to be sender's responsibility,
    /// the contract only treat it as auxiliary info submitted by `blsVK`.
    /// @dev `blsSig` field is necessary to prevent "rogue public-key attack".
    /// The signature is over the caller address of the function to ensure that each message is
    /// unique.
    function register(
        BN254.G2Point memory blsVK,
        EdOnBN254.EdOnBN254Point memory schnorrVK,
        uint256 amount,
        BN254.G1Point memory blsSig,
        uint64 validUntilEpoch
    ) external virtual;

    /// @notice Deposit more stakes to registered keys
    ///
    /// @param blsVK The BLS verification key
    /// @param amount The amount to deposit
    /// @return (newBalance, effectiveEpoch) the new balance effective at a future epoch
    function deposit(BN254.G2Point memory blsVK, uint256 amount)
        external
        virtual
        returns (uint256, uint64);

    /// @notice Request to exit from the stake table, not immediately withdrawable!
    ///
    /// @param blsVK The BLS verification key to exit
    function requestExit(BN254.G2Point memory blsVK) external virtual;

    /// @notice Withdraw from the staking pool. Transfers occur! Only successfully exited keys can
    /// withdraw past their `exitEpoch`.
    ///
    /// @param blsVK The BLS verification key to withdraw
    /// @return The total amount withdrawn, equal to `Node.balance` associated with `blsVK`
    function withdrawFunds(BN254.G2Point memory blsVK) external virtual returns (uint256);
}
