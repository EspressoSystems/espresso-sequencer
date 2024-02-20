// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "forge-std/Script.sol";
import { BN254 } from "bn254/BN254.sol";
import { LightClient as LC } from "../src/LightClient.sol";
import { LightClientTest as LCTest } from "../test/mocks/LightClientTest.sol";

/// @notice deployment script for LightClientTest which is designed for testing purposes with
/// verification key corresponding to smaller circuit, thus faster proof generation during tests.
///
/// @dev for production deployment script, please use `gen-light-client-deploy/main.rs` to
/// generate `LightClient.s.sol` with proper genesis block values.
contract DeployLightClientTestScript is Script {
    function run() external {
        string memory seedPhrase = vm.envString("MNEMONIC");
        uint256 privateKey = vm.deriveKey(seedPhrase, 0);
        vm.startBroadcast(privateKey);

        // For this version there will be only one epoch
        uint32 blocksPerEpoch = type(uint32).max;

        uint64 viewNum = 0;
        uint64 blockHeight = 0;
        BN254.ScalarField blockCommRoot = BN254.ScalarField.wrap(42);
        BN254.ScalarField feeLedgerComm = BN254.ScalarField.wrap(42);
        BN254.ScalarField stakeTableBlsKeyComm = BN254.ScalarField.wrap(
            0x0d168a6fe13a8b344caea06d1fd2a2f1e62eeb7b2b00c6284e094a819a1f1790
        );
        BN254.ScalarField stakeTableSchnorrKeyComm = BN254.ScalarField.wrap(
            0x2e4036835adc3ea606c3cc76870d7ed9fb0ae1ff435d96d480ce42420dfb8026
        );
        BN254.ScalarField stakeTableAmountComm = BN254.ScalarField.wrap(
            0x03be29d3caf0c790070faa48051d86bfd6ec8c4ad4eef24af477741a180ee2ce
        );
        uint256 threshold = 36;

        LC.LightClientState memory genesis = LC.LightClientState(
            viewNum,
            blockHeight,
            blockCommRoot,
            feeLedgerComm,
            stakeTableBlsKeyComm,
            stakeTableSchnorrKeyComm,
            stakeTableAmountComm,
            threshold
        );
        new LCTest(genesis, blocksPerEpoch);

        vm.stopBroadcast();
    }
}
