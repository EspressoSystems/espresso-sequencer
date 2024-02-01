// SPDX-License-Identifier: Unlicensed

/* solhint-disable contract-name-camelcase, func-name-mixedcase, one-contract-per-file */

pragma solidity ^0.8.0;

// Libraries
import { Test } /*, console2 }*/ from "forge-std/Test.sol";
import { OwnableUpgradeable } from
    "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import { Initializable } from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import { UUPSUpgradeable } from
    "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

// Target contract
import { FeeContract } from "../src/FeeContract.sol";
import { DeployFeeContract } from "../script/DeployFeeContract.s.sol";

/// @title FeeContract Test
contract FeeContractTest is Test {
    FeeContract public feeContract;

    function setUp() public {
        feeContract = new FeeContract();
    }

    //test deposits work
    function testFuzz_deposit(address user, uint256 amount) public payable {
        vm.assume(user != address(0));
        amount = bound(amount, feeContract.MIN_DEPOSIT_AMOUNT(), feeContract.MAX_DEPOSIT_AMOUNT());

        uint256 balanceBeforeUser = feeContract.getBalance(user);

        //check that the depositEvent is emitted
        vm.expectEmit(true, false, false, true);
        // We emit the event we expect to see.
        emit FeeContract.Deposit(user, amount);

        //deposit for the two users
        feeContract.deposit{ value: amount }(user);

        //get the balance for that user after the deposit
        uint256 balanceAfterUser = feeContract.getBalance(user);

        //test that the users' balances have been incremented accurately
        assertEq(balanceAfterUser, balanceBeforeUser + amount);

        //test that the smart contract has the accumulative balance for both users
        assertEq(address(feeContract).balance, amount);
    }

    //test that depositing twice increases the user's baalance
    function testFuzz_depositTwice(address user1, address user2, uint256 amount1, uint256 amount2)
        public
        payable
    {
        vm.assume(user1 != address(0) && user2 != address(0));
        amount1 = bound(amount1, feeContract.MIN_DEPOSIT_AMOUNT(), feeContract.MAX_DEPOSIT_AMOUNT());
        amount2 = bound(amount2, feeContract.MIN_DEPOSIT_AMOUNT(), feeContract.MAX_DEPOSIT_AMOUNT());

        vm.prank(user1);
        uint256 balanceBefore1 = feeContract.getBalance(user1);

        //deposit for the user
        feeContract.deposit{ value: amount1 }(user1);

        //get the balance for that user after the deposit
        uint256 balanceAfter1 = feeContract.getBalance(user1);
        assertEq(balanceAfter1, balanceBefore1 + amount1);

        vm.prank(user2);
        uint256 balanceBefore2 = feeContract.getBalance(user2);

        //deposit for the user
        feeContract.deposit{ value: amount2 }(user2);

        //get the balance for that user after the deposit
        uint256 balanceAfter2 = feeContract.getBalance(user2);
        assertEq(balanceAfter2, balanceBefore2 + amount2);
    }

    function testFuzz_noFunction(uint256 amount) public payable {
        address fcAddress = address(feeContract);
        (bool success,) = fcAddress.call{ value: amount }("");

        //assert that the transaction was not successful
        assertFalse(success);

        //assert that the balance of the fee contract is still zero
        assertEq(address(feeContract).balance, 0);
    }

    function testFuzz_nonExistentFunction(uint256 amount) public payable {
        address fcAddress = address(feeContract);
        (bool success,) =
            fcAddress.call{ value: amount }(abi.encodeWithSignature("withdraw(address)", "0x"));

        //assert that the transaction was not successful
        assertFalse(success);

        //assert that the balance of the fee contract is still zero
        assertEq(address(feeContract).balance, 0);
    }

    //test deposits with a large amount reverts
    function test_depositMaxAmount() public {
        address user = makeAddr("user");
        uint256 amount = feeContract.MAX_DEPOSIT_AMOUNT() + 1;

        vm.expectRevert(FeeContract.DepositTooLarge.selector);

        //deposit for the user
        feeContract.deposit{ value: amount }(user);
    }

    //test deposits with a less than the min amount reverts
    function test_depositMinAmount() public {
        address user = makeAddr("user");
        uint256 amount = feeContract.MIN_DEPOSIT_AMOUNT() - 0.0001 ether;

        vm.expectRevert(FeeContract.DepositTooSmall.selector);

        //deposit for the user
        feeContract.deposit{ value: amount }(user);
    }

    //test deposits with invalid user address reverts
    function test_invalidUserAddress() public {
        address user = address(0);
        uint256 amount = 0.5 ether;

        vm.expectRevert(FeeContract.InvalidUserAddress.selector);

        //deposit for the user
        feeContract.deposit{ value: amount }(user);
    }

    function testFuzz_newUserHasZeroBalance(address user) public {
        vm.assume(user != address(0));

        uint256 balance = feeContract.getBalance(user);

        assertEq(balance, 0);
    }
}

contract FeeContractUpgradabilityTest is Test {
    address payable public proxy;
    address public admin;
    FeeContract public feeContractProxy;
    DeployFeeContract public deployer = new DeployFeeContract();

    function setUp() public {
        (proxy, admin) = deployer.run();
        feeContractProxy = FeeContract(proxy);
    }

    //test deposits work with a proxy
    function testFuzz_deposit(address user, uint256 amount) public payable {
        vm.assume(user != address(0));
        amount = bound(
            amount, feeContractProxy.MIN_DEPOSIT_AMOUNT(), feeContractProxy.MAX_DEPOSIT_AMOUNT()
        );

        uint256 balanceBefore = feeContractProxy.getBalance(user);

        //check that the depositEvent is emitted
        vm.expectEmit(true, false, false, true);
        // We emit the event we expect to see.
        emit FeeContract.Deposit(user, amount);

        //deposit for the user
        feeContractProxy.deposit{ value: amount }(user);

        //get the balance for that user after the deposit
        uint256 balanceAfter = feeContractProxy.getBalance(user);
        assertEq(balanceAfter, balanceBefore + amount);
    }

    //test that depositing twice increases the user's baalance
    function testFuzz_depositTwice(address user) public payable {
        if (msg.value == 0) return;
        vm.prank(user);
        uint256 balanceBefore = feeContractProxy.getBalance(user);

        uint256 depositAmount = msg.value / 2;
        //deposit for the user
        feeContractProxy.deposit{ value: depositAmount }(user);

        //get the balance for that user after the deposit
        uint256 balanceAfter = feeContractProxy.getBalance(user);
        assertEq(balanceAfter, balanceBefore + depositAmount);

        //deposit the remainder for the user
        feeContractProxy.deposit{ value: depositAmount }(user);

        //get the balance for that user after the 2nd deposit
        uint256 balanceAfter2 = feeContractProxy.getBalance(user);
        assertEq(balanceAfter2, balanceAfter + depositAmount);
    }

    function testFuzz_noFunction(uint256 amount) public payable {
        address fcAddress = address(feeContractProxy);
        (bool success,) = fcAddress.call{ value: amount }("");

        //assert that the transaction was not successful
        assertFalse(success);

        //assert that the balance of the fee contract is still zero
        assertEq(address(feeContractProxy).balance, 0);
    }

    function testFuzz_nonExistentFunction(uint256 amount) public payable {
        address fcAddress = address(feeContractProxy);
        (bool success,) =
            fcAddress.call{ value: amount }(abi.encodeWithSignature("withdraw(address)", "0x"));

        //assert that the transaction was not successful
        assertFalse(success);

        //assert that the balance of the fee contract is still zero
        assertEq(address(feeContractProxy).balance, 0);
    }

    function testUpgradeTo() public {
        FeeContractV2Test feeContractV2 = new FeeContractV2Test();

        vm.prank(admin);
        vm.expectEmit(false, false, false, true);
        // We emit the event we expect to see.
        emit FeeContract.Upgrade(address(feeContractV2));

        feeContractProxy.upgradeToAndCall(address(feeContractV2), "");
    }

    function testFailUpgradeToWithWrongAdmin() public {
        FeeContractV2Test feeContractV2 = new FeeContractV2Test();

        //start the upgrade with another address which isn't the admin
        vm.prank(makeAddr("otherUser"));
        feeContractProxy.upgradeToAndCall(address(feeContractV2), "");
    }
}

contract FeeContractV2Test is Initializable, OwnableUpgradeable, UUPSUpgradeable {
    /// @notice upgrade event when the proxy updates the implementation it's pointing to
    event Upgrade(address implementation);

    /// @notice since the constuctor initializes storage on this contract we disable it
    /// @dev storage is on the proxy contract since it calls this contract via delegatecall
    constructor() {
        _disableInitializers();
    }

    /// @notice This contract is called by the proxy when you deploy this contract
    function initialize() public initializer {
        __Ownable_init(msg.sender); //sets owner to msg.sender
        __UUPSUpgradeable_init();
    }

    /// @notice only the owner can authorize an upgrade
    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {
        emit Upgrade(newImplementation);
    }
}
