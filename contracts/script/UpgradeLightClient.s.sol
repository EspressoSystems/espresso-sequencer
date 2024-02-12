// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import { Script } from "forge-std/Script.sol";
// import {DevOpsTools} from "../lib/foundry-devops/src/DevOpsTools.sol";
import { DemoBoxV2 } from "../demo/upgradeDemo/DemoBoxV2.sol";
import { DemoBoxV1 } from "../demo/upgradeDemo/DemoBoxV1.sol";

import { LightClientV2 as LCV2 } from "../src/LightClientV2.sol";
import { LightClient as LC } from "../src/LightClient.sol";

contract UpgradeLightClientScript is Script {
    /// @notice runs the upgrade
    /// @param mostRecentlyDeployedProxy address of deployed proxy
    /// @return address of the proxy
    /// TODO get th most recent deployment from the devops tooling

    function run(address admin, address mostRecentlyDeployedProxy) external returns (address) {
        LCV2 newLC = new LCV2();
        address proxy = upgradeLightClient(admin, mostRecentlyDeployedProxy, address(newLC));
        return proxy;
    }

    // TODO update documentation
    /// @notice upgrades the box but calling the upgrade function the implementation contract via
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
