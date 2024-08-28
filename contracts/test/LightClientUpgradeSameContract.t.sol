// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;
pragma experimental ABIEncoderV2;

import { Test } /*, console2*/ from "forge-std/Test.sol";
import { LightClient as LCV1 } from "../src/LightClient.sol";
import { LightClient as LCV2 } from "../src/LightClient.sol";
import { DeployLightClientContractScript } from "../script/LightClient.s.sol";
import { UpgradeLightClientScript } from "../script/UpgradeSameLightClient.s.sol";

contract LightClientUpgradeSameContractTest is Test {
    LCV1 public lcV1Proxy;
    LCV2 public lcV2Proxy;

    DeployLightClientContractScript public deployer = new DeployLightClientContractScript();
    UpgradeLightClientScript public upgrader = new UpgradeLightClientScript();

    LCV1.LightClientState public stateV1;

    address public admin;
    address public proxy;

    uint32 public constant MAX_HISTORY_SECONDS = 864000; //10 days

    // deploy the first implementation with its proxy
    function setUp() public {
        (proxy, admin, stateV1) = deployer.run(10, 5, MAX_HISTORY_SECONDS);
        lcV1Proxy = LCV1(proxy);
    }

    function testCorrectInitialization() public view {
        assertEq(abi.encode(lcV1Proxy.getGenesisState()), abi.encode(stateV1));

        assertEq(abi.encode(lcV1Proxy.getFinalizedState()), abi.encode(stateV1));

        bytes32 stakeTableComm = lcV1Proxy.computeStakeTableComm(stateV1);
        assertEq(lcV1Proxy.votingStakeTableCommitment(), stakeTableComm);
        assertEq(lcV1Proxy.frozenStakeTableCommitment(), stakeTableComm);
        assertEq(lcV1Proxy.votingThreshold(), stateV1.threshold);
        assertEq(lcV1Proxy.frozenThreshold(), stateV1.threshold);
    }

    // that the data remains the same after upgrading the implementation
    function testUpgradeSameData() public {
        // Upgrade LightClient and check that the genesis state is not changed and that the new
        // field
        // of the upgraded contract is set to 0
        lcV2Proxy = LCV2(upgrader.run(0, proxy));

        LCV2.LightClientState memory expectedLightClientState = LCV2.LightClientState(
            stateV1.viewNum,
            stateV1.blockHeight,
            stateV1.blockCommRoot,
            stateV1.feeLedgerComm,
            stateV1.stakeTableBlsKeyComm,
            stateV1.stakeTableSchnorrKeyComm,
            stateV1.stakeTableAmountComm,
            stateV1.threshold
        );

        assertEq(abi.encode(lcV2Proxy.getFinalizedState()), abi.encode(expectedLightClientState));
    }

    // check that the proxy address remains the same
    function testUpgradesSameProxyAddress() public {
        (uint8 major, uint8 minor, uint8 patch) = lcV1Proxy.getVersion();
        assertEq(major, 1);
        assertEq(minor, 0);
        assertEq(patch, 0);

        //upgrade box
        lcV2Proxy = LCV2(upgrader.run(0, proxy));
        assertEq(address(lcV2Proxy), address(lcV1Proxy));
    }

    function testMaliciousUpgradeFails() public {
        address attacker = makeAddr("attacker");

        //attempted upgrade as attacker will revert
        vm.prank(attacker);
        vm.expectRevert();
        lcV2Proxy = LCV2(upgrader.run(0, address(proxy)));
    }
}
