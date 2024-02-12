// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "forge-std/Script.sol";
import { ERC1967Proxy } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import { HotShot } from "../src/HotShot.sol";
import { LightClient as LC } from "../src/LightClient.sol";

contract DeployHotShotScript is Script {
    function run() external {
        string memory seedPhrase = vm.envString("MNEMONIC");
        uint256 privateKey = vm.deriveKey(seedPhrase, 0);
        vm.startBroadcast(privateKey);

        //// Legacy HotShot contract deployment
        new HotShot();

        vm.stopBroadcast();
    }
}
