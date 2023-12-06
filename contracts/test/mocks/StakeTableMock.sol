// SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.8.0;

import { StakeTable } from "../../src/StakeTable.sol";

/// @dev A helper that wraps LightClient contract for testing
contract StakeTableMock is StakeTable {
    enum QueueType {
        Registration,
        Exit
    }

    constructor(address _tokenAddress, address _lightClientAddress)
        StakeTable(_tokenAddress, _lightClientAddress)
    { }

    function nextEpoch(QueueType queueType) public returns (uint64) {
        if (queueType == QueueType.Registration) {
            return this.nextRegistrationEpoch();
        } else {
            return this.nextExitEpoch();
        }
    }

    function getQueueParameters(QueueType queueType) public view returns (uint64, uint64) {
        uint64 firstAvailableEpoch;
        uint64 pendingRequests;
        if (queueType == QueueType.Registration) {
            (firstAvailableEpoch, pendingRequests) = this.registrationQueue();
        } else {
            (firstAvailableEpoch, pendingRequests) = this.exitQueue();
        }

        return (firstAvailableEpoch, pendingRequests);
    }

    function resetQueue(QueueType queueType) public {
        if (queueType == QueueType.Registration) {
            registrationQueue.firstAvailableEpoch = 0;
            registrationQueue.pendingRequests = 0;
        } else {
            exitQueue.firstAvailableEpoch = 0;
            exitQueue.pendingRequests = 0;
        }
    }
}
