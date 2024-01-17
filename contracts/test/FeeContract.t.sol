// SPDX-License-Identifier: Unlicensed

/* solhint-disable contract-name-camelcase, func-name-mixedcase, one-contract-per-file */

pragma solidity ^0.8.0;

// Libraries
import { Test } /*, console2*/ from "forge-std/Test.sol";

// Target contract
import { FeeContract } from "../src/FeeContract.sol";

contract FeeContractTest is Test {
    FeeContract public feeContract;

    function setUp() public {
        feeContract = new FeeContract();
    }

    //test deposits work
    function testFuzz_deposit() public payable {
        if (msg.value == 0) return;
        address user = makeAddr("user");
        vm.prank(user);
        uint256 balanceBefore = feeContract.getBalance(user);

        //deposit for the user
        feeContract.deposit(user);

        //get the balance for that user after the deposit
        uint256 balanceAfter = feeContract.getBalance(user);
        assertEq(balanceAfter, balanceBefore + msg.value);
    }

    function testFuzz_depositTwice() public payable {
        if (msg.value == 0) return;
        address user = makeAddr("user");
        vm.prank(user);
        uint256 balanceBefore = feeContract.getBalance(user);

        uint256 depositAmount = msg.value / 2;
        //deposit for the user
        feeContract.deposit{ value: depositAmount }(user);

        //get the balance for that user after the deposit
        uint256 balanceAfter = feeContract.getBalance(user);
        assertEq(balanceAfter, balanceBefore + depositAmount);

        //deposit the remainder for the user
        feeContract.deposit{ value: depositAmount }(user);

        //get the balance for that user after the 2nd deposit
        uint256 balanceAfter2 = feeContract.getBalance(user);
        assertEq(balanceAfter2, balanceAfter + depositAmount);
    }

    function testFuzz_noFunction() public payable {
        address fcAddress = address(feeContract);
        (bool success,) = fcAddress.call{ value: msg.value }("");
        assertFalse(success);
    }

    function testFuzz_nonExistentFunction() public payable {
        address fcAddress = address(feeContract);
        (bool success,) =
            fcAddress.call{ value: msg.value }(abi.encodeWithSignature("withdraw(address)", "0x"));
        assertFalse(success);
    }
}
