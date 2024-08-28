// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "forge-std/Script.sol";

import { LightClientMock as LCMock } from "./mocks/LightClientMock.sol";
import { LightClient as LC } from "../src/LightClient.sol";
import { ERC1967Proxy } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract DeployLightClientTestScript is Script {
    function run(
        uint32 numBlocksPerEpoch, // TODO remove
        uint64 numInitValidators,
        uint32 stateHistoryRetentionPeriod,
        address owner
    ) external returns (address payable proxyAddress, address admin, LC.LightClientState memory) {
        // TODO for a production deployment provide the right genesis state and value

        string[] memory cmds = new string[](4);
        cmds[0] = "diff-test";
        cmds[1] = "mock-genesis";
        cmds[2] = vm.toString(numBlocksPerEpoch); // TODO remove
        cmds[3] = vm.toString(uint256(numInitValidators));

        bytes memory result = vm.ffi(cmds);
        (LC.LightClientState memory state,,) =
            abi.decode(result, (LC.LightClientState, bytes32, bytes32));

        return deployContract(state, stateHistoryRetentionPeriod, owner);
    }

    function runBench(
        uint32 numBlocksPerEpoch,
        uint64 numInitValidators,
        uint32 stateHistoryRetentionPeriod
    ) external returns (address payable, address, LC.LightClientState memory) {
        address payable lcTestProxy;
        address admin;
        LC.LightClientState memory state;
        string memory seedPhrase = vm.envString("MNEMONIC");
        (admin,) = deriveRememberKey(seedPhrase, 0);
        (lcTestProxy, admin, state) =
            this.run(numBlocksPerEpoch, numInitValidators, stateHistoryRetentionPeriod, admin);
        LCMock lc = LCMock(lcTestProxy);
        vm.prank(admin);
        lc.setPermissionedProver(admin);

        return (lcTestProxy, admin, state);
    }

    // TODO remove numBlocksPerEpoch
    function runDemo(uint32 numBlocksPerEpoch, uint32 stateHistoryRetentionPeriod, address owner)
        external
        returns (address payable proxyAddress, address admin, LC.LightClientState memory)
    {
        string[] memory cmds = new string[](1);
        cmds[0] = "gen-demo-genesis";

        bytes memory result = vm.ffi(cmds);
        LC.LightClientState memory state = abi.decode(result, (LC.LightClientState));

        return deployContract(state, stateHistoryRetentionPeriod, owner);
    }

    /// @notice deploys the impl, proxy & initializes the impl
    /// @return proxyAddress The address of the proxy
    /// @return admin The address of the admin
    /// @return the light client state
    function deployContract(
        LC.LightClientState memory state,
        uint32 stateHistoryRetentionPeriod,
        address owner
    ) public returns (address payable proxyAddress, address admin, LC.LightClientState memory) {
        vm.startBroadcast(owner);

        LCMock lightClientContract = new LCMock(state, stateHistoryRetentionPeriod);

        // Encode the initializer function call
        bytes memory data = abi.encodeWithSignature(
            "initialize((uint64,uint64,uint256,uint256,uint256,uint256,uint256,uint256),uint32,uint32,address)",
            state,
            stateHistoryRetentionPeriod,
            owner
        );

        // our proxy
        ERC1967Proxy proxy = new ERC1967Proxy(address(lightClientContract), data);
        vm.stopBroadcast();

        proxyAddress = payable(address(proxy));

        return (proxyAddress, owner, state);
    }
}
