// SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.8.0;

import { LightClient } from "./LightClient.sol";
// import { ArbSys } from "./interfaces/ArbSys.sol";

interface ArbSys {
    /**
     * @notice Get Arbitrum block number (distinct from L1 block number)
     * @return block number
     */
    function arbBlockNumber() external view returns (uint256);
}

contract LightClientArbitrum is LightClient {
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function currentBlockNumber() public view virtual override returns (uint256) {
        return ArbSys(address(0x0000000000000000000000000000000000000064)).arbBlockNumber();
    }
}
