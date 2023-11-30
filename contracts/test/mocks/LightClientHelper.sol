// SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.8.0;

import { LightClient } from "../../src/LightClient.sol";

contract LightClientHelper is LightClient {
    constructor(LightClientState memory genesis, uint32 numBlockPerEpoch)
        LightClient(genesis, numBlockPerEpoch)
    { }

    function updateCurrentEpoch(uint64 newEpoch) public {
        currentEpoch = newEpoch;
    }
}
