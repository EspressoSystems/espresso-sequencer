pragma solidity ^0.8.0;

import { SafeTransferLib, ERC20 } from "solmate/utils/SafeTransferLib.sol";
import { BN254 } from "bn254/BN254.sol";
import { BLSSig } from "./libraries/BLSSig.sol";
import { AbstractStakeTable } from "./interfaces/AbstractStakeTable.sol";
import { LightClient } from "../src/LightClient.sol";
import { EdOnBN254 } from "./libraries/EdOnBn254.sol";

/// @title Implementation of the Stake Table interface
contract StakeTable is AbstractStakeTable {
    /// Error to notify restaking is not implemented yet.
    error RestakingNotImplemented();

    /// Error raised when the registration is aborted because it happens after the user specified
    /// deadline. The first field is the next registration epoch and the second is the last epoch
    /// the user is willing to wait for the registration to happen.
    error InvalidNextRegistrationEpoch(uint64, uint64);

    /// Error raised when a user tries to register another set of keys from the same ethereum
    /// account.
    error NodeAlreadyRegistered();

    /// Error raised when a user tries to withdraw funds from a node that is not registered.
    error NodeNotRegistered();

    /// Error raised when a user tries to make a deposit or request an exit but does not control the
    /// node public key.
    error Unauthenticated();

    /// Error raised when a user tries to deposit before the registration is complete.
    error PrematureDeposit();

    /// Error raised when a user tries to exit before the registration is complete.
    error PrematureExit();

    /// Error raised when a user tries to deposit while an exit request is in progress.
    error ExitRequestInProgress();

    // Error raised when a user tries to withdraw funds before the exit escrow period is over.
    error PrematureWithdrawal();

    // Error raised when this contract does not have the sufficient allowance on the stake ERC20
    // token
    error InsufficientAllowance(uint256, uint256);

    // Error raised when the staker does not have the sufficient balance on the stake ERC20 token
    error InsufficientBalance(uint256);

    // Error raised when the staker does not have the sufficient stake balance to withdraw
    error InsufficientStakeBalance(uint256);

    // Error raised when the staker does not register with the correct stakeAmount
    error InsufficientStakeAmount(uint256);

    // Error raised when the staker does not provide a new key
    error NoKeyChange();

    /// Mapping from a hash of a BLS key to a node struct defined in the abstract contract.
    mapping(address account => Node node) public nodes;

    /// Total stake locked;
    uint256 public totalStake;

    /// Address of the native token contract.
    address public tokenAddress;

    /// Reference to the light client contract.
    LightClient public lightClient;

    /// @notice the first available epoch for registration, please use `nextRegistrationEpoch()` to
    /// get the correct epoch
    uint64 public firstAvailableRegistrationEpoch;
    /// @notice number of pending registrations in the `firstAvailableRegistrationEpoch` (not the
    /// total pending queue size!)
    uint64 private _numPendingRegistrations;

    /// @notice the first available epoch for exit, please use `nextExitEpoch()` to get the correct
    /// epoch
    uint64 public firstAvailableExitEpoch;
    /// @notice number of pending exits in the `firstAvailableExitEpoch` (not the total pending
    /// queue size!)
    uint64 private _numPendingExits;

    uint64 public maxChurnRate;

    constructor(address _tokenAddress, address _lightClientAddress, uint64 churnRate) {
        tokenAddress = _tokenAddress;
        lightClient = LightClient(_lightClientAddress);

        maxChurnRate = churnRate;

        // A set of hardcoded stakers is defined for the first epoch.
        firstAvailableRegistrationEpoch = 1;
        _numPendingRegistrations = 0;

        // It is not possible to exit during the first epoch.
        firstAvailableExitEpoch = 1;
        _numPendingExits = 0;
    }

    /// @dev Computes a hash value of some G2 point.
    /// @param blsVK BLS verification key in G2
    /// @return keccak256(blsVK)
    function _hashBlsKey(BN254.G2Point memory blsVK) public pure returns (bytes32) {
        return keccak256(abi.encode(blsVK.x0, blsVK.x1, blsVK.y0, blsVK.y1));
    }

    /// @dev Compares two BLS keys for equality
    /// @param a First BLS key
    /// @param b Second BLS key
    /// @return True if the keys are equal, false otherwise
    function _isEqualBlsKey(BN254.G2Point memory a, BN254.G2Point memory b)
        public
        pure
        returns (bool)
    {
        return BN254.BaseField.unwrap(a.x0) == BN254.BaseField.unwrap(b.x0)
            && BN254.BaseField.unwrap(a.x1) == BN254.BaseField.unwrap(b.x1)
            && BN254.BaseField.unwrap(a.y0) == BN254.BaseField.unwrap(b.y0)
            && BN254.BaseField.unwrap(a.y1) == BN254.BaseField.unwrap(b.y1);
    }

    /// TODO handle this logic more appropriately when epochs are re-introduced
    /// @dev Fetches the current epoch from the light client contract.
    /// @return current epoch (computed from the current block)
    function currentEpoch() public pure returns (uint64) {
        return 0;
    }

    /// @notice Look up the balance of `account`
    /// @param account account controlled by the user.
    /// @return Current balance owned by the user.
    function lookupStake(address account) external view override returns (uint256) {
        Node memory node = this.lookupNode(account);
        return node.balance;
    }

    /// @notice Look up the full `Node` state associated with `account`
    /// @return Node indexed by account
    function lookupNode(address account) external view override returns (Node memory) {
        return nodes[account];
    }

    /// @notice Get the next available epoch and queue size in that epoch
    /// TODO modify this according to the current spec
    /// TODO modify this according to the current spec
    function nextRegistrationEpoch() external view override returns (uint64, uint64) {
        uint64 epoch;
        uint64 queueSize;

        if (firstAvailableRegistrationEpoch < currentEpoch() + 1) {
            epoch = currentEpoch() + 1;
            queueSize = 0;
        } else if (_numPendingRegistrations >= maxChurnRate) {
            epoch = firstAvailableRegistrationEpoch + 1;
            queueSize = 0;
        } else {
            epoch = firstAvailableRegistrationEpoch;
            queueSize = _numPendingRegistrations;
        }
        return (epoch, queueSize);
    }

    // @notice Update the registration queue
    // @param epoch next available registration epoch
    // @param queueSize current size of the registration queue (after insertion of new element in
    // the queue)
    /// TODO modify this according to the current spec
    /// TODO modify this according to the current spec
    function appendRegistrationQueue(uint64 epoch, uint64 queueSize) private {
        firstAvailableRegistrationEpoch = epoch;
        _numPendingRegistrations = queueSize + 1;
    }

    /// @notice Get the number of pending registration requests in the waiting queue
    /// TODO modify this according to the current spec
    /// TODO modify this according to the current spec
    function numPendingRegistrations() external view override returns (uint64) {
        return _numPendingRegistrations;
    }

    /// @notice Get the next available epoch for exit and queue size in that epoch
    /// TODO modify this according to the current spec
    /// TODO modify this according to the current spec
    function nextExitEpoch() external view override returns (uint64, uint64) {
        uint64 epoch;
        uint64 queueSize;

        if (firstAvailableExitEpoch < currentEpoch() + 1) {
            epoch = currentEpoch() + 1;
            queueSize = 0;
        } else if (_numPendingExits >= maxChurnRate) {
            epoch = firstAvailableExitEpoch + 1;
            queueSize = 0;
        } else {
            epoch = firstAvailableExitEpoch;
            queueSize = _numPendingExits;
        }
        return (epoch, queueSize);
    }

    // @notice Update the exit queue
    // @param epoch next available exit epoch
    // @param queueSize current size of the exit queue (after insertion of new element in the queue)
    /// TODO modify this according to the current spec
    /// TODO modify this according to the current spec
    function appendExitQueue(uint64 epoch, uint64 queueSize) private {
        firstAvailableExitEpoch = epoch;
        _numPendingExits = queueSize + 1;
    }

    /// @notice Get the number of pending exit requests in the waiting queue
    /// TODO modify this according to the current spec
    /// TODO modify this according to the current spec
    function numPendingExits() external view override returns (uint64) {
        return _numPendingExits;
    }

    /// @notice Defines the exit escrow period for a node.
    /// TODO discuss Alex, Jeb. How much do we want to specify this function? Also marked as public
    /// for easier testing.
    /// @dev To put this function into context let us consider the following workflow: requestExit
    /// --> (queueing) --> Exited --> (escrow) --> Witdrawable. The first phase is about waiting in
    /// queue due to rate-limiting on exit, the wait is dependent on the exit amount and currently
    /// exit traffic. At the point of "Exited", the node is officially off duty, and stops
    /// participating in consensus.
    ///  The second phase is about slashable security, the wait is dependent only on amount, during
    /// which period cryptographic evidence of misbehavior (e.g. double-voting) might still lead to
    /// the forfeit of stakes. From the point of `Withdrawable` onwards, the staker can freely
    /// withdraw.
    /// @param node node which is assigned an exit escrow period.
    /// @return Number of epochs post exit after which funds can be withdrawn.
    /// TODO modify this according to the current spec
    /// TODO modify this according to the current spec
    function exitEscrowPeriod(Node memory node) public pure returns (uint64) {
        if (node.balance > 100) {
            return 10;
        } else {
            return 5;
        }
    }

    /// @notice Register a validator in the stake table, transfer of tokens incurred!
    ///
    /// @param blsVK The BLS verification key
    /// @param schnorrVK The Schnorr verification key (as the auxiliary info)
    /// @param amount The amount to register
    /// @param blsSig The BLS signature that authenticates the ethereum account this function is
    /// called from
    /// @param validUntilEpoch The maximum epoch the sender is willing to wait to be included
    /// (cannot be smaller than the current epoch)
    ///
    /// @dev The function will revert if the sender does not have the correct stake amount.
    /// @dev The function will revert if the sender does not have the correct allowance.
    /// @dev The function will revert if the sender does not have the correct balance.
    /// @dev The function will revert if the sender does not have the correct BLS signature.
    /// `blsSig` field is necessary to prevent "rogue public-key attack".
    /// The signature is over the caller address of the function to ensure that each message is
    /// unique.
    /// @dev No validity check on `schnorrVK`, as it's assumed to be sender's responsibility,
    /// the contract only treat it as auxiliary info submitted by `blsVK`.
    /// @dev The function will revert if the sender does not have the correct registration epoch.
    function register(
        BN254.G2Point memory blsVK,
        EdOnBN254.EdOnBN254Point memory schnorrVK,
        uint256 amount,
        BN254.G1Point memory blsSig,
        uint64 validUntilEpoch
    ) external override {
        uint256 fixedStakeAmount = minStakeAmount();

        // Verify that the sender amount is the minStakeAmount
        if (amount < fixedStakeAmount) {
            revert InsufficientStakeAmount(amount);
        }

        Node memory node = nodes[msg.sender];

        // Verify that the node is not already registered.
        if (node.account != address(0x0)) {
            revert NodeAlreadyRegistered();
        }

        // Verify that this contract has permissions to access the validator's stake token.
        uint256 allowance = ERC20(tokenAddress).allowance(msg.sender, address(this));
        if (allowance < fixedStakeAmount) {
            revert InsufficientAllowance(allowance, fixedStakeAmount);
        }

        // Verify that the validator has the balance for this stake token.
        uint256 balance = ERC20(tokenAddress).balanceOf(msg.sender);
        if (balance < fixedStakeAmount) {
            revert InsufficientBalance(balance);
        }

        // Verify that the validator can sign for that blsVK
        bytes memory message = abi.encode(msg.sender);
        BLSSig.verifyBlsSig(message, blsSig, blsVK);

        // Find the earliest epoch at which this node can register. Usually, this will be
        // currentEpoch() + 1 (the start of the next full epoch), but in periods of high churn the
        // queue may fill up and it may be later. If the queue is so full that the wait time exceeds
        // the caller's desired maximum wait, abort.
        (uint64 registerEpoch, uint64 queueSize) = this.nextRegistrationEpoch();
        if (registerEpoch > validUntilEpoch) {
            revert InvalidNextRegistrationEpoch(registerEpoch, validUntilEpoch);
        }
        appendRegistrationQueue(registerEpoch, queueSize);

        // Transfer the stake amount of ERC20 tokens from the sender to this contract.
        SafeTransferLib.safeTransferFrom(
            ERC20(tokenAddress), msg.sender, address(this), fixedStakeAmount
        );

        // Update the total staked amount
        totalStake += fixedStakeAmount;

        // Create an entry for the node.
        node.account = msg.sender;
        node.balance = fixedStakeAmount;
        node.blsVK = blsVK;
        node.schnorrVK = schnorrVK;
        node.registerEpoch = registerEpoch;

        nodes[msg.sender] = node;

        emit Registered(msg.sender, registerEpoch, fixedStakeAmount);
    }

    /// @notice Deposit more stakes to registered keys
    /// @dev TODO this implementation will be revisited later. See
    /// https://github.com/EspressoSystems/espresso-sequencer/issues/806
    /// @dev TODO modify this according to the current spec
    /// @param amount The amount to deposit
    /// @return (newBalance, effectiveEpoch) the new balance effective at a future epoch
    function deposit(uint256 amount) external override returns (uint256, uint64) {
        Node memory node = nodes[msg.sender];

        // if the node is not registered, revert
        if (node.account == address(0)) {
            revert NodeNotRegistered();
        }

        // The deposit must come from the node's registered account.
        if (node.account != msg.sender) {
            revert Unauthenticated();
        }

        // A node cannot deposit more tokens while it waiting to register.
        uint64 _currentEpoch = currentEpoch();
        if (_currentEpoch <= node.registerEpoch) {
            revert PrematureDeposit();
        }

        // A node cannot deposit more tokens if an exit request is in progress.
        if (node.exitEpoch != 0) {
            revert ExitRequestInProgress();
        }

        nodes[msg.sender].balance += amount;
        SafeTransferLib.safeTransferFrom(ERC20(tokenAddress), msg.sender, address(this), amount);

        emit Deposit(msg.sender, uint256(amount));

        uint64 effectiveEpoch = _currentEpoch + 1;

        return (nodes[msg.sender].balance, effectiveEpoch);
    }

    /// @notice Request to exit from the stake table, not immediately withdrawable!
    ///
    /// @dev TODO modify this according to the current spec
    function requestExit() external override {
        Node memory node = nodes[msg.sender];

        // if the node is not registered, revert
        if (node.account == address(0)) {
            revert NodeNotRegistered();
        }

        // The exit request must come from the node's withdrawal account.
        if (node.account != msg.sender) {
            revert Unauthenticated();
        }

        // Cannot request to exit if an exit request is already in progress.
        if (node.exitEpoch != 0) {
            revert ExitRequestInProgress();
        }

        // Cannot exit before becoming an active participant. Activation happens one epoch after the
        // node's registration epoch, due to the consensus-imposed activation waiting period.
        if (currentEpoch() < node.registerEpoch + 1) {
            revert PrematureExit();
        }

        // Prepare the node to exit.
        (uint64 exitEpoch, uint64 queueSize) = this.nextExitEpoch();
        nodes[msg.sender].exitEpoch = exitEpoch;

        appendExitQueue(exitEpoch, queueSize);

        emit Exit(msg.sender, exitEpoch);
    }

    /// @notice Withdraw from the staking pool. Transfers occur! Only successfully exited keys can
    /// withdraw past their `exitEpoch`.
    ///
    /// @return The total amount withdrawn, equal to `Node.balance` associated with `blsVK`
    function withdrawFunds() external override returns (uint256) {
        Node memory node = nodes[msg.sender];

        // Verify that the node is already registered.
        if (node.account == address(0)) {
            revert NodeNotRegistered();
        }

        // The exit request must come from the node's withdrawal account.
        if (node.account != msg.sender) {
            revert Unauthenticated();
        }

        // Verify that the balance is greater than zero
        uint256 balance = node.balance;
        if (balance == 0) {
            revert InsufficientStakeBalance(0);
        }

        // Verify that the exit escrow period is over.
        if (currentEpoch() < node.exitEpoch + exitEscrowPeriod(node)) {
            revert PrematureWithdrawal();
        }

        // Delete the node from the stake table.
        delete nodes[msg.sender];

        // Transfer the balance to the node's account.
        SafeTransferLib.safeTransfer(ERC20(tokenAddress), node.account, balance);

        return balance;
    }

    /// @notice Update the consensus keys for a validator
    /// @dev This function is used to update the consensus keys for a validator
    /// @dev This function can only be called by the validator itself when it's not in the exit
    /// queue
    /// @dev The validator will need to give up either its old BLS key and/or old Schnorr key
    /// @dev The validator will need to provide a BLS signature over the new BLS key
    /// @param currBlsVK The current BLS verification key
    /// @param newBlsVK The new BLS verification key
    /// @param newSchnorrVK The new Schnorr verification key
    /// @param newBlsSig The BLS signature that the account owns the new BLS key
    /// TODO: This function should be tested
    function updateConsensusKeys(
        BN254.G2Point memory currBlsVK,
        BN254.G2Point memory newBlsVK,
        EdOnBN254.EdOnBN254Point memory newSchnorrVK,
        BN254.G1Point memory newBlsSig
    ) external override {
        Node memory node = nodes[msg.sender];

        // Verify that the node is already registered.
        if (node.account == address(0)) {
            revert NodeNotRegistered();
        }

        // Verify that the node is not in the exit queue
        if (node.exitEpoch != 0) {
            revert ExitRequestInProgress();
        }

        // The staker does not provide a key change
        if (
            (_isEqualBlsKey(newBlsVK, currBlsVK) && EdOnBN254.isEqual(newSchnorrVK, node.schnorrVK))
                || (
                    _isEqualBlsKey(
                        newBlsVK,
                        BN254.G2Point(
                            BN254.BaseField.wrap(0),
                            BN254.BaseField.wrap(0),
                            BN254.BaseField.wrap(0),
                            BN254.BaseField.wrap(0)
                        )
                    ) && EdOnBN254.isEqual(newSchnorrVK, EdOnBN254.EdOnBN254Point(0, 0))
                )
        ) {
            revert NoKeyChange();
        }

        // Update the node's schnorr key once it's not the same as the old one and it's nonzero
        if (
            !EdOnBN254.isEqual(newSchnorrVK, node.schnorrVK)
                && !EdOnBN254.isEqual(newSchnorrVK, EdOnBN254.EdOnBN254Point(0, 0))
        ) {
            node.schnorrVK = newSchnorrVK;
        }

        // Update the node's bls key once it's not the same as the old one and it's nonzero
        if (
            !_isEqualBlsKey(newBlsVK, currBlsVK)
                && !_isEqualBlsKey(
                    newBlsVK,
                    BN254.G2Point(
                        BN254.BaseField.wrap(0),
                        BN254.BaseField.wrap(0),
                        BN254.BaseField.wrap(0),
                        BN254.BaseField.wrap(0)
                    )
                )
        ) {
            // Verify that the validator can sign for that blsVK
            bytes memory message = abi.encode(msg.sender);
            BLSSig.verifyBlsSig(message, newBlsSig, newBlsVK);

            // Update the node's bls key
            node.blsVK = newBlsVK;
        }

        // Update the node in the stake table
        nodes[msg.sender] = node;

        // Emit the event
        emit ConsensusKeysUpdated(msg.sender);
    }

    /// @notice Minimum stake amount
    /// @return Minimum stake amount
    /// TODO: This value should be a variable modifiable by admin
    function minStakeAmount() public pure returns (uint256) {
        return 10 ether;
    }
}
