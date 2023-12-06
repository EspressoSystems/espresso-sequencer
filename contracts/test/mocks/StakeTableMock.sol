// SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.8.0;

import { StakeTable } from "../../src/StakeTable.sol";

/// @dev A helper that wraps LightClient contract for testing
contract StakeTableMock is StakeTable {
    /// Enum to be able to distinguish between the two kind of queues
    enum QueueType {
        Registration,
        Exit
    }

    constructor(address _tokenAddress, address _lightClientAddress)
        StakeTable(_tokenAddress, _lightClientAddress)
    { }

    function nextEpoch(QueueType queueType) public returns (uint64) {
        if (queueType == QueueType.Registration) {
            return nextRegistrationEpoch();
        } else {
            return nextExitEpoch();
        }
    }

    function getQueueParameters(QueueType queueType) public view returns (uint64, uint64) {
        if (queueType == QueueType.Registration) {
            return (firstAvailableRegistrationEpoch, pendingRegistrations);
        } else {
            return (firstAvailableExitEpoch, pendingExits);
        }
    }
}
