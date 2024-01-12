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

contract StakeTableHandlerTest is Test, StakeTableCommonTest {
    S public stakeTable;
    address public tokenCreator;
    ExampleToken public token;
    mapping(uint256 index => BN254.G2Point vk) public vks;
    BN254.G2Point[] public vksWithdraw;
    LightClientTest public lightClient;
    address[] public users;
    mapping(bytes32 vkHash => uint256 userIndex) public userIndexFromVk;
    uint256 public numberUsers;

    // Variables for testing invariant relative to Register
    uint64 public nextRegistrationEpochBefore;
    uint64 public pendingRegistrationsBefore;
    uint64 public stakeTableFirstAvailableRegistrationEpoch;
    uint64 public stakeTableNumPendingRegistrations;
    bool public registrationSuccessful;

    // Variables for testing invariant relative to requestExit and withdrawFunds
    uint64 public nextExitEpochBefore;
    uint64 public pendingExitsBefore;
    uint64 public stakeTableFirstAvailableExitEpoch;
    uint64 public stakeTableNumPendingExits;
    uint64 public currentEpoch;
    bool public requestExitSuccessful;
    mapping(bytes32 blsKeyHash => uint64 exitEpoch) public exitEpochForBlsVK;
    mapping(uint256 userIndex => bool registered) public isUserRegistered;
    BN254.G2Point[] public requestExitKeys;

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
        uint8 seed = uint8(userIndex);
        address userAddress = users[userIndex];

        (
            BN254.G2Point memory blsVK,
            EdOnBN254.EdOnBN254Point memory schnorrVK,
            BN254.G1Point memory sig
        ) = this.genClientWallet(userAddress, seed);
        uint64 depositAmount = uint64(bound(amount, 1, 10));
        uint64 validUntilEpoch = 100000;

        // Transfer some tokens to userAddress
        vm.prank(sender);
        token.transfer(userAddress, depositAmount);

        // Prepare for the token transfer
        vm.prank(userAddress);
        token.approve(address(stakeTable), depositAmount);

        vm.prank(userAddress);

        bool res = stakeTable.register(
            blsVK,
            schnorrVK,
            depositAmount,
            AbstractStakeTable.StakeType.Native,
            sig,
            validUntilEpoch
        );

        if (res) {
            isUserRegistered[userIndex] = true;
            bytes32 vkHash = stakeTable._hashBlsKey(blsVK);
            userIndexFromVk[vkHash] = userIndex;
            vks[userIndex] = blsVK;
        }

        return res;
    }

    function register(uint8 userIndex, uint64 amount) public {
        // Return if the user is already registered
        userIndex = uint8(bound(userIndex, 0, numberUsers - 1));

        (nextRegistrationEpochBefore, pendingRegistrationsBefore) =
            stakeTable.nextRegistrationEpoch();

        bool res = registerWithSeed(tokenCreator, userIndex, amount);

        stakeTableFirstAvailableRegistrationEpoch = stakeTable.firstAvailableRegistrationEpoch();
        stakeTableNumPendingRegistrations = stakeTable.numPendingRegistrations();

        registrationSuccessful = res;
    }

    function requestExit(uint256 rand) public {
        requestExitSuccessful = false;
        uint256 index = bound(rand, 0, numberUsers - 1);

        (nextExitEpochBefore, pendingExitsBefore) = stakeTable.nextExitEpoch();
        currentEpoch = stakeTable.currentEpoch();

        vm.prank(users[index]);
        BN254.G2Point memory vk = vks[index];
        bool res = stakeTable.requestExit(vk);
        if (res) {
            bytes32 vkHash = stakeTable._hashBlsKey(vk);
            (
                address account,
                AbstractStakeTable.StakeType stakeType,
                uint64 balance,
                uint64 registerEpoch,
                uint64 exitEpoch,
            ) = stakeTable.nodes(vkHash);

            // In order to avoid sol-lint warnings.
            account;
            stakeType;
            balance;
            registerEpoch;

            exitEpochForBlsVK[vkHash] = exitEpoch;
            requestExitKeys.push(vk);

            stakeTableFirstAvailableExitEpoch = stakeTable.firstAvailableExitEpoch();
            stakeTableNumPendingExits = stakeTable.numPendingExits();
        }
        requestExitSuccessful = res;
    }

    function advanceEpoch() public {
        lightClient.setCurrentEpoch(lightClient.currentEpoch() + 1);
    }

    function deposit(uint256 userIndex, uint64 amount) public {
        userIndex = bound(userIndex, 0, numberUsers - 1);
        if (!isUserRegistered[userIndex]) {
            return;
        }
        amount = uint64(bound(amount, 1, 10));
        BN254.G2Point memory vk = vks[userIndex];
        vm.prank(users[userIndex]);
        stakeTable.deposit(vk, amount);
    }

    function withdrawFunds(uint256 rand) public {
        // Check if some withdrawals are possible
        if (requestExitKeys.length == 0) {
            return;
        }

        uint256 index = bound(rand, 0, requestExitKeys.length - 1);
        BN254.G2Point memory vk = requestExitKeys[index];
        bytes32 vkHash = stakeTable._hashBlsKey(vk);
        uint64 exitEpoch = exitEpochForBlsVK[vkHash];

        // Ensure we have reached the right epoch before withdrawing
        uint64 slackForEscrowPeriod = 100;
        uint64 nextEpoch;

        // Move forward to the exit epoch
        nextEpoch = exitEpoch + slackForEscrowPeriod;
        lightClient.setCurrentEpoch(nextEpoch);

        uint256 userIndex = userIndexFromVk[vkHash];
        vm.prank(users[userIndex]);
        stakeTable.withdrawFunds(vk);

        // Remove element from array
        requestExitKeys[index] = requestExitKeys[requestExitKeys.length - 1];
        requestExitKeys.pop();
    }
}

contract StakeTableInvariant_Tests is Test {
    S public stakeTable;
    ExampleToken public token;
    LightClientTest public lightClientContract;
    uint256 public constant INITIAL_BALANCE = 1_000_000_000_000;
    address public exampleTokenCreator;
    address[] public users;
    uint256 public constant NUM_USERS = 10;

    StakeTableHandlerTest public handler;

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
            SafeTransferLib.safeTransfer(token, userAddress, 10000);
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
        uint32 numBlocksPerEpoch = 10;
        uint64 churnRate = 10;
        lightClientContract = new LightClientTest(genesis, numBlocksPerEpoch);
        stakeTable = new S(address(token), address(lightClientContract), churnRate);
        handler = new StakeTableHandlerTest(
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
            assertGe(handler.stakeTableFirstAvailableExitEpoch(), handler.currentEpoch() + 1);
            assertGe(handler.stakeTableNumPendingExits(), 1);
        } else {
            assertGe(handler.stakeTableFirstAvailableExitEpoch(), handler.currentEpoch());
        }
    }

    function invariant_Queue() external {
        // Global invariants
        assertLe(stakeTable.numPendingRegistrations(), stakeTable.maxChurnRate());
        assertLe(stakeTable.numPendingExits(), stakeTable.maxChurnRate());
    }
}
