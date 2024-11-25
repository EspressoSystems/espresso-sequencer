// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/SimpleStakeTable.sol";
import { BN254 } from "bn254/BN254.sol";
import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";

contract SimpleStakeTableTest is Test {
    SimpleStakeTable stakeTable;
    address owner = address(1); // mock owner address

    function setUp() public {
        vm.prank(owner); // impersonate the owner
        stakeTable = new SimpleStakeTable(owner);
    }

    function points(uint64 num) private returns (BN254.G2Point[] memory) {
        string[] memory cmds = new string[](3);
        cmds[0] = "diff-test";
        cmds[1] = "gen-random-g2-point";

        BN254.G2Point[] memory ps = new BN254.G2Point[](num);

        for (uint64 i = 0; i < num; i++) {
            cmds[2] = vm.toString(i + 1);
            bytes memory result = vm.ffi(cmds);
            BN254.G2Point memory p = abi.decode(result, (BN254.G2Point));
            ps[i] = p;
        }
        return ps;
    }

    function testInsertAndIsStaker() public {
        vm.prank(owner);
        BN254.G2Point[] memory stakers = points(1);
        stakeTable.insert(stakers);
        assertTrue(stakeTable.isStaker(stakers[0]));
    }

    function testInsertAndIsStakerMany() public {
        vm.prank(owner);
        BN254.G2Point[] memory stakers = points(10);
        stakeTable.insert(stakers);
        assertTrue(stakeTable.isStaker(stakers[0]));
    }

    function testInsertRevertsIfStakerExists() public {
        vm.prank(owner);
        BN254.G2Point[] memory stakers = points(1);
        stakeTable.insert(stakers);

        // Try adding the same staker again
        vm.expectRevert(
            abi.encodeWithSelector(SimpleStakeTable.StakerAlreadyExists.selector, stakers[0])
        );
        vm.prank(owner);
        stakeTable.insert(stakers);
    }

    function testRemoveAndIsNotStaker() public {
        BN254.G2Point[] memory stakers = points(1);
        vm.prank(owner);
        stakeTable.insert(stakers);

        vm.prank(owner);
        stakeTable.remove(stakers);

        assertFalse(stakeTable.isStaker(stakers[0]));
    }

    function testRemoveRevertsIfStakerNotFound() public {
        vm.prank(owner);
        BN254.G2Point[] memory stakers = points(1);
        vm.expectRevert(abi.encodeWithSelector(SimpleStakeTable.StakerNotFound.selector, stakers[0]));
        // Attempt to remove a non-existent staker
        stakeTable.remove(stakers);
    }

    function testNonOwnerCannotInsert() public {
        vm.prank(address(2));
        vm.expectRevert(
            abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, address(2))
        );
        BN254.G2Point[] memory stakers = points(1);
        stakeTable.insert(stakers);
    }

    function testNonOwnerCannotRemove() public {
        vm.prank(address(2));
        vm.expectRevert(
            abi.encodeWithSelector(Ownable.OwnableUnauthorizedAccount.selector, address(2))
        );
        BN254.G2Point[] memory stakers = points(1);
        stakeTable.remove(stakers);
    }
}
