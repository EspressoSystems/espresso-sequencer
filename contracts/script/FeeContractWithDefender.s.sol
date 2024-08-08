pragma solidity ^0.8.20;

import { Script } from "forge-std/Script.sol";
import {
    ProposeUpgradeResponse,
    Defender,
    ApprovalProcessResponse
} from "openzeppelin-foundry-upgrades/Defender.sol";
import { Upgrades, Options } from "openzeppelin-foundry-upgrades/Upgrades.sol";
import { FeeContract as FC } from "../src/FeeContract.sol";
import { UtilsScript } from "./Utils.s.sol";

contract FeeContractDeployScript is Script {
    string internal contractName = "FeeContract.sol";
    UtilsScript internal utils = new UtilsScript();
    uint256 internal contractSalt = uint256(vm.envInt("FEE_CONTRACT_SALT"));

    function run(address owner)
        public
        returns (address payable proxy, address implementationAddress)
    {
        uint256 deployerPrivateKey = vm.envUint("DEPLOYER_PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);

        address proxyAddress =
            Upgrades.deployUUPSProxy(contractName, abi.encodeCall(FC.initialize, (owner)));

        // Get the implementation address
        implementationAddress = Upgrades.getImplementationAddress(proxyAddress);

        vm.stopBroadcast();

        return (payable(proxyAddress), implementationAddress);
    }
}

contract FeeContractDefenderDeployScript is Script {
    string internal contractName = "FeeContract.sol";
    UtilsScript internal utils = new UtilsScript();
    uint256 internal contractSalt = uint256(vm.envInt("FEE_CONTRACT_SALT"));

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

        //generate the deployment file path, output and write to the file
        (string memory filePath, string memory fileData) = utils.generateProxyDeploymentOutput(
            contractName, contractSalt, proxyAddress, multisig, approvalProcessId, viaType
        );
        utils.writeJson(filePath, fileData);

        //generate the salt history file path,  output and write to the file
        (string memory saltFilePath, string memory saltFileData) =
            utils.generateSaltOutput(contractName, contractSalt);
        utils.writeJson(saltFilePath, saltFileData);

        return (payable(proxyAddress), multisig);
    }
}

contract FeeContractDefenderUpgradeScript is Script {
    string internal originalContractName = "FeeContract.sol";
    string internal upgradeContractName = vm.envString("FEE_CONTRACT_UPGRADE_NAME");
    uint256 internal contractSalt = uint256(vm.envInt("FEE_CONTRACT_SALT"));
    UtilsScript internal utils = new UtilsScript();

    function run() public returns (string memory proposalId, string memory proposalUrl) {
        //get the previous salt from the salt history - this assumes there was first a deployment
        // using `FeeContractDefenderDeployScript`
        (string memory saltFilePath,) = utils.generateSaltFilePath(originalContractName);
        (, string memory saltData) = utils.readFile(saltFilePath);
        uint256 prevContractSalt = vm.parseJsonUint(saltData, ".previousSalt");

        (string memory filePath,) =
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

        //generate the salt history file path,  output and write to the file
        string memory saltFileData;
        (saltFilePath, saltFileData) = utils.generateSaltOutput(originalContractName, contractSalt);
        utils.writeJson(saltFilePath, saltFileData);

        return (response.proposalId, response.url);
    }
}
