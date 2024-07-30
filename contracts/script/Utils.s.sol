pragma solidity ^0.8.20;

import { Script } from "forge-std/Script.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";

contract UtilsScript is Script {
    string public constant SALT_HISTORY_FILE_NAME = "saltHistory.json";
    string public constant DEFENDER_OUTPUT_FILE_PATH =
        "/contracts/script/output/defenderDeployments/";

    function readFile(string memory path) external returns (bool, string memory) {
        if (vm.exists(path)) {
            return (true, vm.readFile(path));
        }
        return (false, "");
    }

    function writeJson(string memory path, string memory output) external {
        vm.writeJson(output, path);
    }

    function createDir(string memory path) public returns (string memory result) {
        if (vm.isDir(path)) {
            return "directory already exists";
        }
        string[] memory cmds = new string[](3);
        cmds[0] = "mkdir";
        cmds[1] = "-p";
        cmds[2] = path;
        return string(vm.ffi(cmds));
    }

    function addrToStr(address addr) external pure returns (string memory) {
        return Strings.toHexString(uint256(uint160(address(addr))));
    }

    function generateProxyDeploymentOutput(
        string memory contractName,
        uint256 contractSalt,
        address proxy,
        address multisig,
        string memory approvalProcessId,
        string memory viaType
    ) external returns (string memory filePath, string memory data) {
        string memory outputDir = string.concat(
            vm.projectRoot(),
            DEFENDER_OUTPUT_FILE_PATH,
            contractName,
            "/",
            vm.toString(block.chainid),
            "/"
        );
        filePath = string.concat(outputDir, Strings.toString(contractSalt), ".json");

        createDir(outputDir);

        string memory obj1 = "object";
        vm.serializeAddress(obj1, "proxyAddress", proxy);
        vm.serializeAddress(obj1, "multisig", multisig);
        vm.serializeString(obj1, "approvalProcessId", approvalProcessId);
        vm.serializeString(obj1, "approvalType", viaType);
        string memory obj3 = vm.serializeUint(obj1, "salt", contractSalt);

        return (filePath, obj3);
    }

    function generateDeploymentOutput(
        string memory contractName,
        uint256 contractSalt,
        address contractAddress,
        address multisig,
        string memory approvalProcessId,
        string memory viaType
    ) external returns (string memory filePath, string memory data) {
        string memory outputDir = string.concat(
            vm.projectRoot(),
            DEFENDER_OUTPUT_FILE_PATH,
            contractName,
            "/",
            vm.toString(block.chainid),
            "/"
        );
        filePath = string.concat(outputDir, Strings.toString(contractSalt), ".json");

        createDir(outputDir);

        string memory obj1 = "object";
        vm.serializeAddress(obj1, "contractAddress", contractAddress);
        vm.serializeAddress(obj1, "multisig", multisig);
        vm.serializeString(obj1, "approvalProcessId", approvalProcessId);
        vm.serializeString(obj1, "approvalType", viaType);
        string memory obj3 = vm.serializeUint(obj1, "salt", contractSalt);

        return (filePath, obj3);
    }

    function generateUpgradeOutput(
        string memory originalContractName,
        uint256 contractSalt,
        string memory newContractName,
        address proxy,
        address multisig,
        string memory proposalId,
        string memory responseUrl
    ) external returns (string memory filePath, string memory data) {
        string memory outputDir = string.concat(
            vm.projectRoot(),
            DEFENDER_OUTPUT_FILE_PATH,
            originalContractName,
            "/",
            vm.toString(block.chainid),
            "/"
        );
        filePath = string.concat(outputDir, Strings.toString(contractSalt), ".json");

        (filePath, outputDir) = generateDeploymentFilePath(originalContractName, contractSalt);

        createDir(outputDir);

        string memory obj1 = "object";
        vm.serializeAddress(obj1, "proxyAddress", proxy);
        vm.serializeAddress(obj1, "multisig", multisig);
        vm.serializeString(obj1, "newContractName", newContractName);
        vm.serializeString(obj1, "proposalId", proposalId);
        vm.serializeString(obj1, "responseUrl", responseUrl);
        string memory obj3 = vm.serializeUint(obj1, "salt", contractSalt);

        return (filePath, obj3);
    }

    function generateDeploymentFilePath(string memory contractName, uint256 contractSalt)
        public
        view
        returns (string memory filePath, string memory outputDir)
    {
        outputDir = string.concat(
            vm.projectRoot(),
            DEFENDER_OUTPUT_FILE_PATH,
            contractName,
            "/",
            vm.toString(block.chainid),
            "/"
        );
        filePath = string.concat(outputDir, Strings.toString(contractSalt), ".json");

        return (filePath, outputDir);
    }

    function generateSaltOutput(string memory contractName, uint256 contractSalt)
        external
        returns (string memory filePath, string memory data)
    {
        string memory outputDir;
        (filePath, outputDir) = generateSaltFilePath(contractName);

        createDir(outputDir);

        string memory obj1 = "contract";
        vm.serializeString(obj1, "contractName", contractName);
        string memory obj2 = vm.serializeUint(obj1, "previousSalt", contractSalt);

        return (filePath, obj2);
    }

    function generateSaltFilePath(string memory contractName)
        public
        view
        returns (string memory filePath, string memory outputDir)
    {
        outputDir = string.concat(
            vm.projectRoot(),
            DEFENDER_OUTPUT_FILE_PATH,
            contractName,
            "/",
            vm.toString(block.chainid),
            "/"
        );
        filePath = string.concat(outputDir, SALT_HISTORY_FILE_NAME);

        return (filePath, outputDir);
    }

    function updateEnvVariable(string memory variableName, string memory newValue) external {
        string memory commandValue = string.concat(
            "s/",
            variableName,
            "=",
            vm.envString(variableName),
            "/",
            variableName,
            "=",
            newValue,
            "/"
        );
        string[] memory cmds = new string[](4);
        cmds[0] = "sed";
        cmds[1] = "-i";
        cmds[2] = commandValue;
        cmds[3] = ".env";
        vm.ffi(cmds);
    }
}
