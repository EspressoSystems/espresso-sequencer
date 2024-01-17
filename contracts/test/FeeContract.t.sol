// SPDX-License-Identifier: Unlicensed

/* solhint-disable contract-name-camelcase, func-name-mixedcase, one-contract-per-file */

pragma solidity ^0.8.0;

// Libraries
import { Test } /*, console2*/ from "forge-std/Test.sol";

// Target contract
import { FeeContract } from "../src/FeeContract.sol";

/// @title FeeContract Test
contract FeeContractTest is Test {
    FeeContract public feeContract;

    function setUp() public {
        feeContract = new FeeContract();
    }

    //test deposits work
    function testFuzz_deposit(address user) public payable {
        if (msg.value == 0) return;
        uint256 balanceBefore = feeContract.getBalance(user);

        //deposit for the user
        feeContract.deposit(user);

        //get the balance for that user after the deposit
        uint256 balanceAfter = feeContract.getBalance(user);
        assertEq(balanceAfter, balanceBefore + msg.value);
    }

    //test deposits work
    function testFuzz_deposit(address user1, address user2) public payable {
        if (msg.value == 0) return;
        uint256 balanceBeforeUser1 = feeContract.getBalance(user1);
        uint256 balanceBeforeUser2 = feeContract.getBalance(user2);

        uint256 depositAmount = msg.value / 2;

        //deposit for the two users
        feeContract.deposit{ value: depositAmount }(user1);
        feeContract.deposit{ value: depositAmount }(user2);

        //get the balance for that user after the deposit
        uint256 balanceAfterUser1 = feeContract.getBalance(user1);
        uint256 balanceAfterUser2 = feeContract.getBalance(user2);

        //test that the users' balances have been incremented accurately
        assertEq(balanceAfterUser1, balanceBeforeUser1 + depositAmount);
        assertEq(balanceAfterUser2, balanceBeforeUser2 + depositAmount);

        //test that the smart contract has the accumulative balance for both users
        assertEq(address(feeContract).balance, msg.value);
    }

    //test that depositing twice increases the user's baalance
    function testFuzz_depositTwice(address user) public payable {
        if (msg.value == 0) return;
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

        //assert that the transaction was not successful
        assertFalse(success);

        //assert that the balance of the fee contract is still zero
        assertEq(address(feeContract).balance, 0);
    }

    function testFuzz_nonExistentFunction() public payable {
        address fcAddress = address(feeContract);
        (bool success,) =
            fcAddress.call{ value: msg.value }(abi.encodeWithSignature("withdraw(address)", "0x"));

        //assert that the transaction was not successful
        assertFalse(success);

        //assert that the balance of the fee contract is still zero
        assertEq(address(feeContract).balance, 0);
    }
}
