// SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.8.0;

import { LightClient as LC } from "../../src/LightClient.sol";
import { IPlonkVerifier } from "../../src/interfaces/IPlonkVerifier.sol";
import { PlonkVerifier } from "../../src/libraries/PlonkVerifier.sol";
import { BN254 } from "bn254/BN254.sol";
import { LightClientStateUpdateVKTest as VkLib } from "./LightClientStateUpdateVKTest.sol";

/// @dev A helper that wraps LightClient contract for testing
contract LightClientTest is LC {
    constructor(LC.LightClientState memory genesis, uint32 numBlockPerEpoch)
        LC(genesis, numBlockPerEpoch)
    { }

    /// @dev Directly mutate `currentEpoch` variable for test
    function setCurrentEpoch(uint64 newEpoch) public {
        currentEpoch = newEpoch;
    }

    /// @dev override the production-implementation with test VK.
    function verifyProof(
        LC.LightClientState memory state,
        bool isNewEpoch,
        IPlonkVerifier.PlonkProof memory proof
    ) internal override {
        IPlonkVerifier.VerifyingKey memory vk = VkLib.getVk();
        uint256[] memory publicInput = preparePublicInput(state, isNewEpoch);

        if (!PlonkVerifier.verify(vk, publicInput, proof, bytes(""))) {
            revert InvalidProof();
        }
    }
}
