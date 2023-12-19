// SPDX-License-Identifier: Unlicensed

/* solhint-disable contract-name-camelcase, func-name-mixedcase, one-contract-per-file */

pragma solidity ^0.8.0;
// Libraries

import "forge-std/Test.sol";

using stdStorage for StdStorage;

import { ERC20 } from "solmate/utils/SafeTransferLib.sol";
import { BN254 } from "bn254/BN254.sol";
import { BLSSig } from "../src/libraries/BLSSig.sol";
import { EdOnBN254 } from "../src/libraries/EdOnBn254.sol";
import { AbstractStakeTable } from "../src/interfaces/AbstractStakeTable.sol";
import { LightClient } from "../src/LightClient.sol";
import { LightClientTest } from "../test/mocks/LightClientTest.sol";

// Token contract
import { ExampleToken } from "../src/ExampleToken.sol";

// Target contract
import { StakeTable as S } from "../src/StakeTable.sol";

import { CommonBase } from "forge-std/Base.sol";
import { StdCheats } from "forge-std/StdCheats.sol";
import { StdUtils } from "forge-std/StdUtils.sol";

// TODO avoid code duplication with StakeTable.t.sol

contract StakeTableHandler is CommonBase, StdCheats, StdUtils {
    S public stakeTable;
    address tokenCreator;
    ExampleToken public token;
    BN254.G2Point[] vks;
    BN254.G2Point[] vksWithdraw;
    LightClientTest lightClient;

    function genClientWallet(address sender, uint8 seed)
        private
        returns (BN254.G2Point memory, EdOnBN254.EdOnBN254Point memory, BN254.G1Point memory)
    {
        // Generate a BLS signature and other values using rust code
        string[] memory cmds = new string[](4);
        cmds[0] = "diff-test";
        cmds[1] = "gen-client-wallet";
        cmds[2] = vm.toString(sender);
        cmds[3] = vm.toString(seed);

        bytes memory result = vm.ffi(cmds);
        (
            BN254.G1Point memory blsSig,
            BN254.G2Point memory blsVK,
            uint256 schnorrVKx,
            uint256 schnorrVKy
        ) = abi.decode(result, (BN254.G1Point, BN254.G2Point, uint256, uint256));

        return (
            blsVK,
            EdOnBN254.EdOnBN254Point(schnorrVKx, schnorrVKy), // schnorrVK
            blsSig
        );
    }

    constructor(
        S _stakeTable,
        address _tokenCreator,
        ExampleToken _token,
        LightClientTest _lightClient
    ) {
        stakeTable = _stakeTable;
        token = _token;
        tokenCreator = _tokenCreator;
        lightClient = _lightClient;
    }

    function registerWithSeed(address sender, uint8 seed, uint256 amount) private {
        (
            BN254.G2Point memory blsVK,
            EdOnBN254.EdOnBN254Point memory schnorrVK,
            BN254.G1Point memory sig
        ) = genClientWallet(sender, seed);
        uint64 depositAmount = uint64(bound(amount, 0, 100));
        uint64 validUntilEpoch = 1000;

        // Transfer some tokens to sender
        vm.prank(tokenCreator);
        token.transfer(sender, depositAmount);

        // Prepare for the token transfer
        vm.prank(sender);
        token.approve(address(stakeTable), depositAmount);

        vm.prank(sender);

        stakeTable.register(
            blsVK,
            schnorrVK,
            depositAmount,
            AbstractStakeTable.StakeType.Native,
            sig,
            validUntilEpoch
        );
        vks.push(blsVK);
    }

    function register(uint8 seed, uint64 amount) public {
        registerWithSeed(tokenCreator, seed, amount);
    }

    function requestExit(uint256 rand) public {
        uint256 index = bound(rand, 0, vks.length - 1);
        vm.prank(tokenCreator);
        stakeTable.requestExit(vks[index]);
        BN254.G2Point memory vk = vks[index];

        // Remove vk from the list of registered keys
        delete vks[index];

        // Add vk to the list of keys that can allow a withdrawal
        vksWithdraw.push(vk);
    }

    function advanceEpoch() public {
        uint64 currentEpoch = lightClient.currentEpoch();
        uint64 nextEpoch = currentEpoch + 1;
        lightClient.setCurrentEpoch(nextEpoch);
    }

    function withdrawFunds(uint256 rand) public {
        uint256 index = bound(rand, 0, vks.length - 1);
        BN254.G2Point memory vk = vksWithdraw[index];

        uint64 currentEpoch = lightClient.currentEpoch();
        uint64 slackForEscrowPeriod = 100;
        uint64 nextEpoch = currentEpoch + slackForEscrowPeriod;
        lightClient.setCurrentEpoch(nextEpoch);

        vm.prank(tokenCreator);
        stakeTable.withdrawFunds(vk);
        delete vksWithdraw[index];
    }
}

contract StakeTableInvariant_Tests is Test {
    /// forge-config: default.invariant.runs = 256
    /// forge-config: default.invariant.depth = 20

    S public stakeTable;
    ExampleToken public token;
    LightClientTest public lightClientContract;
    uint256 constant INITIAL_BALANCE = 1_000_000_000;
    address exampleTokenCreator;

    StakeTableHandler public handler;

    function setUp() public {
        exampleTokenCreator = makeAddr("tokenCreator");
        vm.prank(exampleTokenCreator);
        token = new ExampleToken(INITIAL_BALANCE);

        LightClient.LightClientState memory genesis = LightClient.LightClientState({
            viewNum: 0,
            blockHeight: 0,
            blockCommRoot: BN254.ScalarField.wrap(0),
            feeLedgerComm: BN254.ScalarField.wrap(0),
            stakeTableBlsKeyComm: BN254.ScalarField.wrap(0),
            stakeTableSchnorrKeyComm: BN254.ScalarField.wrap(0),
            stakeTableAmountComm: BN254.ScalarField.wrap(0),
            threshold: 0
        });
        lightClientContract = new LightClientTest(genesis, 10);
        stakeTable = new S(address(token), address(lightClientContract), 10);
        handler = new StakeTableHandler(stakeTable, exampleTokenCreator, token, lightClientContract);

        // Only test the handler
        targetContract(address(handler));
    }

    function invariant_BalancesAreConsistent() external {
        uint256 balance1 = token.balanceOf(exampleTokenCreator);
        uint256 balance2 = token.balanceOf(address(stakeTable));
        assertEq(balance1 + balance2, INITIAL_BALANCE);
    }

    function invaritant_Queue() external {
        // Global invariants
        assertLe(stakeTable.numPendingRegistrations(), stakeTable.maxChurnRate());
        assertLe(stakeTable.numPendingExits(), stakeTable.maxChurnRate());
    }
}
