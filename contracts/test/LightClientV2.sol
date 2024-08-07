// SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.8.0;
pragma experimental ABIEncoderV2;

import { BN254 } from "bn254/BN254.sol";
import { IPlonkVerifier } from "../src/interfaces/IPlonkVerifier.sol";
import { PlonkVerifier } from "../src/libraries/PlonkVerifier.sol";
import { LightClientStateUpdateVK as VkLib } from "../src/libraries/LightClientStateUpdateVK.sol";
import { LightClient } from "../src/LightClient.sol";

/// @notice A light client for HotShot consensus. Keeping track of its finalized states in safe,
/// authenticated ways.
contract LightClientV2 is LightClient {
    /// @notice new field for testing purposes
    /// @dev In order to add a field to LightClientState struct one can: add a new contract variable
    /// that has the new struct type, or put the struct inside a map.
    uint256 public newField;

    /// @notice this field is used to check initialized versions so that one can ensure that the
    /// initialization only happens once
    uint8 internal _initializedVersion;

    struct ExtendedLightClientState {
        uint256 extraField;
    }

    /// @notice mapping to store the extended light client states in order to simplify upgrades
    mapping(uint32 index => ExtendedLightClientState state) public extendedStates;

    /// @notice Initialize v2
    /// @param _newField   New field amount
    function initializeV2(uint256 _newField) external {
        require(_initializedVersion == 0);
        newField = _newField;
        _initializedVersion = 2;
    }

    /// @notice Use this to get the implementation contract version
    function getVersion()
        public
        pure
        virtual
        override
        returns (uint8 majorVersion, uint8 minorVersion, uint8 patchVersion)
    {
        return (2, 0, 0);
    }

    // === State Modifying APIs ===
    //
    /// @notice Update the latest finalized light client state. It is expected to be updated
    /// periodically, especially an update for the last block for every epoch has to be submitted
    /// before any newer state can be accepted since the stake table commitments of that block
    /// become the snapshots used for vote verifications later on.
    function newFinalizedState(
        LightClientState memory newState,
        IPlonkVerifier.PlonkProof memory proof
    ) external virtual override {
        if (
            newState.viewNum <= getFinalizedState().viewNum
                || newState.blockHeight <= getFinalizedState().blockHeight
        ) {
            revert OutdatedState();
        }
        uint64 epochEndingBlockHeight = currentEpoch * blocksPerEpoch;

        // TODO consider saving gas in the case BLOCKS_PER_EPOCH == type(uint32).max
        bool isNewEpoch = getFinalizedState().blockHeight == epochEndingBlockHeight;
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

        // New condition to check w.r.t. LightClient contract V1
        require(newField == 0, "newField can only be set to 0");

        // upon successful verification, update the latest finalized state
        // because newState is in memory and states[finalizedState] is in storage, they have
        // different data handling mechanisms
        // and this each field needs to be assigned individually
        states[finalizedState].viewNum = newState.viewNum;
        states[finalizedState].blockHeight = newState.blockHeight;
        states[finalizedState].blockCommRoot = newState.blockCommRoot;
        states[finalizedState].feeLedgerComm = newState.feeLedgerComm;
        states[finalizedState].stakeTableBlsKeyComm = newState.stakeTableBlsKeyComm;
        states[finalizedState].stakeTableSchnorrKeyComm = newState.stakeTableSchnorrKeyComm;
        states[finalizedState].stakeTableAmountComm = newState.stakeTableAmountComm;
        states[finalizedState].threshold = newState.threshold;

        extendedStates[finalizedState].extraField = 2;

        emit NewState(newState.viewNum, newState.blockHeight, newState.blockCommRoot);
    }

    /// @dev Simple getter function for the extended finalized state
    function getExtendedFinalizedState()
        public
        view
        virtual
        returns (ExtendedLightClientState memory)
    {
        return extendedStates[finalizedState];
    }
}
