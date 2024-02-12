// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;
pragma experimental ABIEncoderV2;

import { Test } /*, console2*/ from "forge-std/Test.sol";
import { BN254 } from "bn254/BN254.sol";
import { LightClient as LC } from "../src/LightClient.sol";
import { DeployLightClientContractScript } from "../script/DeployLightContract.s.sol";

contract LightClientUpgradeTest is Test {
    LC public lcV1Proxy;

    DeployLightClientContractScript public deployer = new DeployLightClientContractScript();
    LC.LightClientState state;

    address public admin;
    address public proxy;

    // deploy the first implementation with its proxy
    function setUp() public {
        (proxy, admin, state) = deployer.run();
        lcV1Proxy = LC(proxy);
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
            LC.LightClientState memory expectedState = LC.LightClientState(
                viewNum,
                blockHeight,
                blockCommRoot,
                feeLedgerComm,
                stakeTableBlsKeyComm,
                stakeTableSchnorrKeyComm,
                stakeTableAmountComm,
                threshold
            );

            assertEq(abi.encode(expectedState), abi.encode(state));
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
            LC.LightClientState memory expectedState = LC.LightClientState(
                viewNum,
                blockHeight,
                blockCommRoot,
                feeLedgerComm,
                stakeTableBlsKeyComm,
                stakeTableSchnorrKeyComm,
                stakeTableAmountComm,
                threshold
            );

            assertEq(abi.encode(expectedState), abi.encode(state));
        }

        bytes32 stakeTableComm = lcV1Proxy.computeStakeTableComm(state);
        assertEq(lcV1Proxy.votingStakeTableCommitment(), stakeTableComm);
        assertEq(lcV1Proxy.frozenStakeTableCommitment(), stakeTableComm);
        assertEq(lcV1Proxy.votingThreshold(), state.threshold);
        assertEq(lcV1Proxy.frozenThreshold(), state.threshold);
    }

    /// @dev helper getter since solidity doesn't return struct but tuples only
    function getGenesisState(LC lc) public view returns (LC.LightClientState memory) {
        (
            uint64 viewNum,
            uint64 blockHeight,
            BN254.ScalarField blockCommRoot,
            BN254.ScalarField feeLedgerComm,
            BN254.ScalarField stakeTableBlsKeyComm,
            BN254.ScalarField stakeTableSchnorrKeyComm,
            BN254.ScalarField stakeTableAmountComm,
            uint256 threshold
        ) = lc.genesisState();

        return LC.LightClientState(
            viewNum,
            blockHeight,
            blockCommRoot,
            feeLedgerComm,
            stakeTableBlsKeyComm,
            stakeTableSchnorrKeyComm,
            stakeTableAmountComm,
            threshold
        );
    }
}
