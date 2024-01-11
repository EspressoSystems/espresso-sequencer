// SPDX-License-Identifier: Unlicensed

/* solhint-disable contract-name-camelcase, func-name-mixedcase, one-contract-per-file */

pragma solidity ^0.8.0;
// Libraries

import "forge-std/Test.sol";

using stdStorage for StdStorage;

import { SafeTransferLib } from "solmate/utils/SafeTransferLib.sol";
import { BN254 } from "bn254/BN254.sol";
import { EdOnBN254 } from "../src/libraries/EdOnBn254.sol";
import { AbstractStakeTable } from "../src/interfaces/AbstractStakeTable.sol";
import { LightClient } from "../src/LightClient.sol";
import { LightClientTest } from "../test/mocks/LightClientTest.sol";
import { StakeTableCommonTest } from "../test/StakeTable.t.sol";

// Token contract
import { ExampleToken } from "../src/ExampleToken.sol";

// Target contract
import { StakeTable as S } from "../src/StakeTable.sol";

contract StakeTableHandler is Test, StakeTableCommonTest {
    S public stakeTable;
    address public tokenCreator;
    ExampleToken public token;
    mapping(uint256 index => BN254.G2Point vk) public vks;
    BN254.G2Point[] public vksWithdraw;
    LightClientTest public lightClient;
    address[] public users;
    uint256 public numberUsers;

    // Variables for testing invariant relative to Register
    uint64 public nextRegistrationEpochBefore;
    uint64 public pendingRegistrationsBefore;
    uint64 public stakeTableFirstAvailableRegistrationEpoch;
    uint64 public stakeTableNumPendingRegistrations;
    bool public registrationSuccessful;

    // Variables for testing invariant relative to requestExit
    uint64 public nextExitEpochBefore;
    uint64 public pendingExitsBefore;
    uint64 public stakeTableFirstAvailableExitEpoch;
    uint64 public stakeTableNumPendingExits;
    bool public requestExitSuccessful;

    constructor(
        S _stakeTable,
        address _tokenCreator,
        ExampleToken _token,
        LightClientTest _lightClient,
        address[] memory _users
    ) {
        stakeTable = _stakeTable;
        token = _token;
        tokenCreator = _tokenCreator;
        lightClient = _lightClient;
        users = _users;
        numberUsers = users.length;
    }

    function registerWithSeed(address sender, uint256 userIndex, uint256 amount)
        private
        returns (bool)
    {
        userIndex = bound(userIndex, 0, numberUsers - 1);

        uint8 seed = uint8(userIndex);
        address userAddress = users[userIndex];

        (
            BN254.G2Point memory blsVK,
            EdOnBN254.EdOnBN254Point memory schnorrVK,
            BN254.G1Point memory sig
        ) = this.genClientWallet(sender, seed);
        uint64 depositAmount = uint64(bound(amount, 0, 100));
        uint64 validUntilEpoch = 1000;

        // Transfer some tokens to sender
        vm.prank(userAddress);
        token.transfer(sender, depositAmount);

        // Prepare for the token transfer
        vm.prank(sender);
        token.approve(address(stakeTable), depositAmount);

        vm.prank(sender);

        bool res = stakeTable.register(
            blsVK,
            schnorrVK,
            depositAmount,
            AbstractStakeTable.StakeType.Native,
            sig,
            validUntilEpoch
        );
        vks[userIndex] = blsVK;

        return res;
    }

    function register(uint8 seed, uint64 amount) public {
        (nextRegistrationEpochBefore, pendingRegistrationsBefore) =
            stakeTable.nextRegistrationEpoch();

        bool res = registerWithSeed(tokenCreator, seed, amount);

        stakeTableFirstAvailableRegistrationEpoch = stakeTable._firstAvailableRegistrationEpoch();
        stakeTableNumPendingRegistrations = stakeTable.numPendingRegistrations();
        registrationSuccessful = res;
    }

    function requestExit(uint256 rand) public {
        uint256 index = bound(rand, 0, numberUsers - 1);

        (nextExitEpochBefore, pendingExitsBefore) = stakeTable.nextExitEpoch();

        vm.prank(users[index]);
        bool res = stakeTable.requestExit(vks[index]);

        stakeTableFirstAvailableExitEpoch = stakeTable._firstAvailableExitEpoch();
        stakeTableNumPendingExits = stakeTable.numPendingExits();
        requestExitSuccessful = res;
    }

    function advanceEpoch() public {
        uint64 currentEpoch = lightClient.currentEpoch();
        uint64 nextEpoch = currentEpoch + 1;
        lightClient.setCurrentEpoch(nextEpoch);
    }

    function withdrawFunds(uint256 rand) public {
        uint256 index = bound(rand, 0, numberUsers - 1);
        BN254.G2Point memory vk = vks[index];

        uint64 currentEpoch = lightClient.currentEpoch();
        uint64 slackForEscrowPeriod = 100;
        uint64 nextEpoch = currentEpoch + slackForEscrowPeriod;
        lightClient.setCurrentEpoch(nextEpoch);

        vm.prank(users[index]);
        stakeTable.withdrawFunds(vk);
    }
}

contract StakeTableInvariant_Tests is Test {
    S public stakeTable;
    ExampleToken public token;
    LightClientTest public lightClientContract;
    uint256 public constant INITIAL_BALANCE = 1_000_000_000;
    address public exampleTokenCreator;
    address[] public users;
    uint256 public constant NUM_USERS = 10;

    StakeTableHandler public handler;

    function setUp() public {
        exampleTokenCreator = makeAddr("tokenCreator");
        vm.prank(exampleTokenCreator);
        token = new ExampleToken(INITIAL_BALANCE);

        address userAddress;

        // Distribute tokens to users
        for (uint256 i = 0; i < NUM_USERS; i++) {
            string memory userLabel = string.concat("user", vm.toString(i));
            userAddress = makeAddr(userLabel);
            vm.label(userAddress, userLabel);
            users.push(userAddress);
            vm.prank(exampleTokenCreator);
            SafeTransferLib.safeTransfer(token, userAddress, 1000);
        }

        LightClient.LightClientState memory genesis = LightClient.LightClientState({
            viewNum: 0,
            blockHeight: 0,
            blockCommRoot: BN254.ScalarField.wrap(1),
            feeLedgerComm: BN254.ScalarField.wrap(1),
            stakeTableBlsKeyComm: BN254.ScalarField.wrap(1),
            stakeTableSchnorrKeyComm: BN254.ScalarField.wrap(1),
            stakeTableAmountComm: BN254.ScalarField.wrap(1),
            threshold: 10
        });
        lightClientContract = new LightClientTest(genesis, 10);
        stakeTable = new S(address(token), address(lightClientContract), 10);
        handler = new StakeTableHandler(
            stakeTable, exampleTokenCreator, token, lightClientContract, users
        );

        // Only test the handler
        targetContract(address(handler));
    }

    function invariant_BalancesAreConsistent() external {
        uint256 totalBalanceUsers = 0;
        for (uint256 i = 0; i < NUM_USERS; i++) {
            totalBalanceUsers += token.balanceOf(users[i]);
        }
        uint256 balanceStakeTable = token.balanceOf(address(stakeTable));
        uint256 tokenCreatorBalance = token.balanceOf(address(exampleTokenCreator));
        assertEq(totalBalanceUsers + balanceStakeTable + tokenCreatorBalance, INITIAL_BALANCE);
    }

    function invariant_Register() external {
        // Here we check that the queue state is updated in a consistent manner with the output
        // of nextExitEpoch.
        if (handler.registrationSuccessful()) {
            assertEq(
                handler.stakeTableFirstAvailableRegistrationEpoch(),
                handler.nextRegistrationEpochBefore()
            );
            assertEq(
                handler.stakeTableNumPendingRegistrations(),
                handler.pendingRegistrationsBefore() + 1
            );
        }
    }

    function invariant_RequestExit() external {
        // Here we check that the queue state is updated in a consistent manner with the output
        // of nextExitEpoch.
        if (handler.requestExitSuccessful()) {
            assertGe(handler.stakeTableFirstAvailableExitEpoch(), stakeTable.currentEpoch() + 1);
            assertGe(handler.stakeTableNumPendingExits(), 1);
        }
    }

    function invariant_Queue() external {
        // Global invariants
        assertLe(stakeTable.numPendingRegistrations(), stakeTable.maxChurnRate());
        assertLe(stakeTable.numPendingExits(), stakeTable.maxChurnRate());
    }
}
