// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "forge-std/Script.sol";

import { FeeContract } from "../src/FeeContract.sol";
import { ERC1967Proxy } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract DeployFeeContract is Script {
    function run() external returns (address payable proxy, address admin) {
        // string memory seedPhrase = vm.envString("MNEMONIC");
        // (address admin,) = deriveRememberKey(seedPhrase, 0);
        (proxy, admin) = deployFeeContract();

        return (proxy, admin);
    }

    /// @notice deploys the impl, proxy & initializes the impl
    /// @return proxyAddress The address of the proxy
    /// @return admin The address of the admin
    function deployFeeContract() public returns (address payable proxyAddress, address admin) {
        vm.startBroadcast();

        FeeContract feeContract = new FeeContract(); //Our implementation(logic).Proxy will point
            // here to delegate

        // Encode the initializer function call
        bytes memory data = abi.encodeWithSelector(
            FeeContract.initialize.selector,
            msg.sender // Initial owner/admin of the contract
        );

        ERC1967Proxy proxy = new ERC1967Proxy(address(feeContract), data);
        vm.stopBroadcast();

        proxyAddress = payable(address(proxy));
        admin = FeeContract(proxyAddress).owner();
        return (proxyAddress, admin);
    }
}
