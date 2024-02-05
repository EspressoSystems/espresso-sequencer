// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "forge-std/Script.sol";
import { BN254 } from "bn254/BN254.sol";
import { LightClient as LC } from "../src/LightClient.sol";

contract DeployLightClientScript is Script {
    function run() external {
        string memory seedPhrase = vm.envString("MNEMONIC");
        uint256 privateKey = vm.deriveKey(seedPhrase, 0);
        vm.startBroadcast(privateKey);

        // For Decaf there will be only one epoch
        uint32 blocksPerEpoch = type(uint32).max;

        // TODO for a production deployment provide the right genesis state
        uint64 viewNum = 0;
        uint64 blockHeight = 0;
        BN254.ScalarField blockCommRoot = BN254.ScalarField.wrap(42);
        BN254.ScalarField feeLedgerComm = BN254.ScalarField.wrap(0);
        BN254.ScalarField stakeTableBlsKeyComm = BN254.ScalarField.wrap(42);
        BN254.ScalarField stakeTableSchnorrKeyComm = BN254.ScalarField.wrap(42);
        BN254.ScalarField stakeTableAmountComm = BN254.ScalarField.wrap(42);
        uint256 threshold = 10;

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
        new LC(genesis, blocksPerEpoch);

        vm.stopBroadcast();
    }
}
