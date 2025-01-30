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
    /// @param account the address of the validator
    /// @param registerEpoch epoch when the registration becomes effective.
    /// @param amountDeposited amount deposited when registering the new node.
    event Registered(address account, uint64 registerEpoch, uint256 amountDeposited);

    /// @notice Signals an exit request has been granted.
    /// @param account the address of the validator
    /// @param exitEpoch epoch when the user will be allowed to withdraw its funds.
    event Exit(address account, uint64 exitEpoch);

    /// @notice Signals a deposit to a BLS public key.
    /// @param account the address of the validator
    /// @param amount amount of the deposit
    event Deposit(address account, uint256 amount);

    /// @notice Signals a consensus key update for a validator
    /// @param account the address of the validator
    /// @param newBlsVK the new BLS verification key
    /// @param newSchnorrVK the new Schnorr verification key
    event UpdatedConsensusKeys(
        address account, BN254.G2Point newBlsVK, EdOnBN254.EdOnBN254Point newSchnorrVK
    );

    /// @notice Signals the admin has been updated
    /// @param admin the new admin
    event AdminUpdated(address admin);

    /// @notice Signals the min stake amount has been updated
    /// @param minStakeAmount the new min stake amount
    event MinStakeAmountUpdated(uint256 minStakeAmount);

    /// @notice Signals the max churn rate has been updated
    /// @param maxChurnRate the new max churn rate
    event MaxChurnRateUpdated(uint256 maxChurnRate);

    /// @notice Signals the light client address has been updated
    /// @param lightClientAddress the new light client address
    event LightClientAddressUpdated(address lightClientAddress);

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
    /// @param schnorrVK The Schnorr verification key associated. Used for signing the light client
    /// state.
    /// @param blsVK The BLS verification key associated. Used for consensus voting.
    struct Node {
        address account;
        uint256 balance;
        uint64 registerEpoch;
        uint64 exitEpoch;
        EdOnBN254.EdOnBN254Point schnorrVK;
        BN254.G2Point blsVK;
    }

    // === Table State & Stats ===

    /// @notice Look up the balance of `account`
    function lookupStake(address account) external view virtual returns (uint256);

    /// @notice Look up the full `Node` state associated with `account`
    function lookupNode(address account) external view virtual returns (Node memory);

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
    /// @param blsSig The BLS signature that the caller owns the `blsVK`
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
    /// @param amount The amount to deposit
    /// @return (newBalance, effectiveEpoch) the new balance effective at a future epoch
    function deposit(uint256 amount) external virtual returns (uint256, uint64);

    /// @notice Request to exit from the stake table, not immediately withdrawable!
    ///
    function requestExit() external virtual;

    /// @notice Withdraw from the staking pool. Transfers occur! Only successfully exited keys can
    /// withdraw past their `exitEpoch`.
    ///
    /// @return The total amount withdrawn, equal to `Node.balance` associated with `blsVK`
    function withdrawFunds() external virtual returns (uint256);

    /// @notice Update the consensus keys for a validator
    /// @dev This function is used to update the consensus keys for a validator
    /// @dev This function can only be called by the validator itself when it's not in the exit
    /// queue
    /// @dev The validator will need to give up either its old BLS key and/or old Schnorr key
    /// @dev The validator will need to provide a BLS signature over the new BLS key
    /// @param newBlsVK The new BLS verification key
    /// @param newSchnorrVK The new Schnorr verification key
    /// @param newBlsSig The BLS signature that the account owns the new BLS key
    function updateConsensusKeys(
        BN254.G2Point memory newBlsVK,
        EdOnBN254.EdOnBN254Point memory newSchnorrVK,
        BN254.G1Point memory newBlsSig
    ) external virtual;
}
