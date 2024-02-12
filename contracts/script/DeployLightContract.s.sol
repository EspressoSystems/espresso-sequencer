// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;
pragma experimental ABIEncoderV2;

import "forge-std/Script.sol";

import { LightClient as LC } from "../src/LightClient.sol";
import { ERC1967Proxy } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract DeployLightClientContractScript is Script {
    /// @notice deploys the impl, proxy & initializes the impl
    /// @return proxyAddress The address of the proxy
    /// @return admin The address of the admin
    function run() external returns (address payable proxyAddress, address admin) {
        string memory seedPhrase = vm.envString("MNEMONIC");
        (admin,) = deriveRememberKey(seedPhrase, 0);
        vm.startBroadcast(admin);

        // Our implementation(logic).Proxy will point here to delegate
        // TODO for a production deployment provide the right genesis state and value
        // numBlocksPerEpoch

        uint32 numBlocksPerEpoch = 10;

        string[] memory cmds = new string[](4);
        cmds[0] = "diff-test";
        cmds[1] = "mock-genesis";
        cmds[2] = vm.toString(numBlocksPerEpoch);
        cmds[3] = vm.toString(uint256(5));

        bytes memory result = vm.ffi(cmds);
        (LC.LightClientState memory state,,) =
            abi.decode(result, (LC.LightClientState, bytes32, bytes32));

        LC lightClientContract = new LC();

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

        return (proxyAddress, admin);
    }
}

//// LightClient contract deployment

// For now there will be only one epoch
//uint32 blocksPerEpoch = type(uint32).max;

// TODO for a production deployment provide the right genesis state
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
