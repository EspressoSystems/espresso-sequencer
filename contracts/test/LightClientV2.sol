// SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.8.0;
pragma experimental ABIEncoderV2;

import { BN254 } from "bn254/BN254.sol";
import { IPlonkVerifier } from "../src/interfaces/IPlonkVerifier.sol";
import { LightClient } from "../src/LightClient.sol";

/// @notice A light client for HotShot consensus. Keeping track of its finalized states in safe,
/// authenticated ways.
contract LightClientV2 is LightClient {
    /// @notice new field for testing purposes
    /// @dev In order to add a field to LightClientState struct one can: add a new contract variable
    /// that has the new struct type, or put the struct inside a map.
    uint256 public newField;

    struct ExtendedLightClientState {
        uint256 extraField;
    }

    /// @notice mapping to store the extended light client states in order to simplify upgrades
    ExtendedLightClientState public extendedFinalzedState;

    /// @notice Initialize v2
    /// @param _newField   New field amount
    /// @dev the reinitializer modifier is used to reinitialize new versions of a contract and
    /// is called at most once. The modifier has an uint64 argument which indicates the next
    /// contract version.
    /// when the base implementation contract is initialized for the first time, the _initialized
    /// version
    /// is set to 1. Since this is the 2nd implementation, the next contract version is 2.
    function initializeV2(uint256 _newField) external reinitializer(2) {
        newField = _newField;
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
            newState.viewNum <= finalizedState.viewNum
                || newState.blockHeight <= finalizedState.blockHeight
        ) {
            revert OutdatedState();
        }
        // format validity check
        BN254.validateScalarField(newState.blockCommRoot);

        // check plonk proof
        verifyProof(newState, proof);

        // New condition to check w.r.t. LightClient contract V1
        require(newField == 0, "newField can only be set to 0");

        // upon successful verification, update the latest finalized state
        // because newState is in memory and states[finalizedState] is in storage, they have
        // different data handling mechanisms
        // and this each field needs to be assigned individually
        finalizedState = newState;

        extendedFinalzedState.extraField = 2;

        emit NewState(newState.viewNum, newState.blockHeight, newState.blockCommRoot);
    }

    /// @dev Simple getter function for the extended finalized state
    function getExtendedFinalizedState()
        public
        view
        virtual
        returns (ExtendedLightClientState memory)
    {
        return extendedFinalzedState;
    }
}
