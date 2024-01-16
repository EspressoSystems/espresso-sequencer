// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import { Test } /*, console2*/ from "forge-std/Test.sol";
import { BoxV1 } from "../src/upgradeDemo/BoxV1.sol";
import { BoxV2 } from "../src/upgradeDemo/BoxV2.sol";
import { DeployBox } from "../script/DeployBox.s.sol";
import { UpgradeBox } from "../script/UpgradeBox.s.sol";

contract BoxTest is Test {
    BoxV1 public boxV1Proxy;
    BoxV2 public boxV2Proxy;
    DeployBox public deployer = new DeployBox();
    UpgradeBox public upgrader = new UpgradeBox();
    address public proxy;

    // deploy the first implementation with its proxy
    function setUp() public {
        proxy = deployer.run();
        boxV1Proxy = BoxV1(proxy);
    }

    // test the addbox method call via the proxy
    function testAddBox() public {
        uint256 boxSize = 1;
        boxV1Proxy.addBox(boxSize);
        assertEq(boxV1Proxy.getBox().size, boxSize);
    }

    // that the data remains the same after upgrading the implementation
    function testUpgradeSameData() public {
        //add Box of size 1
        uint256 boxSize = 1;
        boxV1Proxy.addBox(boxSize);
        assertEq(boxV1Proxy.getBox().size, boxSize);

        //upgrade box and check that the box size is maintained and the capacity is empty
        boxV2Proxy = BoxV2(upgrader.run(address(proxy)));
        assertEq(boxV2Proxy.getBox().size, boxSize);
        assertTrue(boxV2Proxy.getBox().status == BoxV2.BoxStatus.EMPTY); //is it possible to get
            // type of the actual item e.g. BoxV2.Feature.maxItems
        assertEq(boxV2Proxy.getBox().maxItems, type(uint256).min); //is it possible to get type of
            // the actual item e.g. BoxV2.Feature.maxItems
    }

    // check that the proxy address remains the same
    function testUpgradesSameProxyAddress() public {
        uint256 boxSize = 1;
        boxV1Proxy.addBox(boxSize);

        boxV2Proxy = BoxV2(upgrader.run(address(proxy)));
        assertEq(address(boxV2Proxy), address(boxV1Proxy));
        uint256 newSize = 2;
        uint256 newCapacity = 20;
        boxV2Proxy.updateBox(newSize, newCapacity);
        assertEq(boxV2Proxy.getBox().size, newSize);
    }

    // test that the ETH balance is correct after the upgrade
    function testETHDepositCorrectWhenUpgraded() public {
        uint256 boxSize = 1;
        uint256 amount = 1 ether;
        boxV1Proxy.addBox(boxSize);
        boxV1Proxy.deposit{ value: amount }(address(this));
        assertEq(boxV1Proxy.getBox().balance, amount);

        boxV2Proxy = BoxV2(upgrader.run(address(proxy)));
        assertEq(boxV2Proxy.getBox().balance, amount);
    }

    // test that users can withdraw even if the withdraw function
    // did not exist in the initial implementation
    function testIntroducingWithdrawalAfterUpgradeWorks() public {
        vm.prank(msg.sender);

        uint256 boxSize = 1;
        uint256 amount = 1 ether;
        boxV1Proxy.addBox(boxSize);

        //deposit
        vm.prank(msg.sender);
        boxV1Proxy.deposit{ value: amount }(msg.sender);

        vm.prank(msg.sender);
        assertEq(boxV1Proxy.getBox().balance, amount);

        //upgrade
        boxV2Proxy = BoxV2(upgrader.run(address(proxy)));

        //withdraw ETH
        vm.prank(msg.sender);

        boxV2Proxy.withdrawETH();

        //assert that the balance for that user is empty as the user withdrew their funds
        vm.prank(msg.sender);

        assertEq(boxV2Proxy.getBox().balance, 0);
    }

    // test that overloading a method works for new implementations
    function testNewDepositLogicWorksWithUpdatedClientPostUpgrade() public {
        vm.prank(msg.sender);

        uint256 boxSize = 1;
        uint256 amount = 1 ether;
        boxV1Proxy.addBox(boxSize);

        //upgrade
        boxV2Proxy = BoxV2(upgrader.run(address(proxy)));

        //deposit
        vm.prank(msg.sender);
        boxV2Proxy.deposit{ value: amount }(address(0), msg.sender);

        vm.prank(msg.sender);
        assertEq(boxV2Proxy.getBox().balance, amount);
    }

    // test backward compatibility with an overloaded function
    function testNewDepositLogicWorksWithOldClientPostUpgrade() public {
        vm.prank(msg.sender);

        uint256 boxSize = 1;
        uint256 amount = 1 ether;
        boxV1Proxy.addBox(boxSize);

        //upgrade
        boxV2Proxy = BoxV2(upgrader.run(address(proxy)));

        //deposit
        vm.prank(msg.sender);
        boxV2Proxy.deposit{ value: amount }(msg.sender);

        vm.prank(msg.sender);
        assertEq(boxV2Proxy.getBox().balance, amount);
    }

    // test upgrading a struct works post ugprade
    function testUpgradeNewStructElement() public {
        //add Box of size 1
        uint256 boxSize = 1;
        boxV1Proxy.addBox(boxSize);
        assertEq(boxV1Proxy.getBox().size, boxSize);

        //upgrade Box Implementation and check that the box size is maintained and the capacity is
        boxV2Proxy = BoxV2(upgrader.run(address(proxy)));
        assertEq(boxV2Proxy.getBox().size, boxSize);
        assertTrue(boxV2Proxy.getBox().status == BoxV2.BoxStatus.EMPTY); //is it possible to get
            // type of the actual item e.g. BoxV2.Feature.maxItems
        assertEq(boxV2Proxy.getBox().maxItems, type(uint256).min); //is it possible to get type of
            // the actual item e.g. BoxV2.Feature.maxItems

        //use newly updated struct element, maxItems
        uint256 newCapacity = 20;
        boxV2Proxy.updateBoxCapacity(newCapacity);
        assertEq(boxV2Proxy.getBox().maxItems, newCapacity);
    }

    // test upgrading a new enum works post ugprade
    function testUpgradeNewEnumType() public {
        //add Box of size 1
        uint256 boxSize = 1;
        BoxV1.BoxStatus boxStatus = BoxV1.BoxStatus.FULL;
        boxV1Proxy.addBox(boxSize);
        boxV1Proxy.updateBoxStatus(boxStatus);
        assertEq(boxV1Proxy.getBox().size, boxSize);
        assertTrue(boxV1Proxy.getBox().status == boxStatus);

        boxV2Proxy = BoxV2(upgrader.run(address(proxy)));
        uint256 currentMaxItems = type(uint256).min;
        assertEq(boxV2Proxy.getBox().size, boxSize);
        assertTrue(boxV2Proxy.getBox().status == BoxV2.BoxStatus.FULL);
        assertEq(boxV2Proxy.getBox().maxItems, currentMaxItems);
    }
}
