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

        //// LightClient contract deployment

        // For now there will be only one epoch
        //uint32 blocksPerEpoch = type(uint32).max;

        // TODO for a production deployment provide the right genesis state
        // TODO uncomment and make it work? Or maybe in another file?
        //        string[] memory cmds = new string[](4);
        //        cmds[0] = "diff-test";
        //        cmds[1] = "mock-genesis";
        //        cmds[2] = vm.toString(blocksPerEpoch);
        //        cmds[3] = vm.toString(uint256(5));
        //
        //        bytes memory result = vm.ffi(cmds);
        //        (LC.LightClientState memory state,,) =
        //            abi.decode(result, (LC.LightClientState, bytes32, bytes32));
        //
        //        LC.LightClientState memory genesis = state;
        //        //        LC lightClientContract = new LC();
        //
        //        // Encode the initializer function call
        //        bytes memory data = abi.encodeWithSelector(LC.initialize.selector, genesis,
        // blocksPerEpoch);
        //
        //        // TODO how to test this is really working?
        //        // Proxy
        //        ERC1967Proxy proxy = new ERC1967Proxy(address(lightClientContract), data);

        //// Legacy HotShot contract deployment
        new HotShot();

        vm.stopBroadcast();
    }
}
