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
import { LightClientV2 as LCV2 } from "../test/LightClientV2.sol";
import { ERC1967Proxy } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

/// @notice Deploy the upgradeable light client contract using the OpenZeppelin Upgrades plugin.
contract DeployLightClientScript is Script {
    string public contractName = vm.envString("LIGHT_CLIENT_ORIGINAL_CONTRACT_NAME");

    /// @dev Deploys both the proxy and the implementation contract.
    /// The proxy admin is set as the owner of the contract upon deployment.
    /// The `owner` parameter should be the address of the multisig wallet to ensure proper
    /// ownership management.
    /// @param numInitValidators number of the validators initially
    /// @param owner The address that will be set as the owner of the proxy (typically a multisig
    /// wallet).
    function run(uint32 numInitValidators, uint32 stateHistoryRetentionPeriod, address owner)
        public
        returns (
            address proxyAddress,
            address implementationAddress,
            LC.LightClientState memory lightClientState
        )
    {
        string[] memory cmds = new string[](3);
        cmds[0] = "diff-test";
        cmds[1] = "mock-genesis";
        cmds[2] = vm.toString(uint256(numInitValidators));

        bytes memory result = vm.ffi(cmds);
        (LC.LightClientState memory state, LC.StakeTableState memory stakeState) =
            abi.decode(result, (LC.LightClientState, LC.StakeTableState));

        // get the deployer info from the environment and start broadcast as the deployer
        string memory seedPhrase = vm.envString("DEPLOYER_MNEMONIC");
        uint32 seedPhraseOffset = uint32(vm.envUint("DEPLOYER_MNEMONIC_OFFSET"));
        (address admin,) = deriveRememberKey(seedPhrase, seedPhraseOffset);
        vm.startBroadcast(admin);

        proxyAddress = Upgrades.deployUUPSProxy(
            contractName,
            abi.encodeCall(LC.initialize, (state, stakeState, stateHistoryRetentionPeriod, owner))
        );

        // Get the implementation address
        implementationAddress = Upgrades.getImplementationAddress(proxyAddress);

        vm.stopBroadcast();

        return (proxyAddress, implementationAddress, state);
    }
}

contract DeployLightClientWithHDWalletScript is Script {
    string public contractName = vm.envString("LIGHT_CLIENT_ORIGINAL_CONTRACT_NAME");

    /// @dev Deploys both the proxy and the implementation contract.
    /// The proxy admin is set as the owner of the contract upon deployment.
    /// The `owner` parameter should be the address of the multisig wallet to ensure proper
    /// ownership management.
    /// @param numInitValidators number of the validators initially
    /// @param owner The address that will be set as the owner of the proxy (typically a multisig
    /// wallet).
    function run(uint32 numInitValidators, uint32 stateHistoryRetentionPeriod, address owner)
        public
        returns (
            address proxyAddress,
            address implementationAddress,
            LC.LightClientState memory lightClientState
        )
    {
        // get the deployer info from the environment and start broadcast as the deployer
        address admin = vm.envAddress("DEPLOYER_HARDWARE_WALLET_ADDRESS");

        string[] memory cmds = new string[](3);
        cmds[0] = "diff-test";
        cmds[1] = "mock-genesis";
        cmds[2] = vm.toString(uint256(numInitValidators));

        bytes memory result = vm.ffi(cmds);
        (LC.LightClientState memory state, LC.StakeTableState memory stakeState) =
            abi.decode(result, (LC.LightClientState, LC.StakeTableState));

        vm.startBroadcast(admin);

        proxyAddress = Upgrades.deployUUPSProxy(
            contractName,
            abi.encodeCall(LC.initialize, (state, stakeState, stateHistoryRetentionPeriod, owner))
        );

        // Get the implementation address
        implementationAddress = Upgrades.getImplementationAddress(proxyAddress);

        vm.stopBroadcast();

        return (proxyAddress, implementationAddress, state);
    }
}

/// @notice Upgrades the light client contract first by deploying the new implementation
/// and then executing the upgrade via the Safe Multisig wallet using the SAFE SDK.
contract LightClientContractUpgradeScript is Script {
    string internal originalContractName = vm.envString("LIGHT_CLIENT_ORIGINAL_CONTRACT_NAME");
    string internal upgradeContractName = vm.envString("LIGHT_CLIENT_CONTRACT_UPGRADE_NAME");

    /// @dev First the new implementation contract is deployed via the deployer wallet.
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
        LCV2 implementationContract = new LCV2();

        vm.stopBroadcast();

        bytes memory initData = abi.encodeWithSignature("setNewField(uint256)", 2);

        // call upgradeToAndCall command so that the proxy can be upgraded to call from the new
        // implementation above and
        // execute the command via the Safe Multisig wallet
        string[] memory cmds = new string[](3);
        cmds[0] = "bash";
        cmds[1] = "-c";
        cmds[2] = string(
            abi.encodePacked(
                "source .env.contracts && ts-node contracts/script/multisigTransactionProposals/safeSDK/upgradeProxy.ts upgradeProxy ",
                vm.toString(vm.envAddress("LIGHT_CLIENT_CONTRACT_PROXY_ADDRESS")),
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

/// @notice Upgrades the light client contract first by deploying the new implementation
/// and then executing the upgrade via the Safe Multisig wallet using the SAFE SDK.
/// @dev this is used when upgrading to the same base contract file which is being actively modified
/// before mainnet
contract UpgradeLightClientContractWithSameContractScript is Script {
    string internal originalContractName = vm.envString("LIGHT_CLIENT_ORIGINAL_CONTRACT_NAME");
    string internal upgradeContractName = vm.envString("LIGHT_CLIENT_CONTRACT_UPGRADE_NAME");

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
        LC implementationContract = new LC();

        vm.stopBroadcast();

        bytes memory initData = abi.encodeWithSignature("setNewField(uint256)", 2);

        // call upgradeToAndCall command so that the proxy can be upgraded to call from the new
        // implementation above and
        // execute the command via the Safe Multisig wallet
        string[] memory cmds = new string[](3);
        cmds[0] = "bash";
        cmds[1] = "-c";
        cmds[2] = string(
            abi.encodePacked(
                "source .env.contracts && ts-node contracts/script/multisigTransactionProposals/safeSDK/upgradeProxy.ts upgradeProxy ",
                vm.toString(vm.envAddress("LIGHT_CLIENT_CONTRACT_PROXY_ADDRESS")),
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

/// @notice Deploys an upgradeable LightClient Contract using OpenZeppelin Defender.
/// the deployment environment details are set in OpenZeppelin Defender which is
/// identified via the Defender Key and Secret in the environment file
contract DeployLightClientDefenderScript is Script {
    string public contractName = vm.envString("LIGHT_CLIENT_ORIGINAL_CONTRACT_NAME");
    UtilsScript public utils = new UtilsScript();
    uint256 public contractSalt = uint256(vm.envInt("LIGHT_CLIENT_SALT"));

    /// @dev When this function is run, a transaction to deploy the implementation is submitted to
    /// Defender
    /// This transaction must be signed via OpenZeppelin Defender's UI and once it completes
    /// another transaction is available to sign for the deployment of the proxy
    function run(uint32 stateHistoryRetentionPeriod)
        public
        returns (address proxy, address multisig, LC.LightClientState memory lightClientState)
    {
        // TODO for a production deployment provide the right genesis state and value
        uint32 numInitValidators = 1;

        string[] memory cmds = new string[](3);
        cmds[0] = "diff-test";
        cmds[1] = "mock-genesis";
        cmds[3] = vm.toString(uint256(numInitValidators));

        bytes memory result = vm.ffi(cmds);
        (LC.LightClientState memory state, LC.StakeTableState memory stakeState) =
            abi.decode(result, (LC.LightClientState, LC.StakeTableState));

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

/// @notice Upgrades the LightClient Contract using OpenZeppelin Defender.
/// the deployment environment details are set in OpenZeppelin Defender which is
/// identified via the Defender Key and Secret in the environment file

contract UpgradeLightClientWithDefenderScript is Script {
    string public originalContractName = vm.envString("LIGHT_CLIENT_ORIGINAL_CONTRACT_NAME");
    string public upgradeContractName = vm.envString("LIGHT_CLIENT_CONTRACT_UPGRADE_NAME");
    uint256 public contractSalt = uint256(vm.envInt("LIGHT_CLIENT_SALT"));
    UtilsScript public utils = new UtilsScript();

    /// @dev it depends on the `LIGHT_CLIENT_CONTRACT_UPGRADE_NAME` in the environment file to
    /// determine
    /// which implementation it's being upgraded to
    /// When this function is run, a transaction to deploy the new implementation is submitted to
    /// Defender
    /// This transaction must be signed via OpenZeppelin Defender's UI and once it completes
    /// another transaction is available to sign to call the upgrade method on the proxy
    function run() public returns (string memory proposalId, string memory proposalUrl) {
        //get the previous salt from the salt history - this assumes there was first a deployment
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

/// @notice Deploys the upgradable light client contract
/// the admin is not a multisig wallet but is the same as the associated mnemonic
/// used in staging deployments only
contract DeployLightClientContractScript is Script {
    function run(uint32 numInitValidators, uint32 stateHistoryRetentionPeriod)
        external
        returns (
            address payable proxyAddress,
            address admin,
            LC.LightClientState memory,
            LC.StakeTableState memory
        )
    {
        // TODO for a production deployment provide the right genesis state and value

        string[] memory cmds = new string[](3);
        cmds[0] = "diff-test";
        cmds[1] = "mock-genesis";
        cmds[2] = vm.toString(uint256(numInitValidators));

        bytes memory result = vm.ffi(cmds);
        (LC.LightClientState memory state, LC.StakeTableState memory stakeState) =
            abi.decode(result, (LC.LightClientState, LC.StakeTableState));

        return deployContract(state, stakeState, stateHistoryRetentionPeriod);
    }

    /// @notice deploys the impl, proxy & initializes the impl
    /// @return proxyAddress The address of the proxy
    /// @return admin The address of the admin

    function deployContract(
        LC.LightClientState memory state,
        LC.StakeTableState memory stakeState,
        uint32 stateHistoryRetentionPeriod
    )
        private
        returns (
            address payable proxyAddress,
            address admin,
            LC.LightClientState memory,
            LC.StakeTableState memory
        )
    {
        // get the deployer info from the environment and start broadcast as the deployer
        string memory seedPhrase = vm.envString("MNEMONIC");
        uint32 seedPhraseOffset = uint32(vm.envUint("MNEMONIC_OFFSET"));
        (admin,) = deriveRememberKey(seedPhrase, seedPhraseOffset);
        vm.startBroadcast(admin);

        LC lightClientContract = new LC();

        // Encode the initializer function call
        bytes memory data = abi.encodeWithSignature(
            "initialize((uint64,uint64,uint256),(uint256,uint256,uint256,uint256),uint32,address)",
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

/// @notice Upgrades the light client contract first by deploying the new implementation
/// and then calling the upgradeToAndCall method of the proxy
/// @dev This is used when the admin is not a multisig wallet
/// used in staging deployments only
contract UpgradeLightClientWithoutMultisigAdminScript is Script {
    /// @notice runs the upgrade
    /// @param mostRecentlyDeployedProxy address of deployed proxy
    /// @return address of the proxy
    /// TODO get the most recent deployment from the devops tooling
    function run(address mostRecentlyDeployedProxy) external returns (address) {
        // get the deployer info from the environment and start broadcast as the deployer
        string memory seedPhrase = vm.envString("MNEMONIC");
        uint32 seedPhraseOffset = uint32(vm.envUint("MNEMONIC_OFFSET"));
        (address admin,) = deriveRememberKey(seedPhrase, seedPhraseOffset);
        vm.startBroadcast(admin);

        address proxy = upgradeLightClient(mostRecentlyDeployedProxy, address(new LCV2()));
        return proxy;
    }

    /// @notice upgrades the light client contract by calling the upgrade function the
    /// implementation contract via
    /// the proxy
    /// @param proxyAddress address of proxy
    /// @param newLightClient address of new implementation
    /// @return address of the proxy
    function upgradeLightClient(address proxyAddress, address newLightClient)
        public
        returns (address)
    {
        LC proxy = LC(proxyAddress); //make the function call on the previous implementation
        proxy.upgradeToAndCall(newLightClient, ""); //proxy address now points to the new
            // implementation
        vm.stopBroadcast();
        return address(proxy);
    }
}
