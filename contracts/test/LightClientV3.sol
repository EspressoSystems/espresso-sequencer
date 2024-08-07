// SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.8.0;
pragma experimental ABIEncoderV2;

import { LightClientV2 } from "./LightClientV2.sol";
/// @notice A light client for HotShot consensus. Keeping track of its finalized states in safe,
/// authenticated ways.

contract LightClientV3 is LightClientV2 {
    uint256 public anotherField;

    /// @param _newField   New field amount
    function initializeV3(uint256 _newField) external {
        require(_initializedVersion == 2, "already initialized");
        anotherField = _newField;
        _initializedVersion = 3;
    }

    /// @notice Use this to get the implementation contract version
    function getVersion()
        public
        pure
        virtual
        override
        returns (uint8 majorVersion, uint8 minorVersion, uint8 patchVersion)
    {
        return (3, 0, 0);
    }
}
