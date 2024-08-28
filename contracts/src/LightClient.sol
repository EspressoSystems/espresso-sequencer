// SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.8.0;

import { OwnableUpgradeable } from
    "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import { Initializable } from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import { UUPSUpgradeable } from
    "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

import { BN254 } from "bn254/BN254.sol";
import { IPlonkVerifier } from "./interfaces/IPlonkVerifier.sol";
import { PlonkVerifier } from "./libraries/PlonkVerifier.sol";
import { LightClientStateUpdateVK as VkLib } from "./libraries/LightClientStateUpdateVK.sol";

/// @title Light Client Contract
/// @notice This contract serves as an always-on client
/// that verifies HotShot's state (Espresso's consensus state) which can be used by
/// Rollup contracts on L1 (Ethereum).
/// This state is submitted by any state-prover with evidence which is
/// a SNARK proof that proves consensus.
/// This contract also keeps track of the current epoch.
/// For this version, the epoch is not used. <br>
/// The light client state primarily consists of:<br>
/// - the merkle root of finalized block commitments,<br>
/// - the fee ledger commitment and <br>
/// - the active stake table commitment<br>
/// @dev You can use this contract to keep track of its finalized states in safe,
/// authenticated ways.
contract LightClient is Initializable, OwnableUpgradeable, UUPSUpgradeable {
    // === Events ===
    //
    // @notice Notify a new epoch is starting
    event EpochChanged(uint64);

    /// @notice upgrade event when the proxy updates the implementation it's pointing to
    event Upgrade(address implementation);

    /// @notice a permissioned prover is needed to interact `newFinalizedState`
    event PermissionedProverRequired(address permissionedProver);

    /// @notice a permissioned prover is no longer needed to interact `newFinalizedState`
    event PermissionedProverNotRequired();

    // === System Parameters ===
    //
    /// @notice number of blocks per epoch
    uint32 public blocksPerEpoch;

    // === Storage ===
    //
    /// @notice current (finalized) epoch number
    uint64 public currentEpoch;

    /// @notice The commitment of the stake table used in current voting (i.e. snapshot at the start
    /// of last epoch)
    bytes32 public votingStakeTableCommitment;

    /// @notice The quorum threshold for the stake table used in current voting
    uint256 public votingThreshold;

    /// @notice The commitment of the stake table frozen for change (i.e. snapshot at the start of
    /// last epoch)
    bytes32 public frozenStakeTableCommitment;

    /// @notice The quorum threshold for the frozen stake table
    uint256 public frozenThreshold;

    /// @notice mapping to store light client states in order to simplify upgrades
    mapping(uint32 index => LightClientState value) public states;

    /// @notice genesis block commitment
    LightClientState public genesisState;

    /// @notice Finalized HotShot's light client state
    LightClientState public finalizedState;

    /// @notice the address of the prover that can call the newFinalizedState function when the
    /// contract is
    /// in permissioned prover mode. This address is address(0) when the contract is not in the
    /// permissioned prover mode
    address public permissionedProver;

    /// @notice a flag that indicates when a permissioned provrer is needed
    bool public permissionedProverEnabled;

    /// @notice Max number of seconds worth of state commitments to record based on this block
    /// timestamp
    uint32 public stateHistoryRetentionPeriod;

    /// @notice index of first block in block state series
    ///@dev use this instead of index 0 since old states would be set to zero to keep storage costs
    /// constant to stateHistoryRetentionPeriod
    uint64 public stateHistoryFirstIndex;

    /// @notice an array to store the L1 block heights, HotShot Block Heights and their respective
    /// state history
    /// commitments
    StateHistoryCommitment[] public stateHistoryCommitments;

    // === Data Structure ===
    //
    /// @notice The finalized HotShot state (as the digest of the entire HotShot state)
    /// @param viewNum The latest view number of the finalized HotShot chain
    /// @param blockHeight The block height of the latest finalized block
    /// @param blockCommRoot The merkle root of historical block commitments (BN254::ScalarField)
    /// @param feeLedgerComm The commitment to the fee ledger state (type: BN254::ScalarField)
    /// @param stakeTableBlsKeyComm The commitment to the BlsVerKey column of the stake table
    /// @param stakeTableSchnorrKeyComm The commitment to the SchnorrVerKey column of the table
    /// @param stakeTableAmountComm The commitment to the stake amount column of the stake table
    /// @param threshold The (stake-weighted) quorum threshold for a QC to be considered as valid
    struct LightClientState {
        uint64 viewNum;
        uint64 blockHeight;
        BN254.ScalarField blockCommRoot;
        BN254.ScalarField feeLedgerComm;
        BN254.ScalarField stakeTableBlsKeyComm;
        BN254.ScalarField stakeTableSchnorrKeyComm;
        BN254.ScalarField stakeTableAmountComm;
        uint256 threshold;
    }

    /// @notice Simplified HotShot commitment struct
    /// @param blockHeight The block height of the latest finalized HotShot block
    /// @param blockCommRoot The merkle root of historical block commitments (BN254::ScalarField)
    struct HotShotCommitment {
        uint64 blockHeight;
        BN254.ScalarField blockCommRoot;
    }

    /// @notice Simplified HotShot commitment struct
    /// @param l1BlockHeight the block height of l1 when this state update was stored
    /// @param l1BlockTimestamp the block timestamp of l1 when this state update was stored
    /// @param hotShotCommitment The HotShot commitment info of the latest finalized HotShot block
    struct StateHistoryCommitment {
        uint64 l1BlockHeight;
        uint64 l1BlockTimestamp;
        HotShotCommitment hotShotCommitment;
    }

    /// @notice Event that a new finalized state has been successfully verified and updated
    event NewState(
        uint64 indexed viewNum, uint64 indexed blockHeight, BN254.ScalarField blockCommRoot
    );

    /// @notice The state is outdated and older than currently known `finalizedState`
    error OutdatedState();
    /// @notice Warning that the last block of the current epoch is not yet submitted before newer
    /// states are proposed.
    error MissingLastBlockForCurrentEpoch(uint64 expectedBlockHeight);
    /// @notice Invalid user inputs: wrong format or non-sensible arguments
    error InvalidArgs();
    /// @notice Wrong plonk proof or public inputs.
    error InvalidProof();
    /// @notice Wrong stake table used, should match `finalizedState`
    error WrongStakeTableUsed();
    /// @notice Invalid address
    error InvalidAddress();
    /// @notice Only a permissioned prover can perform this action
    error ProverNotPermissioned();
    /// @notice If the same mode or prover is sent to the function, then no change is required
    error NoChangeRequired();
    /// @notice Invalid L1 Block for checking Light Client Updates, premature or in the future
    error InsufficientSnapshotHistory();
    /// @notice Invalid HotShot Block for checking HotShot commitments, premature or in the future
    error InvalidHotShotBlockForCommitmentCheck();
    /// @notice Invalid Max Block States
    error InvalidMaxStateHistory();

    /// @notice Constructor disables initializers to prevent the implementation contract from being
    /// initialized
    /// @dev This is standard practice for OpenZeppelin upgradeable contracts. Storage is on the
    /// proxy contract
    /// since it calls this cnotract via delegatecall
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /// @notice This contract is called by the proxy when you deploy this contract
    /// @param _genesis The initial state of the light client
    /// @param _blocksPerEpoch The number of blocks per epoch
    /// @param _stateHistoryRetentionPeriod The maximum retention period (in seconds) for the state
    /// history
    /// @param owner The address of the contract owner
    function initialize(
        LightClientState memory _genesis,
        uint32 _blocksPerEpoch,
        uint32 _stateHistoryRetentionPeriod,
        address owner
    ) public initializer {
        __Ownable_init(owner); //sets owner of the contract
        __UUPSUpgradeable_init();
        _initializeState(_genesis, _blocksPerEpoch, _stateHistoryRetentionPeriod);
    }

    /// @notice Use this to get the implementation contract version
    /// @return majorVersion The major version of the contract
    /// @return minorVersion The minor version of the contract
    /// @return patchVersion The patch version of the contract
    function getVersion()
        public
        pure
        virtual
        returns (uint8 majorVersion, uint8 minorVersion, uint8 patchVersion)
    {
        return (1, 0, 0);
    }

    /// @notice only the owner can authorize an upgrade
    function _authorizeUpgrade(address newImplementation) internal virtual override onlyOwner {
        emit Upgrade(newImplementation);
    }

    /// @dev Initialization of contract variables happens in this method because the LightClient
    /// contract is upgradable and thus has its constructor method disabled.
    /// @param _genesis The initial state of the light client
    /// @param _blockPerEpoch The number of blocks per epoch
    /// @param _stateHistoryRetentionPeriod The maximum retention period (in seconds) for the state
    /// history
    function _initializeState(
        LightClientState memory _genesis,
        uint32 _blockPerEpoch,
        uint32 _stateHistoryRetentionPeriod
    ) internal {
        // stake table commitments and threshold cannot be zero, otherwise it's impossible to
        // generate valid proof to move finalized state forward.
        // Whereas blockCommRoot can be zero, if we use special value zero to denote empty tree.
        // feeLedgerComm can be zero, if we optionally support fee ledger yet.
        if (
            _genesis.viewNum != 0 || _genesis.blockHeight != 0
                || BN254.ScalarField.unwrap(_genesis.stakeTableBlsKeyComm) == 0
                || BN254.ScalarField.unwrap(_genesis.stakeTableSchnorrKeyComm) == 0
                || BN254.ScalarField.unwrap(_genesis.stakeTableAmountComm) == 0
                || _genesis.threshold == 0 || _blockPerEpoch == 0
        ) {
            revert InvalidArgs();
        }
        genesisState = _genesis;
        finalizedState = _genesis;

        currentEpoch = 0;

        blocksPerEpoch = _blockPerEpoch;

        stateHistoryRetentionPeriod = _stateHistoryRetentionPeriod;

        bytes32 initStakeTableComm = computeStakeTableComm(_genesis);
        votingStakeTableCommitment = initStakeTableComm;
        votingThreshold = _genesis.threshold;
        frozenStakeTableCommitment = initStakeTableComm;
        frozenThreshold = _genesis.threshold;

        updateStateHistory(uint64(block.number), uint64(block.timestamp), _genesis);
    }

    // === State Modifying APIs ===
    //
    /// @notice Update the latest finalized light client state. It must be updated
    /// periodically (at least once per epoch), especially an update for the last block for every
    /// epoch has to be submitted
    /// before any newer state can be accepted since the stake table commitments of that block
    /// become the snapshots used for vote verifications later on.
    /// @dev in this version, only a permissioned prover doing the computations
    /// can call this function
    /// @dev the state history for `stateHistoryRetentionPeriod` L1 blocks are also recorded in the
    /// `stateHistoryCommitments` array
    /// @notice While `newState.stakeTable*` refers to the (possibly) new stake table states,
    /// the entire `newState` needs to be signed by stakers in `finalizedState`
    /// @param newState new light client state
    /// @param proof PlonkProof
    function newFinalizedState(
        LightClientState memory newState,
        IPlonkVerifier.PlonkProof memory proof
    ) external virtual {
        //revert if we're in permissionedProver mode and the permissioned prover has not been set
        if (permissionedProverEnabled && msg.sender != permissionedProver) {
            revert ProverNotPermissioned();
        }

        if (
            newState.viewNum <= getFinalizedState().viewNum
                || newState.blockHeight <= getFinalizedState().blockHeight
        ) {
            revert OutdatedState();
        }
        uint64 epochEndingBlockHeight = currentEpoch * blocksPerEpoch;

        // TODO consider saving gas in the case BLOCKS_PER_EPOCH == type(uint32).max
        bool isNewEpoch = finalizedState.blockHeight == epochEndingBlockHeight;
        if (!isNewEpoch && newState.blockHeight > epochEndingBlockHeight) {
            revert MissingLastBlockForCurrentEpoch(epochEndingBlockHeight);
        }
        // format validity check
        BN254.validateScalarField(newState.blockCommRoot);
        BN254.validateScalarField(newState.feeLedgerComm);
        BN254.validateScalarField(newState.stakeTableBlsKeyComm);
        BN254.validateScalarField(newState.stakeTableSchnorrKeyComm);
        BN254.validateScalarField(newState.stakeTableAmountComm);

        // If the newState is in a new epoch, increment the `currentEpoch`, update the stake table.
        if (isNewEpoch) {
            _advanceEpoch();
        }

        // check plonk proof
        verifyProof(newState, proof);

        // upon successful verification, update the latest finalized state
        finalizedState = newState;

        updateStateHistory(uint64(block.number), uint64(block.timestamp), newState);

        emit NewState(newState.viewNum, newState.blockHeight, newState.blockCommRoot);
    }

    /// @dev Simple getter function for the genesis state
    function getGenesisState() public view virtual returns (LightClientState memory) {
        return genesisState;
    }

    /// @dev Simple getter function for the finalized state
    function getFinalizedState() public view virtual returns (LightClientState memory) {
        return finalizedState;
    }

    /// @notice Verify the Plonk proof, marked as `virtual` for easier testing as we can swap VK
    /// used in inherited contracts.
    function verifyProof(LightClientState memory state, IPlonkVerifier.PlonkProof memory proof)
        internal
        virtual
    {
        IPlonkVerifier.VerifyingKey memory vk = VkLib.getVk();

        // Prepare the public input
        uint256[] memory publicInput = new uint256[](8);
        publicInput[0] = votingThreshold;
        publicInput[1] = uint256(state.viewNum);
        publicInput[2] = uint256(state.blockHeight);
        publicInput[3] = BN254.ScalarField.unwrap(state.blockCommRoot);
        publicInput[4] = BN254.ScalarField.unwrap(state.feeLedgerComm);
        publicInput[5] = BN254.ScalarField.unwrap(finalizedState.stakeTableBlsKeyComm);
        publicInput[6] = BN254.ScalarField.unwrap(finalizedState.stakeTableSchnorrKeyComm);
        publicInput[7] = BN254.ScalarField.unwrap(finalizedState.stakeTableAmountComm);

        if (!PlonkVerifier.verify(vk, publicInput, proof)) {
            revert InvalidProof();
        }
    }

    /// @notice Advance to the next epoch (without any precondition check!)
    /// @dev This meant to be invoked only internally after appropriate precondition checks are done
    function _advanceEpoch() internal virtual {
        bytes32 newStakeTableComm = computeStakeTableComm(finalizedState);
        votingStakeTableCommitment = frozenStakeTableCommitment;
        frozenStakeTableCommitment = newStakeTableComm;

        votingThreshold = frozenThreshold;
        frozenThreshold = finalizedState.threshold;

        currentEpoch += 1;
        emit EpochChanged(currentEpoch);
    }

    /// @notice Given the light client state, compute the short commitment of the stake table
    function computeStakeTableComm(LightClientState memory state)
        public
        pure
        virtual
        returns (bytes32)
    {
        return keccak256(
            abi.encodePacked(
                state.stakeTableBlsKeyComm,
                state.stakeTableSchnorrKeyComm,
                state.stakeTableAmountComm
            )
        );
    }

    /// @notice set the permissionedProverMode to true and set the permissionedProver to the
    /// non-zero address provided
    /// @dev this function can also be used to update the permissioned prover once it's a different
    /// address
    function setPermissionedProver(address prover) public virtual onlyOwner {
        if (prover == address(0)) {
            revert InvalidAddress();
        }
        if (prover == permissionedProver) {
            revert NoChangeRequired();
        }
        permissionedProver = prover;
        permissionedProverEnabled = true;
        emit PermissionedProverRequired(permissionedProver);
    }

    /// @notice set the permissionedProverMode to false and set the permissionedProver to address(0)
    /// @dev if it was already disabled (permissioneProverMode == false), then revert with
    function disablePermissionedProverMode() public virtual onlyOwner {
        if (permissionedProverEnabled) {
            permissionedProver = address(0);
            permissionedProverEnabled = false;
            emit PermissionedProverNotRequired();
        } else {
            revert NoChangeRequired();
        }
    }

    /// @notice updates the stateHistoryCommitments array each time a new
    /// finalized state is added to the LightClient contract.
    /// Ensures that the time difference between the most recent and oldest
    /// elements in this array does not exceed the stateHistoryRetentionPeriod (in seconds).
    /// @dev the block timestamp is used to determine if the stateHistoryCommitments array
    /// should be pruned, based on the stateHistoryRetentionPeriod.
    /// @dev a FIFO approach is used to delete elements from the start of the array,
    /// ensuring that only the most recent states are retained within the
    /// stateHistoryRetentionPeriod
    /// @dev the `delete` method does not reduce the array length but resets the value at the
    /// specified index to zero.
    /// the stateHistoryFirstIndex variable acts as an offset to indicate the starting point for
    /// reading the array,
    /// since the length of the array is not reduced even after deletion.
    function updateStateHistory(
        uint64 blockNumber,
        uint64 blockTimestamp,
        LightClientState memory state
    ) internal {
        if (
            stateHistoryCommitments.length != 0
                && stateHistoryCommitments[stateHistoryCommitments.length - 1].l1BlockTimestamp
                    - stateHistoryCommitments[stateHistoryFirstIndex].l1BlockTimestamp
                    >= stateHistoryRetentionPeriod
        ) {
            // The stateHistoryCommitments array has reached the maximum retention period
            // delete the oldest (first) non-empty element to maintain the FIFO structure.
            delete stateHistoryCommitments[stateHistoryFirstIndex];

            // increment the offset to the first non-zero element in the stateHistoryCommitments
            // array
            stateHistoryFirstIndex++;
        }

        // add the L1 Block & HotShot commitment to the genesis state
        stateHistoryCommitments.push(
            StateHistoryCommitment(
                blockNumber,
                blockTimestamp,
                HotShotCommitment(state.blockHeight, state.blockCommRoot)
            )
        );
    }

    /// @notice checks if the state updates lag behind the specified threshold based on the provided
    /// block number.
    /// @param blockNumber The block number to compare against the latest state updates
    /// @param threshold The number of blocks updates to this contract is allowed to lag behind
    /// @return bool returns true if the lag exceeds the threshold; otherwise, false
    function lagOverEscapeHatchThreshold(uint256 blockNumber, uint256 threshold)
        public
        view
        virtual
        returns (bool)
    {
        uint256 updatesCount = stateHistoryCommitments.length;

        // Handling Edge Cases
        // Edgecase 1: The block is in the future or
        // before HotShot was live, allow for at least two updates before considering HotShot live
        if (blockNumber > block.number || updatesCount < 3) {
            revert InsufficientSnapshotHistory();
        }

        uint256 prevBlock;
        bool prevUpdateFound;

        uint256 i = updatesCount - 1;
        while (!prevUpdateFound) {
            if (stateHistoryCommitments[i].l1BlockHeight <= blockNumber) {
                prevUpdateFound = true;
                prevBlock = stateHistoryCommitments[i].l1BlockHeight;
            }

            // We don't consider the lag time for the first two updates
            if (i < 2) {
                break;
            }

            // We've reached the first recorded block
            if (i == stateHistoryFirstIndex) {
                break;
            }
            i--;
        }

        // If no snapshot is found, we don't have enough history stored
        // to tell whether HotShot was down.
        if (!prevUpdateFound) {
            revert InsufficientSnapshotHistory();
        }

        return blockNumber - prevBlock > threshold;
    }

    /// @notice get the HotShot commitment that represents the Merkle root containing the leaf at
    /// the provided HotShot height
    /// @param hotShotBlockHeight the HotShot block height
    /// @return HotShotCommitment the HotShot commitment
    function getHotShotCommitment(uint256 hotShotBlockHeight)
        public
        view
        virtual
        returns (HotShotCommitment memory)
    {
        uint256 commitmentsHeight = stateHistoryCommitments.length;
        if (
            hotShotBlockHeight
                >= stateHistoryCommitments[commitmentsHeight - 1].hotShotCommitment.blockHeight
        ) {
            revert InvalidHotShotBlockForCommitmentCheck();
        }
        for (uint256 i = stateHistoryFirstIndex; i < commitmentsHeight; i++) {
            // The first commitment greater than the provided height is the root of the tree
            // that leaf at that HotShot height
            if (stateHistoryCommitments[i].hotShotCommitment.blockHeight > hotShotBlockHeight) {
                return stateHistoryCommitments[i].hotShotCommitment;
            }
        }

        return stateHistoryCommitments[commitmentsHeight - 1].hotShotCommitment;
    }

    /// @notice get the number of state history commitments
    /// @return uint256 The number of state history commitments
    function getStateHistoryCount() public view returns (uint256) {
        return stateHistoryCommitments.length;
    }

    /// @notice sets the maximum retention period for storing block state history.
    /// @param historySeconds The maximum number of seconds for which state history updates
    /// will be stored, based on the block timestamp. It must be greater than or equal to
    /// the current state history retention period and must be at least 1 hour.
    /// @dev Reverts with `InvalidMaxStateHistory` if the provided value is less than 1 hour
    /// or less than or equal to the current state history retention period.
    function setstateHistoryRetentionPeriod(uint32 historySeconds) public onlyOwner {
        if (historySeconds < 1 hours || historySeconds <= stateHistoryRetentionPeriod) {
            revert InvalidMaxStateHistory();
        }

        stateHistoryRetentionPeriod = historySeconds;
    }
}
