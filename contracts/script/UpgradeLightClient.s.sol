// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import { Script } from "forge-std/Script.sol";

import { LightClient as LCV2 } from "../src/LightClient.sol";
import { LightClient as LC } from "../src/LightClient.sol";

contract UpgradeLightClientScript is Script {
    /// @notice runs the upgrade
    /// @param mostRecentlyDeployedProxy address of deployed proxy
    /// @return address of the proxy
    /// TODO get the most recent deployment from the devops tooling
    function run(address admin, address mostRecentlyDeployedProxy) external returns (address) {
        address proxy = upgradeLightClient(admin, mostRecentlyDeployedProxy, address(new LCV2()));
        return proxy;
    }

    /// @notice upgrades the light client contract by calling the upgrade function the
    /// implementation contract via
    /// the proxy
    /// @param proxyAddress address of proxy
    /// @param newLightClient address of new implementation
    /// @return address of the proxy
    function upgradeLightClient(address admin, address proxyAddress, address newLightClient)
        public
        returns (address)
    {
        LC proxy = LC(proxyAddress); //make the function call on the previous implementation
        vm.prank(admin);

        proxy.upgradeToAndCall(newLightClient, ""); //proxy address now points to the new
            // implementation
        return address(proxy);
    }
}
