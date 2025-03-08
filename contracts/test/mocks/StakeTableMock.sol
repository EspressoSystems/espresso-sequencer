// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { StakeTable } from "../../src/StakeTable.sol";

contract StakeTableMock is StakeTable {
    constructor(address token, address lightClient, uint256 escrowPeriod, address initialOwner)
        StakeTable(token, lightClient, escrowPeriod, initialOwner)
    // solhint-disable-next-line no-empty-blocks
    { }
}
