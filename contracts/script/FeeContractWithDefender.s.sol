pragma solidity ^0.8.20;

import { Script, console2 } from "forge-std/Script.sol";
import {
    ProposeUpgradeResponse,
    Defender,
    ApprovalProcessResponse
} from "openzeppelin-foundry-upgrades/Defender.sol";
import { Upgrades, Options } from "openzeppelin-foundry-upgrades/Upgrades.sol";
import { FeeContract as FC } from "../src/FeeContract.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import { UtilsScript } from "./Utils.s.sol";

contract FeeContractDefenderDeployScript is Script {
    string contractName = "FeeContract.sol";
    UtilsScript utils = new UtilsScript();
    uint256 contractSalt = uint256(vm.envInt("FEE_CONTRACT_SALT"));

    function run() public returns (address payable proxy, address multisig) {
        ApprovalProcessResponse memory upgradeApprovalProcess = Defender.getUpgradeApprovalProcess();
        multisig = upgradeApprovalProcess.via;
        string memory approvalProcessId = upgradeApprovalProcess.approvalProcessId;
        string memory viaType = upgradeApprovalProcess.viaType;

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
        opts.defender.salt = bytes32(abi.encodePacked(contractSalt));

        address proxyAddress =
            Upgrades.deployUUPSProxy(contractName, abi.encodeCall(FC.initialize, (multisig)), opts);

        //generate the file path, file output and write to the file
        (string memory filePath, string memory fileData) = utils.generateDeploymentOutput(
            contractName, contractSalt, proxyAddress, multisig, approvalProcessId, viaType
        );
        utils.writeJson(filePath, fileData);

        return (payable(proxyAddress), multisig);
    }
}

contract FeeContractDefenderUpgradeScript is Script {
    string originalContractName = "FeeContract.sol";
    string upgradeContractName = vm.envString("FEE_CONTRACT_UPGRADE_NAME");
    uint256 contractSalt = uint256(vm.envInt("FEE_CONTRACT_SALT"));
    UtilsScript utils = new UtilsScript();

    function run() public returns (string memory proposalId, string memory proposalUrl) {
        //assumes each salt is an increment of the previous
        /* TODO 
        * don't assume that the salt is just an increment less, store the previous salts in a file
        */
        uint256 prevContractSalt = contractSalt - 1;
        string memory filePath =
            utils.generateDeploymentFilePath(originalContractName, prevContractSalt);

        //read the deployment file from the previous deployment to get the proxyAddress & multisig
        // used for deployment
        (, string memory result) = utils.readFile(filePath);
        address proxyAddress = vm.parseJsonAddress(result, ".proxyAddress");
        address multisig = vm.parseJsonAddress(result, ".multisig");

        //set openzeppelin defender options for the deployment
        Options memory opts;
        opts.defender.useDefenderDeploy = true;
        opts.defender.salt = bytes32(abi.encodePacked(contractSalt));
        opts.referenceContract = originalContractName;

        //propose the upgrade via openzeppelin defender
        ProposeUpgradeResponse memory response =
            Defender.proposeUpgrade(proxyAddress, upgradeContractName, opts);

        //generate the file path, file output (deployment info) and write to the file
        (string memory upgradeFilePath, string memory fileData) = utils.generateUpgradeOutput(
            originalContractName,
            contractSalt,
            upgradeContractName,
            proxyAddress,
            multisig,
            response.proposalId,
            response.url
        );

        utils.writeJson(upgradeFilePath, fileData);

        return (response.proposalId, response.url);
    }
}
