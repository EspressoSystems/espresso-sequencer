// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "forge-std/Script.sol";

import { FeeContract } from "../src/FeeContract.sol";
import { ERC1967Proxy } from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract DeployFeeContract is Script {
    function run() external returns (address payable) {
        string memory seedPhrase = vm.envString("MNEMONIC");
        (address admin,) = deriveRememberKey(seedPhrase, 0);
        address payable proxy = deployFeeContract(admin);

        return payable(proxy);
    }

    /// @notice deploys the implementation contract and the proxy with the address of implementation
    /// @return address of the proxy
    function deployFeeContract(address admin) public returns (address payable) {
        vm.startBroadcast(admin);

        FeeContract feeContract = new FeeContract(); //Our implementation(logic).Proxy will point
            // here to delegate
            // call/borrow the functions

        // Encode the initializer function call
        bytes memory data = abi.encodeWithSelector(
            FeeContract(feeContract).initialize.selector,
            msg.sender // Initial owner/admin of the contract
        );

        ERC1967Proxy proxy = new ERC1967Proxy(address(feeContract), data);
        vm.stopBroadcast();
        return payable(address(proxy));
    }
}
