// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "forge-std/Script.sol";

import { FeeContract } from "../src/FeeContract.sol";
import { ERC1967Proxy } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract DeployFeeContract is Script {
    /// @notice deploys the impl, proxy & initializes the impl
    /// @return proxyAddress The address of the proxy
    /// @return admin The address of the admin
    function run() external returns (address payable proxyAddress, address admin) {
        // string memory seedPhrase = vm.envString("MNEMONIC");
        // (address admin,) = deriveRememberKey(seedPhrase, 0);
        vm.startBroadcast();

        //Our implementation(logic).Proxy will point
        // here to delegate
        FeeContract feeContract = new FeeContract();

        // Encode the initializer function call
        bytes memory data = abi.encodeWithSelector(
            FeeContract.initialize.selector,
            msg.sender // Initial owner/admin of the contract
        );

        // our proxy
        ERC1967Proxy proxy = new ERC1967Proxy(address(feeContract), data);
        vm.stopBroadcast();

        //get the owner
        proxyAddress = payable(address(proxy));
        admin = FeeContract(proxyAddress).owner();

        return (proxyAddress, admin);
    }
}
