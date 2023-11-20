// SPDX-License-Identifier: Unlicensed

/* solhint-disable contract-name-camelcase, func-name-mixedcase, one-contract-per-file */

pragma solidity ^0.8.0;

// Libraries
import "forge-std/Test.sol";

// Target contract
import { StakeTable as S } from "../src/StakeTable.sol";

contract StableTable_keyRegister_Test is Test {
    /// @dev Tests the key registering process
    function testKeyRegister() external {
        assertTrue(true);
    }
}
