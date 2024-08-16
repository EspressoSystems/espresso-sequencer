// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import { Test } from "forge-std/Test.sol";
import { DemoBoxV1 } from "../demo/upgradeDemo/DemoBoxV1.sol";
import { DemoBoxV2 } from "../demo/upgradeDemo/DemoBoxV2.sol";
import { DeployBoxScript } from "./script/Box.s.sol";
import { UpgradeBoxScript } from "./script/UpgradeBox.s.sol";

contract DemoBoxTest is Test {
    DemoBoxV1 public boxV1Proxy;
    DemoBoxV2 public boxV2Proxy;
    DeployBoxScript public deployer = new DeployBoxScript();
    UpgradeBoxScript public upgrader = new UpgradeBoxScript();
    address public proxy;
    address private admin = makeAddr("admin");

    // deploy the first implementation with its proxy
    function setUp() public {
        proxy = deployer.run();
        boxV1Proxy = DemoBoxV1(proxy);
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
        uint256 boxSize = 38;
        boxV1Proxy.addBox(boxSize);
        assertEq(boxV1Proxy.getBox().size, boxSize);

        //upgrade box and check that the box size is maintained and the capacity is empty
        boxV2Proxy = DemoBoxV2(upgrader.run(admin, address(proxy)));
        DemoBoxV2.Box memory boxV2 = boxV2Proxy.getBox();
        assertEq(boxV2.size, boxSize);
        assertTrue(boxV2.status == DemoBoxV2.BoxStatus.EMPTY);
        assertEq(boxV2.maxItems, type(uint256).min);
    }

    // check that the proxy address remains the same
    function testUpgradesSameProxyAddress() public {
        uint256 boxSize = 1;
        uint256 currentVersion = 1;
        boxV1Proxy.addBox(boxSize);
        assertEq(boxV1Proxy.version(), currentVersion);

        //upgrade box
        uint256 newVersion = 2;
        boxV2Proxy = DemoBoxV2(upgrader.run(admin, address(proxy)));
        assertEq(address(boxV2Proxy), address(boxV1Proxy));
        uint256 newSize = 2;
        uint256 newCapacity = 20;
        boxV2Proxy.updateBox(newSize, newCapacity);

        DemoBoxV2.Box memory boxV2 = boxV2Proxy.getBox();
        assertEq(boxV2.size, newSize);
        assertEq(boxV2.maxItems, newCapacity);
        assertEq(boxV2Proxy.version(), newVersion);
    }

    // test that the ETH balance is correct after the upgrade
    function testETHDepositCorrectWhenUpgraded() public {
        uint256 boxSize = 1;
        uint256 amount = 1 ether;
        boxV1Proxy.addBox(boxSize);
        boxV1Proxy.deposit{ value: amount }(address(this));
        assertEq(boxV1Proxy.getBox().balance, amount);

        //upgrade box
        boxV2Proxy = DemoBoxV2(upgrader.run(admin, address(proxy)));
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

        //upgrade box
        boxV2Proxy = DemoBoxV2(upgrader.run(admin, address(proxy)));
        //withdraw ETH
        vm.prank(msg.sender);
        uint256 userBalanceBefore = msg.sender.balance;

        boxV2Proxy.withdrawETH();

        //assert that the balance for that user is empty as the user withdrew their funds
        vm.prank(msg.sender);

        assertEq(boxV2Proxy.getBox().balance, 0);
        assertEq(msg.sender.balance, userBalanceBefore + amount);
    }

    // test that overloading a method works for new implementations
    function testNewDepositLogicWorksWithUpdatedClientPostUpgrade() public {
        vm.prank(msg.sender);

        uint256 boxSize = 1;
        uint256 amount = 1 ether;
        boxV1Proxy.addBox(boxSize);

        //upgrade
        boxV2Proxy = DemoBoxV2(upgrader.run(admin, address(proxy)));

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
        boxV2Proxy = DemoBoxV2(upgrader.run(admin, address(proxy)));

        //deposit
        vm.prank(msg.sender);
        boxV2Proxy.deposit{ value: amount }(msg.sender);

        vm.prank(msg.sender);
        assertEq(boxV2Proxy.getBox().balance, amount);
    }

    // test upgrading a struct works post upgrade
    function testUpgradeNewStructElement() public {
        //add Box of size 1
        uint256 boxSize = 1;
        boxV1Proxy.addBox(boxSize);
        assertEq(boxV1Proxy.getBox().size, boxSize);

        //upgrade Box
        boxV2Proxy = DemoBoxV2(upgrader.run(admin, address(proxy)));
        DemoBoxV2.Box memory boxV2 = boxV2Proxy.getBox();

        assertEq(boxV2.size, boxSize);
        assertTrue(boxV2.status == DemoBoxV2.BoxStatus.EMPTY);
        assertEq(boxV2.maxItems, type(uint256).min);

        //use newly updated struct element, maxItems
        uint256 newCapacity = 20;
        boxV2Proxy.updateBoxCapacity(newCapacity);
        assertEq(boxV2Proxy.getBox().maxItems, newCapacity);
    }

    // test upgrading a new enum works post upgrade
    function testUpgradeNewEnumType() public {
        //add Box of size 1
        uint256 boxSize = 1;
        DemoBoxV1.BoxStatus boxStatus = DemoBoxV1.BoxStatus.FULL;
        boxV1Proxy.addBox(boxSize);
        boxV1Proxy.updateBoxStatus(boxStatus);
        DemoBoxV1.Box memory boxV1 = boxV1Proxy.getBox();
        assertEq(boxV1.size, boxSize);
        assertTrue(boxV1.status == boxStatus);

        //upgrade
        boxV2Proxy = DemoBoxV2(upgrader.run(admin, address(proxy)));
        uint256 currentMaxItems = type(uint256).min;

        DemoBoxV2.Box memory boxV2 = boxV2Proxy.getBox();
        assertEq(boxV2.size, boxSize);
        assertTrue(boxV2.status == DemoBoxV2.BoxStatus.FULL);
        assertEq(boxV2.maxItems, currentMaxItems);
    }

    //test new enum type works
    function testNewEnumType() public {
        //add Box of size 1
        uint256 boxSize = 1;
        DemoBoxV1.BoxStatus boxStatus = DemoBoxV1.BoxStatus.FULL;
        boxV1Proxy.addBox(boxSize);
        boxV1Proxy.updateBoxStatus(boxStatus);

        //upgrade
        boxV2Proxy = DemoBoxV2(upgrader.run(admin, address(proxy)));
        DemoBoxV2.BoxStatus newBoxStatus = DemoBoxV2.BoxStatus.ALMOST_FULL;

        boxV2Proxy.updateBoxStatus(newBoxStatus);

        assertTrue(boxV2Proxy.getBox().status == newBoxStatus);
    }

    //test that the function still works as expected when the logic changes
    function testSameFunctionSignatureDifferentLogic() public {
        //add Box of size 1
        uint256 boxSize = 1;
        boxV1Proxy.addBox(boxSize);
        uint256 boxV1Size = boxV1Proxy.getBox().size;

        //upgrade box
        boxV2Proxy = DemoBoxV2(upgrader.run(admin, address(proxy)));
        uint256 boxV2Size = boxV2Proxy.getBox().size;

        assertEq(boxV1Size, boxV2Size);
    }

    function testMaliciousUpgradeFails() public {
        address attacker = makeAddr("attacker");

        //attempted upgrade as attacker will revert
        vm.expectRevert();
        boxV2Proxy = DemoBoxV2(upgrader.run(attacker, address(proxy)));
    }
}
