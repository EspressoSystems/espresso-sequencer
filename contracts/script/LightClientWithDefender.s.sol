pragma solidity ^0.8.20;

import { Script } from "forge-std/Script.sol";

import {
    Defender,
    ApprovalProcessResponse,
    ProposeUpgradeResponse
} from "openzeppelin-foundry-upgrades/Defender.sol";
import { Upgrades, Options } from "openzeppelin-foundry-upgrades/Upgrades.sol";
import { LightClient as LC } from "../src/LightClient.sol";
import { UtilsScript } from "./Utils.s.sol";

contract LightClientDefenderDeployScript is Script {
    string public contractName = "LightClient.sol";
    UtilsScript public utils = new UtilsScript();
    uint256 public contractSalt = uint256(vm.envInt("LIGHT_CLIENT_SALT"));

    function run()
        public
        returns (address proxy, address multisig, LC.LightClientState memory state)
    {
        // TODO for a production deployment provide the right genesis state and value
        uint32 numInitValidators = 1;
        uint32 stateHistoryRetentionPeriod = 864000;

        string[] memory cmds = new string[](3);
        cmds[0] = "diff-test";
        cmds[1] = "mock-genesis";
        cmds[2] = vm.toString(uint256(numInitValidators));

        bytes memory result = vm.ffi(cmds);
        LC.StakeState memory stakeState;
        (state,,, stakeState) =
            abi.decode(result, (LC.LightClientState, bytes32, bytes32, LC.StakeState));

        ApprovalProcessResponse memory upgradeApprovalProcess = Defender.getUpgradeApprovalProcess();
        multisig = upgradeApprovalProcess.via;

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

        proxy = Upgrades.deployUUPSProxy(
            contractName,
            abi.encodeCall(
                LC.initialize, (state, stakeState, stateHistoryRetentionPeriod, multisig)
            ),
            opts
        );

        //generate the file path, file output and write to the file
        (string memory filePath, string memory fileData) = utils.generateProxyDeploymentOutput(
            contractName,
            contractSalt,
            proxy,
            multisig,
            upgradeApprovalProcess.approvalProcessId,
            upgradeApprovalProcess.viaType
        );
        utils.writeJson(filePath, fileData);

        //generate the salt history file path,  output and write to the file
        (string memory saltFilePath, string memory saltFileData) =
            utils.generateSaltOutput(contractName, contractSalt);
        utils.writeJson(saltFilePath, saltFileData);

        return (proxy, multisig, state);
    }
}

contract LightClientDefenderUpgradeScript is Script {
    string public originalContractName = "LightClient.sol";
    string public upgradeContractName = vm.envString("LIGHT_CLIENT_UPGRADE_NAME");
    uint256 public contractSalt = uint256(vm.envInt("LIGHT_CLIENT_SALT"));
    UtilsScript public utils = new UtilsScript();

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
        opts.defender.salt = bytes32(contractSalt);
        opts.referenceContract = originalContractName;

        // propose the upgrade via openzeppelin defender
        ProposeUpgradeResponse memory response =
            Defender.proposeUpgrade(proxyAddress, upgradeContractName, opts);
        string memory responseProposalId = response.proposalId;
        string memory responseProposalUrl = response.url;

        //generate the file path, file output (deployment info) and write to the file
        (string memory upgradeFilePath, string memory fileData) = utils.generateUpgradeOutput(
            originalContractName,
            contractSalt,
            upgradeContractName,
            proxyAddress,
            multisig,
            responseProposalId,
            responseProposalUrl
        );
        utils.writeJson(upgradeFilePath, fileData);

        //generate the salt history file path,  output and write to the file
        string memory saltFileData;
        (saltFilePath, saltFileData) = utils.generateSaltOutput(originalContractName, contractSalt);
        utils.writeJson(saltFilePath, saltFileData);

        return (responseProposalId, responseProposalUrl);
    }
}
