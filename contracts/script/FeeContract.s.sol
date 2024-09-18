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

/// @notice Deploys an upgradeable Fee Contract using the OpenZeppelin Upgrades plugin.
contract DeployFeeContractScript is Script {
    string internal contractName = vm.envString("FEE_CONTRACT_ORIGINAL_NAME");

    /// @dev Deploys both the proxy and the implementation contract.
    /// The proxy admin is set as the owner of the contract upon deployment.
    /// The `owner` parameter should be the address of the multisig wallet to ensure proper
    /// ownership management.
    /// @param owner The address that will be set as the owner of the proxy (typically a multisig
    /// wallet).
    function run(address owner)
        public
        returns (address payable proxy, address implementationAddress)
    {
        string memory seedPhrase = vm.envString("DEPLOYER_MNEMONIC");
        uint32 seedPhraseOffset = uint32(vm.envUint("DEPLOYER_MNEMONIC_OFFSET"));
        (address admin,) = deriveRememberKey(seedPhrase, seedPhraseOffset);
        vm.startBroadcast(admin);

        address proxyAddress =
            Upgrades.deployUUPSProxy(contractName, abi.encodeCall(FC.initialize, (owner)));

        // Get the implementation address
        implementationAddress = Upgrades.getImplementationAddress(proxyAddress);

        vm.stopBroadcast();

        return (payable(proxyAddress), implementationAddress);
    }
}

/// @notice Deploys an upgradeable Fee Contract with a hardware wallet and using the OpenZeppelin
/// Upgrades plugin.
contract DeployFeeContractWithHDWalletScript is Script {
    string internal contractName = vm.envString("FEE_CONTRACT_ORIGINAL_NAME");

    /// @dev Deploys both the proxy and the implementation contract.
    /// The proxy admin is set as the owner of the contract upon deployment.
    /// The `owner` parameter should be the address of the multisig wallet to ensure proper
    /// ownership management.
    /// @param owner The address that will be set as the owner of the proxy (typically a multisig
    /// wallet).
    function run(address owner)
        public
        returns (address payable proxy, address implementationAddress)
    {
        address admin = vm.envAddress("DEPLOYER_HARDWARE_WALLET_ADDRESS");
        vm.startBroadcast(admin);

        address proxyAddress =
            Upgrades.deployUUPSProxy(contractName, abi.encodeCall(FC.initialize, (owner)));

        // Get the implementation address
        implementationAddress = Upgrades.getImplementationAddress(proxyAddress);

        vm.stopBroadcast();

        return (payable(proxyAddress), implementationAddress);
    }
}

/// @notice Upgrades the fee contract first by deploying the new implementation
/// and then executing the upgrade via the Safe Multisig wallet using the SAFE SDK.
contract UpgradeFeeContractScript is Script {
    string internal originalContractName = vm.envString("FEE_CONTRACT_ORIGINAL_NAME");
    string internal upgradeContractName = vm.envString("FEE_CONTRACT_UPGRADE_NAME");

    /// @dev This function first deploys the new implementation contract using the deployer wallet.
    /// It then uses the SAFE SDK via an ffi command to perform the upgrade through a Safe Multisig
    /// wallet.
    function run() public returns (address implementationAddress, bytes memory result) {
        Options memory opts;
        opts.referenceContract = originalContractName;

        // validate that the new implementation contract is upgrade safe
        Upgrades.validateUpgrade(upgradeContractName, opts);

        // get the deployer info from the environment and start broadcast as the deployer
        string memory seedPhrase = vm.envString("DEPLOYER_MNEMONIC");
        uint32 seedPhraseOffset = uint32(vm.envUint("DEPLOYER_MNEMONIC_OFFSET"));
        (address admin,) = deriveRememberKey(seedPhrase, seedPhraseOffset);
        vm.startBroadcast(admin);

        // deploy the new implementation contract
        FC implementationContract = new FC();

        vm.stopBroadcast();

        //replace with something like this if there is some initiation function to call
        // abi.encodeWithSignature("setNewField(uint256)", 2);
        bytes memory initData = "";

        // call upgradeToAndCall command so that the proxy can be upgraded to call from the new
        // implementation above and
        // execute the command via the Safe Multisig wallet
        string[] memory cmds = new string[](3);
        cmds[0] = "bash";
        cmds[1] = "-c";
        cmds[2] = string(
            abi.encodePacked(
                "source .env.contracts && ts-node contracts/script/multisigTransactionProposals/safeSDK/upgradeProxy.ts upgradeProxy ",
                vm.toString(vm.envAddress("FEE_CONTRACT_PROXY_ADDRESS")),
                " ",
                vm.toString(address(implementationContract)),
                " ",
                vm.toString(initData)
            )
        );

        result = vm.ffi(cmds);

        return (address(implementationContract), result);
    }
}

/// @notice Deploys an upgradeable Fee Contract using OpenZeppelin Defender.
/// the deployment environment details are set in OpenZeppelin Defender which is
/// identified via the Defender Key and Secret in the environment file
contract DeployFeeContractWithDefenderScript is Script {
    string internal contractName = vm.envString("FEE_CONTRACT_ORIGINAL_NAME");
    UtilsScript internal utils = new UtilsScript();
    uint256 internal contractSalt = uint256(vm.envInt("FEE_CONTRACT_SALT"));

    /// @dev When this function is run, a transaction to deploy the implementation is submitted to
    /// Defender
    /// This transaction must be signed via OpenZeppelin Defender's UI and once it completes
    /// another transaction is available to sign for the deployment of the proxy
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

/// @notice Upgrades the Fee Contract using OpenZeppelin Defender.
/// the deployment environment details are set in OpenZeppelin Defender which is
/// identified via the Defender Key and Secret in the environment file
contract UpgradeFeeContractWithDefenderScript is Script {
    string internal originalContractName = vm.envString("FEE_CONTRACT_ORIGINAL_NAME");
    string internal upgradeContractName = vm.envString("FEE_CONTRACT_UPGRADE_NAME");
    uint256 internal contractSalt = uint256(vm.envInt("FEE_CONTRACT_SALT"));
    UtilsScript internal utils = new UtilsScript();

    /// @dev When this function is run, a transaction to deploy the new implementation is submitted
    /// to Defender
    /// This transaction must be signed via OpenZeppelin Defender's UI and once it completes
    /// another transaction is available to sign to call the upgrade method on the proxy
    function run() public returns (string memory proposalId, string memory proposalUrl) {
        //get the previous salt from the salt history - this assumes there was first a deployment
        // using `DeployFeeContractWithDefenderScript`
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
