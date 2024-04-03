// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "forge-std/Script.sol";

import { FeeContract } from "../src/FeeContract.sol";
import { ERC1967Proxy } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract DeployFeeContractScript is Script {
    /// @notice deploys the impl, proxy & initializes the impl
    /// @return proxyAddress The address of the proxy
    /// @return admin The address of the admin
    function run() external returns (address payable proxyAddress, address admin) {
        string memory seedPhrase = vm.envString("MNEMONIC");
        (admin,) = deriveRememberKey(seedPhrase, 0);
        vm.startBroadcast(admin);

        //Our implementation(logic).Proxy will point
        // here to delegate
        FeeContract feeContract = new FeeContract();

        // Encode the initializer function call
        bytes memory data = abi.encodeWithSelector(FeeContract.initialize.selector, admin);

        // our proxy
        ERC1967Proxy proxy = new ERC1967Proxy(address(feeContract), data);
        vm.stopBroadcast();

        proxyAddress = payable(address(proxy));

        return (proxyAddress, admin);
    }
}
