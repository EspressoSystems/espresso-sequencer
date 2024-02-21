// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "forge-std/Script.sol";
import { HotShot } from "../src/HotShot.sol";

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
