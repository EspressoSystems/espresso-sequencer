// SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.8.0;

import { BN254 } from "bn254/BN254.sol";
import { LightClient as LC } from "../../src/LightClient.sol";
import { IPlonkVerifier } from "../../src/interfaces/IPlonkVerifier.sol";
import { PlonkVerifier } from "../../src/libraries/PlonkVerifier.sol";
import { LightClientStateUpdateVKMock as VkLib } from "./LightClientStateUpdateVKMock.sol";

/// @dev A helper that wraps LightClient contract for testing
contract LightClientMock is LC {
    constructor(LC.LightClientState memory genesis, uint32 numBlockPerEpoch) LC() {
        _initializeState(genesis, numBlockPerEpoch);
    }

    /// @dev Directly mutate `currentEpoch` variable for test
    function setCurrentEpoch(uint64 newEpoch) public {
        currentEpoch = newEpoch;
    }

    /// @dev Directly mutate `finalizedState` variable for test
    function setFinalizedState(LC.LightClientState memory state) public {
        states[finalizedState] = state;
    }

    /// @dev override the production-implementation with test VK.
    function verifyProof(LC.LightClientState memory state, IPlonkVerifier.PlonkProof memory proof)
        internal
        view
        override
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
}
