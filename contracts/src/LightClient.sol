// SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.8.0;
pragma experimental ABIEncoderV2;

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

    /// @notice an permissioned prover is no longer needed to interact `newFinalizedState`
    event PermissionedProverNotRequired();

    // === Constants ===
    //
    /// @notice System parameter: number of blocks per epoch

    /// @dev This variable cannot be made immutable due to how UUPS contracts work. See
    /// https://forum.openzeppelin.com/t/upgradable-contracts-instantiating-an-immutable-value/28763/2#why-cant-i-use-immutable-variables-1
    uint32 public blocksPerEpoch;

    /// @notice genesis block commitment index
    uint32 internal genesisState;

    /// @notice Finalized HotShot's light client state index
    uint32 internal finalizedState;

    // === Storage ===

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

    /// @notice the address of the prover that can call the newFinalizedState function when the
    /// contract is
    /// in permissioned prover mode. This address is address(0) when the contract is not in the
    /// permissioned prover mode
    address public permissionedProver;

    /// @notice a flag that indicates when a permissioned provrer is needed
    bool public permissionedProverEnabled;

    /// @notice an array to store the L1 Block Heights where the finalizedState was updated
    uint256[] public stateUpdateBlockNumbers;

    /// @notice an array to store the HotShot Block Heights and their respective HotShot
    /// commitments
    HotShotCommitment[] public hotShotCommitments;

    // === Data Structure ===
    //
    /// @notice The finalized HotShot state (as the digest of the entire HotShot state)
    /// @param viewNum The latest view number of the finalized HotShot chain
    /// @param blockHeight The block height of the latest finalized block
    /// @param blockCommRoot The merkle root of historical block commitments (BN254::ScalarField)
    /// @param feeLedgerComm The commitment to the fee ledger state (type: BN254::ScalarField)
    /// @param stakeTableBlsKeyComm The commitment to the BlsVerKey column of the stake tableg
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
    /// @notice If the contract is in permissioned mode and the permissioned prover is not set when
    /// the newFinalizedState method is called, then revert
    error PermissionedProverNotSet();
    /// @notice If the same mode or prover is sent to the function, then no change is required
    error NoChangeRequired();
    /// @notice Invalid L1 Block for checking Light Client Updates, premature or in the future
    error InvalidL1BlockForStateUpdateCheck();
    /// @notice Invalid HotShot Block for checking HotShot commitments, premature or in the future
    error InvalidHotShotBlockForCommitmentCheck();

    /// @notice since the constructor initializes storage on this contract we disable it
    /// @dev storage is on the proxy contract since it calls this contract via delegatecall
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /// @notice This contract is called by the proxy when you deploy this contract
    function initialize(LightClientState memory genesis, uint32 numBlocksPerEpoch, address owner)
        public
        initializer
    {
        __Ownable_init(owner); //sets owner of the contract
        __UUPSUpgradeable_init();
        genesisState = 0;
        finalizedState = 1;
        _initializeState(genesis, numBlocksPerEpoch);
    }

    /// @notice Use this to get the implementation contract version
    function getVersion()
        public
        pure
        returns (uint8 majorVersion, uint8 minorVersion, uint8 patchVersion)
    {
        return (1, 0, 0);
    }

    /// @notice only the owner can authorize an upgrade
    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {
        emit Upgrade(newImplementation);
    }

    // @dev Initialization of contract variables happens in this method because the LightClient
    // contract is upgradable and thus has its constructor method disabled.
    function _initializeState(LightClientState memory genesis, uint32 numBlockPerEpoch) internal {
        // stake table commitments and threshold cannot be zero, otherwise it's impossible to
        // generate valid proof to move finalized state forward.
        // Whereas blockCommRoot can be zero, if we use special value zero to denote empty tree.
        // feeLedgerComm can be zero, if we optionally support fee ledger yet.
        if (
            genesis.viewNum != 0 || genesis.blockHeight != 0
                || BN254.ScalarField.unwrap(genesis.stakeTableBlsKeyComm) == 0
                || BN254.ScalarField.unwrap(genesis.stakeTableSchnorrKeyComm) == 0
                || BN254.ScalarField.unwrap(genesis.stakeTableAmountComm) == 0 || genesis.threshold == 0
                || numBlockPerEpoch == 0
        ) {
            revert InvalidArgs();
        }
        states[genesisState] = genesis;
        states[finalizedState] = genesis;

        currentEpoch = 0;

        blocksPerEpoch = numBlockPerEpoch;

        bytes32 initStakeTableComm = computeStakeTableComm(genesis);
        votingStakeTableCommitment = initStakeTableComm;
        votingThreshold = genesis.threshold;
        frozenStakeTableCommitment = initStakeTableComm;
        frozenThreshold = genesis.threshold;

        //add the L1 Block to stateUpdateBlockNumbers for the genesis state
        stateUpdateBlockNumbers.push(block.number);

        // add the HotShot commitment for the genesis state
        hotShotCommitments.push(HotShotCommitment(genesis.blockHeight, genesis.blockCommRoot));
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
    ///
    /// @notice While `newState.stakeTable*` refers to the (possibly) new stake table states,
    /// the entire `newState` needs to be signed by stakers in `finalizedState`
    /// @param newState new light client state
    /// @param proof PlonkProof
    function newFinalizedState(
        LightClientState memory newState,
        IPlonkVerifier.PlonkProof memory proof
    ) external {
        //revert if we're in permissionedProver mode and the permissioned prover has not been set
        if (permissionedProverEnabled && msg.sender != permissionedProver) {
            if (permissionedProver == address(0)) {
                revert PermissionedProverNotSet();
            }
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
        bool isNewEpoch = states[finalizedState].blockHeight == epochEndingBlockHeight;
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
        states[finalizedState] = newState;

        /**
         * TODO purge elements from the stateUpdateBlockNumbers array after a decided number of
         * blocks e.g. 14
         * days of blocks
         */
        //add the L1 Block to stateUpdateBlockNumbers for the new finalized state
        stateUpdateBlockNumbers.push(block.number);

        /**
         * TODO purge elements from the hotShotCommitments array after a decided number of blocks
         * e.g. 14 days of blocks
         */
        //add the blockheight and blockCommRoot to hotShotCommitments for the new finalized state
        hotShotCommitments.push(HotShotCommitment(newState.blockHeight, newState.blockCommRoot));

        emit NewState(newState.viewNum, newState.blockHeight, newState.blockCommRoot);
    }

    /// @dev Simple getter function for the genesis state
    function getGenesisState() public view returns (LightClientState memory) {
        return states[genesisState];
    }

    /// @dev Simple getter function for the finalized state
    function getFinalizedState() public view returns (LightClientState memory) {
        return states[finalizedState];
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
        publicInput[5] = BN254.ScalarField.unwrap(states[finalizedState].stakeTableBlsKeyComm);
        publicInput[6] = BN254.ScalarField.unwrap(states[finalizedState].stakeTableSchnorrKeyComm);
        publicInput[7] = BN254.ScalarField.unwrap(states[finalizedState].stakeTableAmountComm);

        if (!PlonkVerifier.verify(vk, publicInput, proof)) {
            revert InvalidProof();
        }
    }

    /// @notice Advance to the next epoch (without any precondition check!)
    /// @dev This meant to be invoked only internally after appropriate precondition checks are done
    function _advanceEpoch() private {
        bytes32 newStakeTableComm = computeStakeTableComm(states[finalizedState]);
        votingStakeTableCommitment = frozenStakeTableCommitment;
        frozenStakeTableCommitment = newStakeTableComm;

        votingThreshold = frozenThreshold;
        frozenThreshold = states[finalizedState].threshold;

        currentEpoch += 1;
        emit EpochChanged(currentEpoch);
    }

    /// @notice Given the light client state, compute the short commitment of the stake table
    function computeStakeTableComm(LightClientState memory state) public pure returns (bytes32) {
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
    function setPermissionedProver(address prover) public onlyOwner {
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
    function disablePermissionedProverMode() public onlyOwner {
        if (permissionedProverEnabled) {
            permissionedProver = address(0);
            permissionedProverEnabled = false;
            emit PermissionedProverNotRequired();
        } else {
            revert NoChangeRequired();
        }
    }

    /// @notice checks the stateUpdateBlockNumbers array and the delayThreshold to determine if
    /// hotshot was
    /// down at a specified L1 block number
    /// @param l1BlockNumber This L1 block number used to reference a point in time when this light
    /// client contract was expected to be updated in a given threshold
    /// @param delayThreshold The delay threshold used to determined if this Light Client contract
    /// was updated in the desired time (measured in blocks)
    function wasL1Updated(uint256 l1BlockNumber, uint256 delayThreshold)
        public
        view
        returns (bool)
    {
        uint256 prevBlock;
        bool prevBlockFound;

        uint256 updatesCount = stateUpdateBlockNumbers.length;

        // Handling Edge Cases
        // Edgecase 1: The block is in the future or in the past before HotShot was live
        if (l1BlockNumber > block.number || updatesCount < 3) {
            revert InvalidL1BlockForStateUpdateCheck();
        }

        for (uint256 i = stateUpdateBlockNumbers.length - 1; i > 1; i--) {
            if (stateUpdateBlockNumbers[i] <= l1BlockNumber) {
                prevBlockFound = true;
                prevBlock = stateUpdateBlockNumbers[i];
                break;
            }
        }

        // If no snapshot is found, we don't have enough history stored to tell whether HotShot was
        // down.
        if (!prevBlockFound) {
            revert InvalidL1BlockForStateUpdateCheck();
        }

        return l1BlockNumber - prevBlock <= delayThreshold;
    }

    /// @notice get the number of L1 block updates
    function getStateUpdateBlockNumbersCount() public view returns (uint256) {
        return stateUpdateBlockNumbers.length;
    }

    /// @notice get the HotShot commitment at the specified block height
    /// @param hotShotBlockHeight hotShotBlockHeight
    function getHotShotCommitment(uint256 hotShotBlockHeight)
        public
        view
        returns (BN254.ScalarField hotShotCommitment)
    {
        uint256 commitmentsHeight = hotShotCommitments.length;
        if (
            hotShotCommitments[0].blockHeight > hotShotBlockHeight
                || hotShotBlockHeight > hotShotCommitments[commitmentsHeight - 1].blockHeight
        ) {
            revert InvalidHotShotBlockForCommitmentCheck();
        }
        for (uint256 i = 1; i < commitmentsHeight; i++) {
            if (hotShotCommitments[i].blockHeight >= hotShotBlockHeight) {
                return hotShotCommitments[i].blockCommRoot;
            }
        }
    }

    /// @notice get the number of HotShot block commitments
    function getHotShotBlockCommitmentsCount() public view returns (uint256) {
        return hotShotCommitments.length;
    }
}
