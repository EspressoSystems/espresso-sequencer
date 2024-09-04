// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "forge-std/Script.sol";

import { LightClient as LC } from "../src/LightClient.sol";
import { ERC1967Proxy } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract DeployLightClientContractScript is Script {
    function run(uint32 numInitValidators, uint32 stateHistoryRetentionPeriod)
        external
        returns (
            address payable proxyAddress,
            address admin,
            LC.LightClientState memory,
            LC.StakeState memory
        )
    {
        // TODO for a production deployment provide the right genesis state and value

        string[] memory cmds = new string[](3);
        cmds[0] = "diff-test";
        cmds[1] = "mock-genesis";
        cmds[2] = vm.toString(uint256(numInitValidators));

        bytes memory result = vm.ffi(cmds);
        (LC.LightClientState memory state,,, LC.StakeState memory stakeState) =
            abi.decode(result, (LC.LightClientState, bytes32, bytes32, LC.StakeState));

        return deployContract(state, stakeState, stateHistoryRetentionPeriod);
    }

    // function runDemo(uint32 stateHistoryRetentionPeriod)
    //     external
    //     returns (address payable proxyAddress, address admin, LC.LightClientState memory)
    // {
    //     string[] memory cmds = new string[](1);
    //     cmds[0] = "gen-demo-genesis";

    //     bytes memory result = vm.ffi(cmds);
    //     LC.LightClientState memory state = abi.decode(result, (LC.LightClientState));
    //     LC.StakeState memory stakeState = LC.StakeState(
    //         state.threshold,
    //         state.stakeTableBlsKeyComm,
    //         state.stakeTableSchnorrKeyComm,
    //         state.stakeTableAmountComm
    //     );

    //     return deployContract(state, stakeState, stateHistoryRetentionPeriod);
    // }

    /// @notice deploys the impl, proxy & initializes the impl
    /// @return proxyAddress The address of the proxy
    /// @return admin The address of the admin

    function deployContract(
        LC.LightClientState memory state,
        LC.StakeState memory stakeState,
        uint32 stateHistoryRetentionPeriod
    )
        private
        returns (
            address payable proxyAddress,
            address admin,
            LC.LightClientState memory,
            LC.StakeState memory
        )
    {
        string memory seedPhrase = vm.envString("MNEMONIC");
        (admin,) = deriveRememberKey(seedPhrase, 0);
        vm.startBroadcast(admin);

        LC lightClientContract = new LC();

        // Encode the initializer function call
        bytes memory data = abi.encodeWithSignature(
            "initialize((uint64,uint64,uint256,uint256,uint256,uint256,uint256,uint256),(uint256,uint256,uint256,uint256),uint32,address)",
            state,
            stakeState,
            stateHistoryRetentionPeriod,
            admin
        );

        // our proxy
        ERC1967Proxy proxy = new ERC1967Proxy(address(lightClientContract), data);
        vm.stopBroadcast();

        proxyAddress = payable(address(proxy));

        return (proxyAddress, admin, state, stakeState);
    }
}
