// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { StakeTable } from "../../src/StakeTable.sol";

contract StakeTableMock is StakeTable {
    constructor(
        address token,
        address lightClientAddress,
        uint64 churnRate,
        uint64 hotShotBlocksPerEpoch
    ) StakeTable(token, lightClientAddress, churnRate, hotShotBlocksPerEpoch) 
    // solhint-disable-next-line no-empty-blocks
    { }

    function setRegistrationEpoch(uint64 epoch) public {
        registrationEpoch = epoch;
    }

    function setExitEpoch(uint64 epoch) public {
        exitEpoch = epoch;
    }

    // Expose the internal function for testing by calling the super implementation
    function mockPushToRegistrationQueue() public {
        super.pushToRegistrationQueue();
    }

    function mockPushToExitQueue() public {
        super.pushToExitQueue();
    }
}
