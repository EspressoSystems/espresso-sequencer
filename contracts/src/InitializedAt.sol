// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.7 <0.9.0;

// Upgradable contracts using Initializable emit an Initialized event:
// https://github.com/OpenZeppelin/openzeppelin-contracts/blob/930598edfb6241a179ade6ad44ba59ed8b68f7d8/contracts/proxy/utils/Initializable.sol#L92C11-L92C22
//
// This contract does the equivalent inside the constructor for contracts that
// aren't Upgrabable. We do this so that clients can query for this event and
// obtain the block number when the event was emitted. The clients then know
// what block they need to query other contract events from.

contract InitializedAt {
    event Initialized(uint64);

    constructor() {
        // The value 1 is emitted to match the Initializable implementation from openzeppelin
        emit Initialized(1);
    }
}
