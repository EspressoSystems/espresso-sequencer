// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "forge-std/Script.sol";
import { LightClient as LC } from "../src/LightClient.sol";

contract DeployHotShot is Script {
    function run() external {
        string memory seedPhrase = vm.envString("MNEMONIC");
        uint256 privateKey = vm.deriveKey(seedPhrase, 0);
        vm.startBroadcast(privateKey);

        //// LightClient contract deployment

        // For Decaf there will be only one epoch
        uint32 blocksPerEpoch = type(uint32).max;

        // TODO change when we start using epochs. See
        // https://github.com/EspressoSystems/espresso-sequencer/issues/940
        uint256 stakeTableCapacity = 10;

        // TODO for a production deployment provide the right genesis state
        string[] memory cmds = new string[](4);
        cmds[0] = "diff-test";
        cmds[1] = "mock-genesis";
        cmds[2] = vm.toString(blocksPerEpoch);
        cmds[3] = vm.toString(stakeTableCapacity / 2);

        bytes memory result = vm.ffi(cmds);
        (LC.LightClientState memory state,,) =
            abi.decode(result, (LC.LightClientState, bytes32, bytes32));

        LC.LightClientState memory genesis = state;
        new LC(genesis, blocksPerEpoch);

        vm.stopBroadcast();
    }
}
