pragma solidity ^0.8.20;

import { Script } from "forge-std/Script.sol";

import { Defender, ApprovalProcessResponse } from "openzeppelin-foundry-upgrades/Defender.sol";
import { Upgrades, Options } from "openzeppelin-foundry-upgrades/Upgrades.sol";
import { LightClient as LC } from "../src/LightClient.sol";
import { FeeContract as FC } from "../src/FeeContract.sol";

contract LightClientDefenderDeployScript is Script {
    function run()
        public
        returns (address proxy, address admin, LC.LightClientState memory state)
    {
        bytes32 contractSalt = bytes32(abi.encodePacked(vm.envString("LIGHT_CLIENT_SALT")));

        // TODO for a production deployment provide the right genesis state and value
        uint32 numBlocksPerEpoch = 5;
        uint32 numInitValidators = 1;

        string[] memory cmds = new string[](4);
        cmds[0] = "diff-test";
        cmds[1] = "mock-genesis";
        cmds[2] = vm.toString(numBlocksPerEpoch);
        cmds[3] = vm.toString(uint256(numInitValidators));

        bytes memory result = vm.ffi(cmds);
        (state,,) = abi.decode(result, (LC.LightClientState, bytes32, bytes32));

        ApprovalProcessResponse memory upgradeApprovalProcess = Defender.getUpgradeApprovalProcess();
        admin = upgradeApprovalProcess.via;

        vm.startBroadcast(admin);

        if (upgradeApprovalProcess.via == address(0)) {
            revert(
                string.concat(
                    "Upgrade approval process with id ",
                    upgradeApprovalProcess.approvalProcessId,
                    " has no assigned address"
                )
            );
        }

        Options memory opts;
        opts.defender.useDefenderDeploy = true;
        opts.defender.salt = contractSalt;

        proxy = Upgrades.deployUUPSProxy(
            "LightClient.sol", abi.encodeCall(LC.initialize, (state, numBlocksPerEpoch)), opts
        );
        vm.stopBroadcast();

        return (proxy, admin, state);
    }
}

contract DeployUUPSContract {
    function deployUUPSContract(
        LC.LightClientState memory state,
        uint32 numBlocksPerEpoch,
        Options memory opts
    ) external returns (address proxy) {
        proxy = Upgrades.deployUUPSProxy(
            "LightClient.sol", abi.encodeCall(LC.initialize, (state, numBlocksPerEpoch)), opts
        );
        return proxy;
    }
}
