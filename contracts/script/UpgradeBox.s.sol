// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import { Script } from "forge-std/Script.sol";
// import {DevOpsTools} from "../lib/foundry-devops/src/DevOpsTools.sol";
import { BoxV2 } from "../src/upgradeDemo/BoxV2.sol";
import { BoxV1 } from "../src/upgradeDemo/BoxV1.sol";

contract UpgradeBox is Script {
    /// @notice runs the upgrade
    /// @param mostRecentlyDeployedProxy address of deployed proxy
    /// @return address of the proxy
    /// TODO get th most recent deployment from the devops tooling
    function run(address mostRecentlyDeployedProxy) external returns (address) {
        // address mostRecentlyDeployedProxy = DevOpsTools
        //     .get_most_recent_deployment("ERC1967Proxy", block.chainid);

        vm.startBroadcast();
        BoxV2 newAddy = new BoxV2(); //gets the address of the new implementation
        vm.stopBroadcast();
        address proxy = upgradeBox(mostRecentlyDeployedProxy, address(newAddy));
        return proxy;
    }

    /// @notice upgrades the box but calling the upgrade function the implementation contract via
    /// the proxy
    /// @param proxyAddress address of proxy
    /// @param newBox address of new implementation
    /// @return address of the proxy
    function upgradeBox(address proxyAddress, address newBox) public returns (address) {
        vm.startBroadcast();
        BoxV1 proxy = BoxV1(proxyAddress); //make the function call on the previous implementation
            // address

        proxy.upgradeToAndCall(newBox, ""); //proxy address now points to the new implementation
            // address
        vm.stopBroadcast();
        return address(proxy);
    }
}
