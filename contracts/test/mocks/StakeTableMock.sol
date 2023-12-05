// SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.8.0;

import { StakeTable } from "../../src/StakeTable.sol";

/// @dev A helper that wraps LightClient contract for testing
contract StakeTableMock is StakeTable {
    constructor(address _tokenAddress, address _lightClientAddress)
        StakeTable(_tokenAddress, _lightClientAddress)
    { }

    function nextEpoch(StakeTable.Queue memory queue) public returns (uint64) {
        return _nextEpoch(queue);
    }
}
