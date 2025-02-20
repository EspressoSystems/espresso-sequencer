// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

// Store the block number when a contract was deployed, or initialized (for upgradable contracts).
//
// Clients can read the member variable `initializedAtBlock` to know at what L1 block they need to
// start processing events.

import { Initializable } from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

contract InitializedAt is Initializable {
    // @notice The block number the contract was initialized at.
    uint256 public initializedAtBlock;

    // @dev If this contract is used as part of a non-upgradable contract the constructor is
    // expected to be called.
    constructor() {
        initialize();
    }

    // @dev If this contract is used as part of an upgradable contract the `initialize` function
    // must be called during initialization.
    // @dev The initializer modifier assures that this function can only be called (once except for
    //      in the constructor, for testing).
    function initialize() public initializer {
        initializedAtBlock = block.number;
    }
}
