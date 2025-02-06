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

    function setRegistrationEpoch(uint64 epoch) public {
        registrationEpoch = epoch;
    }

    function setExitEpoch(uint64 epoch) public {
        exitEpoch = epoch;
    }

    // Expose the internal function for testing by calling the super implementation
    function mockPushToRegistrationQueue() public returns (uint64, uint64) {
        return super.pushToRegistrationQueue();
    }

    function mockPushToExitQueue() public returns (uint64, uint64) {
        return super.pushToExitQueue();
    }
}
