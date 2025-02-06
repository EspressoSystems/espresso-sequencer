// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { StakeTable } from "../../src/StakeTable.sol";

contract StakeTableMock is StakeTable {
    constructor(
        address token,
        address lightClientAddress,
        uint64 churnRate,
        uint64 hotShotBlocksPerEpoch
    ) StakeTable(token, lightClientAddress, churnRate, hotShotBlocksPerEpoch) { }

    function setFirstAvailableRegistrationEpoch(uint64 epoch) public {
        registrationEpoch = epoch;
    }

    function setFirstAvailableExitEpoch(uint64 epoch) public {
        exitEpoch = epoch;
    }
}
