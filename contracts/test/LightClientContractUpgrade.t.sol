// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;
pragma experimental ABIEncoderV2;

import { Test } /*, console2*/ from "forge-std/Test.sol";
import { BN254 } from "bn254/BN254.sol";
import { LightClient as LCV1 } from "../src/LightClient.sol";
import { LightClientV2 as LCV2 } from "../src/LightClientV2.sol";
import { DeployLightClientContractScript } from "../script/DeployLightContract.s.sol";
import { UpgradeLightClientScript } from "../script/UpgradeLightClient.s.sol";

contract LightClientUpgradeTest is Test {
    LCV1 public lcV1Proxy;
    LCV2 public lcV2Proxy;

    DeployLightClientContractScript public deployer = new DeployLightClientContractScript();
    UpgradeLightClientScript public upgrader = new UpgradeLightClientScript();

    LCV1.LightClientState public stateV1;

    address public admin;
    address public proxy;

    // deploy the first implementation with its proxy
    function setUp() public {
        (proxy, admin, stateV1) = deployer.run();
        lcV1Proxy = LCV1(proxy);
    }

    function testCorrectInitialization() public {
        assert(lcV1Proxy.blocksPerEpoch() == 10);
        assert(lcV1Proxy.currentEpoch() == 0);

        {
            (
                uint64 viewNum,
                uint64 blockHeight,
                BN254.ScalarField blockCommRoot,
                BN254.ScalarField feeLedgerComm,
                BN254.ScalarField stakeTableBlsKeyComm,
                BN254.ScalarField stakeTableSchnorrKeyComm,
                BN254.ScalarField stakeTableAmountComm,
                uint256 threshold
            ) = lcV1Proxy.genesisState();
            LCV1.LightClientState memory expectedState = LCV1.LightClientState(
                viewNum,
                blockHeight,
                blockCommRoot,
                feeLedgerComm,
                stakeTableBlsKeyComm,
                stakeTableSchnorrKeyComm,
                stakeTableAmountComm,
                threshold
            );

            assertEq(abi.encode(expectedState), abi.encode(stateV1));
        }
        {
            (
                uint64 viewNum,
                uint64 blockHeight,
                BN254.ScalarField blockCommRoot,
                BN254.ScalarField feeLedgerComm,
                BN254.ScalarField stakeTableBlsKeyComm,
                BN254.ScalarField stakeTableSchnorrKeyComm,
                BN254.ScalarField stakeTableAmountComm,
                uint256 threshold
            ) = lcV1Proxy.finalizedState();
            LCV1.LightClientState memory expectedState = LCV1.LightClientState(
                viewNum,
                blockHeight,
                blockCommRoot,
                feeLedgerComm,
                stakeTableBlsKeyComm,
                stakeTableSchnorrKeyComm,
                stakeTableAmountComm,
                threshold
            );

            assertEq(abi.encode(expectedState), abi.encode(stateV1));
        }

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
        lcV2Proxy = LCV2(upgrader.run(admin, proxy));

        assertEq(lcV2Proxy.newField(), 0);
        assertEq(lcV2Proxy.blocksPerEpoch(), 10);
        assertEq(lcV2Proxy.currentEpoch(), 0);

        (
            uint64 viewNum,
            uint64 blockHeight,
            BN254.ScalarField blockCommRoot,
            BN254.ScalarField feeLedgerComm,
            BN254.ScalarField stakeTableBlsKeyComm,
            BN254.ScalarField stakeTableSchnorrKeyComm,
            BN254.ScalarField stakeTableAmountComm,
            uint256 threshold
        ) = lcV2Proxy.finalizedState();

        LCV1.LightClientState memory expectedState = LCV1.LightClientState(
            viewNum,
            blockHeight,
            blockCommRoot,
            feeLedgerComm,
            stakeTableBlsKeyComm,
            stakeTableSchnorrKeyComm,
            stakeTableAmountComm,
            threshold
        );
        assertEq(abi.encode(expectedState), abi.encode(stateV1));
    }

    // check that the proxy address remains the same
    function testUpgradesSameProxyAddress() public {
        uint256 currentVersion = 1;
        assertEq(lcV1Proxy.VERSION(), currentVersion);

        //upgrade box
        uint256 newVersion = 2;
        lcV2Proxy = LCV2(upgrader.run(admin, proxy));
        assertEq(address(lcV2Proxy), address(lcV1Proxy));
        assertEq(lcV2Proxy.VERSION(), newVersion);
    }

    function testMaliciousUpgradeFails() public {
        address attacker = makeAddr("attacker");

        //attempted upgrade as attacker will revert
        vm.expectRevert();
        lcV2Proxy = LCV2(upgrader.run(attacker, address(proxy)));
    }
}
