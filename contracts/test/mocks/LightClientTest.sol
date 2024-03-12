// SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.8.0;

import { LightClient as LC } from "../../src/LightClient.sol";
import { IPlonkVerifier } from "../../src/interfaces/IPlonkVerifier.sol";
import { PlonkVerifier } from "../../src/libraries/PlonkVerifier.sol";
import { LightClientStateUpdateVKTest as VkLib } from "./LightClientStateUpdateVKTest.sol";

/// @dev A helper that wraps LightClient contract for testing
contract LightClientTest is LC {
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
        uint256[] memory publicInput = preparePublicInput(state);

        if (!PlonkVerifier.verify(vk, publicInput, proof, bytes(""))) {
            revert InvalidProof();
        }
    }
}
