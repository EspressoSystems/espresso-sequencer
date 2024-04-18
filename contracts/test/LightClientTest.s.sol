// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "forge-std/Script.sol";
import { BN254 } from "bn254/BN254.sol";
import { LightClient as LC } from "../src/LightClient.sol";
import { LightClientMock as LCTest } from "../test/mocks/LightClientMock.sol";

/// @notice deployment script for LightClientTest which is designed for testing purposes with
/// verification key corresponding to smaller circuit, thus faster proof generation during tests.
///
/// @dev for production deployment script, please use `gen-light-client-deploy/main.rs` to
/// generate `LightClient.s.sol` with proper genesis block values.
contract DeployLightClientTestScript is Script {
    function run(uint32 numBlocksPerEpoch, uint32 numInitValidators) external {
        string memory seedPhrase = vm.envString("MNEMONIC");
        uint256 privateKey = vm.deriveKey(seedPhrase, 0);
        vm.startBroadcast(privateKey);

        string[] memory cmds = new string[](4);
        cmds[0] = "diff-test";
        cmds[1] = "mock-genesis";
        cmds[2] = vm.toString(numBlocksPerEpoch);
        cmds[3] = vm.toString(uint256(numInitValidators));

        bytes memory result = vm.ffi(cmds);
        (LC.LightClientState memory state,,) =
            abi.decode(result, (LC.LightClientState, bytes32, bytes32));

        new LCTest(state, numBlocksPerEpoch);
        vm.stopBroadcast();
    }
}
