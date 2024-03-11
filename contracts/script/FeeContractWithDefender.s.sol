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
        /*TODO
        * use a better salt and get salt from environment variable
        */
        opts.defender.salt = "1";

        proxy = payable(
            Upgrades.deployUUPSProxy(
                "FeeContract.sol", abi.encodeCall(FC.initialize, (multisig)), opts
            )
        );

        vm.stopBroadcast();

        return (proxy, multisig);
    }
}

contract FeeContractUpgradeDefenderScript is Script {
    function run() public {
        Options memory opts;
        opts.defender.useDefenderDeploy = true;
        opts.defender.salt = "1";
        address proxyAddress = 0x61B4C96475B99A6ce01AfF0da7910605D048c125;
        string memory newContractName = "FeeContract.sol";

        Defender.proposeUpgrade(proxyAddress, newContractName, opts);
    }
}
