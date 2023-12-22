// SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.8.0;

import { LightClient as LC } from "../../src/LightClient.sol";
import { BN254 } from "bn254/BN254.sol";

/// @dev A helper that wraps LightClient contract for testing
contract LightClientTest is LC {
    constructor(LC.LightClientState memory genesis, uint32 numBlockPerEpoch)
        LC(genesis, numBlockPerEpoch)
    { }

    /// @dev Directly mutate `currentEpoch` variable for test
    function setCurrentEpoch(uint64 newEpoch) public {
        currentEpoch = newEpoch;
    }
}
