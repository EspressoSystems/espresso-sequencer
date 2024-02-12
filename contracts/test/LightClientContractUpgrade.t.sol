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

    LCV1.LightClientState stateV1;

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
        //upgrade LightClient and check that the genesis state is not change and that its
        // corresponding newField is set to 0
        lcV2Proxy = LCV2(upgrader.run(admin, proxy));

        (
            uint64 viewNum,
            uint64 blockHeight,
            BN254.ScalarField blockCommRoot,
            BN254.ScalarField feeLedgerComm,
            BN254.ScalarField stakeTableBlsKeyComm,
            BN254.ScalarField stakeTableSchnorrKeyComm,
            BN254.ScalarField stakeTableAmountComm,
            uint256 threshold,
            uint256 newField
        ) = lcV2Proxy.finalizedState();
        assert(lcV2Proxy.blocksPerEpoch() == 10);
        assert(lcV2Proxy.currentEpoch() == 0);
    }
}
