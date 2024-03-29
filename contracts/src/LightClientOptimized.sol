// SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.8.0;
pragma experimental ABIEncoderV2;

import { IPlonkVerifier } from "./interfaces/IPlonkVerifier.sol";
import { PlonkVerifierOptimized } from "./libraries/PlonkVerifierOptimized.sol";
import { LightClientStateUpdateVKTest as VkLibTest } from
    "../test/mocks/LightClientStateUpdateVKTest.sol";
import { LightClient } from "./LightClient.sol";

/// @title Optimized version of Light Client Contract
contract LightClientOptimized is LightClient {
    /// @notice Verify the Plonk proof, marked as `virtual` for easier testing as we can swap VK
    /// used
    /// in inherited contracts.
    function verifyProof(LightClientState memory state, IPlonkVerifier.PlonkProof memory proof)
        internal
        virtual
        override
    {
        // TODO later this key will be hardcoded inside the optimized plonk verifier contract
        IPlonkVerifier.VerifyingKey memory vk = VkLibTest.getVk();
        uint256[] memory publicInput = preparePublicInput(state);

        if (!PlonkVerifierOptimized.verify(vk, publicInput, proof, bytes(""))) {
            revert InvalidProof();
        }
    }
}
