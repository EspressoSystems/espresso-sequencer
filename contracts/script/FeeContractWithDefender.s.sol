pragma solidity ^0.8.20;

import { Script } from "forge-std/Script.sol";
import {
    ProposeUpgradeResponse,
    Defender,
    ApprovalProcessResponse
} from "openzeppelin-foundry-upgrades/Defender.sol";
import { Upgrades, Options } from "openzeppelin-foundry-upgrades/Upgrades.sol";
import { FeeContract as FC } from "../src/FeeContract.sol";

contract FeeContractDefenderDeployScript is Script {
    function run() public returns (address payable proxy, address multisig) {
        bytes32 contractSalt = bytes32(abi.encodePacked(vm.envString("FEE_CONTRACT_SALT")));

        ApprovalProcessResponse memory upgradeApprovalProcess = Defender.getUpgradeApprovalProcess();
        multisig = upgradeApprovalProcess.via;

        vm.startBroadcast(multisig);

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

        proxy = payable(
            Upgrades.deployUUPSProxy(
                "FeeContract.sol", abi.encodeCall(FC.initialize, (multisig)), opts
            )
        );

        vm.stopBroadcast();

        return (proxy, multisig);
    }
}

contract FeeContractDefenderUpgradeScript is Script {
    function run() public returns (string memory proposalId, string memory proposalUrl) {
        bytes32 contractSalt = bytes32(abi.encodePacked(vm.envString("FEE_CONTRACT_SALT")));
        Options memory opts;
        opts.defender.useDefenderDeploy = true;
        opts.defender.salt = contractSalt;
        address proxyAddress = 0x61B4C96475B99A6ce01AfF0da7910605D048c125;
        string memory newContractName = "FeeContract.sol";

        ProposeUpgradeResponse memory response =
            Defender.proposeUpgrade(proxyAddress, newContractName, opts);

        return (response.proposalId, response.url);
    }
}
