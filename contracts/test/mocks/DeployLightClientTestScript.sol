// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "forge-std/Script.sol";

import { LightClientTest as LCTest } from "./LightClientTest.sol";
import { LightClient as LC } from "../../src/LightClient.sol";
import { ERC1967Proxy } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract DeployLightClientTestScript is Script {
    function run(uint32 numBlocksPerEpoch, uint32 numInitValidators)
        external
        returns (address payable proxyAddress, address admin, LCTest.LightClientState memory)
    {
        // TODO for a production deployment provide the right genesis state and value

        string[] memory cmds = new string[](4);
        cmds[0] = "diff-test";
        cmds[1] = "mock-genesis";
        cmds[2] = vm.toString(numBlocksPerEpoch);
        cmds[3] = vm.toString(uint256(numInitValidators));

        bytes memory result = vm.ffi(cmds);
        (LCTest.LightClientState memory state,,) =
            abi.decode(result, (LC.LightClientState, bytes32, bytes32));

        return deployContract(state, numBlocksPerEpoch);
    }

    function runDemo(uint32 numBlocksPerEpoch)
        external
        returns (address payable proxyAddress, address admin, LCTest.LightClientState memory)
    {
        string[] memory cmds = new string[](1);
        cmds[0] = "gen-demo-genesis";

        bytes memory result = vm.ffi(cmds);
        LCTest.LightClientState memory state = abi.decode(result, (LC.LightClientState));

        return deployContract(state, numBlocksPerEpoch);
    }

    /// @notice deploys the impl, proxy & initializes the impl
    /// @return proxyAddress The address of the proxy
    /// @return admin The address of the admin

    function deployContract(LCTest.LightClientState memory state, uint32 numBlocksPerEpoch)
        private
        returns (address payable proxyAddress, address admin, LCTest.LightClientState memory)
    {
        string memory seedPhrase = vm.envString("MNEMONIC");
        (admin,) = deriveRememberKey(seedPhrase, 0);
        vm.startBroadcast(admin);

        LCTest lightClientContract = new LCTest(state, numBlocksPerEpoch);

        // Encode the initializer function call
        bytes memory data = abi.encodeWithSignature(
            "initialize((uint64,uint64,uint256,uint256,uint256,uint256,uint256,uint256),uint32)",
            state,
            numBlocksPerEpoch
        );

        // our proxy
        ERC1967Proxy proxy = new ERC1967Proxy(address(lightClientContract), data);
        vm.stopBroadcast();

        proxyAddress = payable(address(proxy));

        return (proxyAddress, admin, state);
    }
}
