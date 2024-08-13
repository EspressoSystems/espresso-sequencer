pragma solidity ^0.8.20;

import { Script } from "forge-std/Script.sol";

import { Defender, ApprovalProcessResponse } from "openzeppelin-foundry-upgrades/Defender.sol";
import { Upgrades, Options } from "openzeppelin-foundry-upgrades/Upgrades.sol";
import { PlonkVerifier as PV } from "../src/libraries/PlonkVerifier.sol";
import { UtilsScript } from "./Utils.s.sol";

contract PlonkVerifierDefenderDeployScript is Script {
    string public contractName = "PlonkVerifier.sol";
    UtilsScript public utils = new UtilsScript();
    uint256 public contractSalt = uint256(vm.envInt("PLONK_VERIFIER_SALT"));

    function run() public returns (address contractAddress, address multisig) {
        ApprovalProcessResponse memory upgradeApprovalProcess = Defender.getDeployApprovalProcess();
        multisig = upgradeApprovalProcess.via;

        if (upgradeApprovalProcess.via == address(0)) {
            revert(
                string.concat(
                    "Deploy approval process with id ",
                    upgradeApprovalProcess.approvalProcessId,
                    " has no assigned address"
                )
            );
        }

        Options memory opts;
        opts.defender.useDefenderDeploy = true;
        opts.defender.skipLicenseType = true;
        opts.defender.salt = bytes32(abi.encodePacked(contractSalt));

        contractAddress = Defender.deployContract(contractName, opts.defender);

        //generate the file path, file output and write to the file
        (string memory filePath, string memory fileData) = utils.generateDeploymentOutput(
            contractName,
            contractSalt,
            contractAddress,
            multisig,
            upgradeApprovalProcess.approvalProcessId,
            upgradeApprovalProcess.viaType
        );
        utils.writeJson(filePath, fileData);

        //generate the salt history file path,  output and write to the file
        (string memory saltFilePath, string memory saltFileData) =
            utils.generateSaltOutput(contractName, contractSalt);
        utils.writeJson(saltFilePath, saltFileData);

        return (contractAddress, multisig);
    }
}

contract PlonkVerifierDeployScript is Script {
    string public contractName = "PlonkVerifier.sol";

    function run() public returns (address contractAddress) {
        uint256 deployerPrivateKey = vm.envUint("DEPLOYER_PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);

        // Deploy the library
        Options memory opts;
        address plonkVeriifer = Upgrades.deployImplementation(contractName, opts);

        vm.stopBroadcast();

        return (plonkVeriifer);
    }
}
