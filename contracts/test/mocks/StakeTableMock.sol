// SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.8.0;

import { StakeTable } from "../../src/StakeTable.sol";

/// @dev A helper that wraps LightClient contract for testing
contract StakeTableMock is StakeTable {
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
}
