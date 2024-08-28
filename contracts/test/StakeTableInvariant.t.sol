// SPDX-License-Identifier: Unlicensed

/* solhint-disable contract-name-camelcase, func-name-mixedcase, one-contract-per-file */

pragma solidity ^0.8.0;
// Libraries

import "forge-std/Test.sol";

import { BN254 } from "bn254/BN254.sol";
import { EdOnBN254 } from "../src/libraries/EdOnBn254.sol";
import { AbstractStakeTable } from "../src/interfaces/AbstractStakeTable.sol";
import { LightClient } from "../src/LightClient.sol";
import { LightClientMock } from "../test/mocks/LightClientMock.sol";
import { StakeTableCommonTest } from "../test/StakeTable.t.sol";

// Token contract
import { ExampleToken } from "../src/ExampleToken.sol";

// Target contract
import { StakeTable as S } from "../src/StakeTable.sol";

contract StakeTableHandlerTest is StakeTableCommonTest {
    S public stakeTable;
    address public tokenCreator;
    ExampleToken public token;
    mapping(uint256 index => BN254.G2Point vk) public vks;
    BN254.G2Point[] public vksWithdraw;
    LightClientMock public lightClient;
    address[] public users;

    // Variables for testing invariant relative to Register
    uint64 public nextRegistrationEpochBefore;
    uint64 public pendingRegistrationsBefore;
    uint64 public stakeTableFirstAvailableRegistrationEpoch;
    uint64 public stakeTableNumPendingRegistrations;
    bool public registrationCalledAtLeastOnce;

    // Variables for testing invariant relative to requestExit and withdrawFunds
    uint64 public nextExitEpochBefore;
    uint64 public pendingExitsBefore;
    uint64 public stakeTableFirstAvailableExitEpoch;
    uint64 public stakeTableNumPendingExits;
    uint64 public currentEpoch;
    bool public requestExitCalledAtLeastOnce;

    mapping(bytes32 blsKeyHash => uint64 exitEpoch) public exitEpochForBlsVK;
    uint256[] public requestExitKeysIndexes;

    constructor(
        S _stakeTable,
        address _tokenCreator,
        ExampleToken _token,
        LightClientMock _lightClient
    ) {
        stakeTable = _stakeTable;
        token = _token;
        tokenCreator = _tokenCreator;
        lightClient = _lightClient;
        requestExitCalledAtLeastOnce = false;
        registrationCalledAtLeastOnce = false;
    }

    function getNodeAndVKFromUserIndex(uint256 userIndex)
        private
        view
        returns (AbstractStakeTable.Node memory, BN254.G2Point memory)
    {
        BN254.G2Point memory vk = vks[userIndex];
        bytes32 vkHash = stakeTable._hashBlsKey(vk);
        (
            address account,
            AbstractStakeTable.StakeType stakeType,
            uint64 balance,
            uint64 registerEpoch,
            uint64 exitEpoch,
            EdOnBN254.EdOnBN254Point memory schnorrVK
        ) = stakeTable.nodes(vkHash);

        AbstractStakeTable.Node memory node = AbstractStakeTable.Node(
            account, stakeType, balance, registerEpoch, exitEpoch, schnorrVK
        );
        return (node, vk);
    }

    function registerWithSeed(uint256 userIndex, uint256 amount) private {
        uint8 seed = uint8(userIndex);
        address userAddress = users[userIndex];

        (
            BN254.G2Point memory blsVK,
            EdOnBN254.EdOnBN254Point memory schnorrVK,
            BN254.G1Point memory sig
        ) = genClientWallet(userAddress, seed);
        uint64 depositAmount = uint64(bound(amount, 1, 10));
        uint64 validUntilEpoch = 100000;

        // Transfer some tokens to userAddress
        vm.prank(tokenCreator);
        token.transfer(userAddress, depositAmount);

        // Prepare for the token transfer
        vm.prank(userAddress);
        token.approve(address(stakeTable), depositAmount);

        vm.prank(userAddress);

        stakeTable.register(
            blsVK,
            schnorrVK,
            depositAmount,
            AbstractStakeTable.StakeType.Native,
            sig,
            validUntilEpoch
        );

        vks[userIndex] = blsVK;
    }

    function register(uint64 amount) public {
        // Add a new user
        if (users.length == 64) {
            // We use uint8 to encode a user index
            return; // No more users to be processed
        }
        uint256 userIndex = users.length;
        string memory userLabel = string.concat("user", vm.toString(userIndex));
        address userAddress = makeAddr(userLabel);
        vm.label(userAddress, userLabel);
        users.push(userAddress);

        (nextRegistrationEpochBefore, pendingRegistrationsBefore) =
            stakeTable.nextRegistrationEpoch();

        registerWithSeed(userIndex, amount);

        stakeTableFirstAvailableRegistrationEpoch = stakeTable.firstAvailableRegistrationEpoch();
        stakeTableNumPendingRegistrations = stakeTable.numPendingRegistrations();

        registrationCalledAtLeastOnce = true;
    }

    function requestExit(uint256 rand) public {
        if ((users.length) == 0) {
            return;
        }

        uint256 index = bound(rand, 0, users.length - 1);

        (AbstractStakeTable.Node memory node, BN254.G2Point memory vk) =
            getNodeAndVKFromUserIndex(index);

        // An exit request is already pending
        if (node.exitEpoch != 0) {
            return;
        }

        // Check one is early enough to request and exit
        if (currentEpoch < node.registerEpoch + 1) {
            return;
        }

        // At this place of the code the requestExit call will not revert
        currentEpoch = stakeTable.currentEpoch();

        (nextExitEpochBefore, pendingExitsBefore) = stakeTable.nextExitEpoch();

        vm.prank(users[index]);
        stakeTable.requestExit(vk);

        stakeTableFirstAvailableExitEpoch = stakeTable.firstAvailableExitEpoch();
        stakeTableNumPendingExits = stakeTable.numPendingExits();

        bytes32 vkHash = stakeTable._hashBlsKey(vk);
        exitEpochForBlsVK[vkHash] = node.exitEpoch;
        requestExitKeysIndexes.push(index);
        requestExitCalledAtLeastOnce = true;
    }

    function advanceEpoch() public {
        lightClient.setCurrentEpoch(lightClient.currentEpoch() + 1);
    }

    function deposit(uint256 userIndex, uint64 amount) public {
        if ((users.length) == 0) {
            return;
        }
        userIndex = bound(userIndex, 0, users.length - 1);

        (AbstractStakeTable.Node memory node, BN254.G2Point memory vk) =
            getNodeAndVKFromUserIndex(userIndex);

        // Exit if it is too early to deposit
        if (stakeTable.currentEpoch() <= node.registerEpoch) {
            return;
        }

        // Some exit is in progress, do not deposit
        if (node.exitEpoch != 0) {
            return;
        }

        amount = uint64(bound(amount, 1, 10));

        // Exit if the tokenCreator ran out of funds
        if (token.balanceOf(tokenCreator) < amount) {
            return;
        }

        // Transfer some tokens to userAddress
        vm.prank(tokenCreator);
        token.transfer(node.account, amount);

        // Prepare for the token transfer
        vm.prank(node.account);
        token.approve(address(stakeTable), amount);

        vm.prank(node.account);
        stakeTable.deposit(vk, amount);
    }

    function withdrawFunds(uint256 rand) public {
        // Check if some withdrawals are possible
        if (requestExitKeysIndexes.length == 0) {
            return;
        }

        uint256 index = bound(rand, 0, requestExitKeysIndexes.length - 1);

        BN254.G2Point memory vk = vks[requestExitKeysIndexes[index]];
        bytes32 vkHash = stakeTable._hashBlsKey(vk);
        uint64 exitEpoch = exitEpochForBlsVK[vkHash];

        // Ensure we have reached the right epoch before withdrawing
        uint64 slackForEscrowPeriod = 100;
        uint64 nextEpoch;

        // Move forward to the exit epoch
        nextEpoch = exitEpoch + slackForEscrowPeriod;
        lightClient.setCurrentEpoch(nextEpoch);

        uint256 userIndex = requestExitKeysIndexes[index];
        address userAddress = users[userIndex];
        vm.prank(userAddress);
        stakeTable.withdrawFunds(vk);

        // Remove element from array
        requestExitKeysIndexes[index] = requestExitKeysIndexes[requestExitKeysIndexes.length - 1];
        requestExitKeysIndexes.pop();
        exitEpochForBlsVK[vkHash] = 0; // No exit in progress anymore
    }
}

contract StakeTableInvariant_Tests is Test {
    S public stakeTable;
    ExampleToken public token;
    LightClientMock public lightClientContract;
    uint256 public constant INITIAL_BALANCE = 1_000_000_000_000;
    address public exampleTokenCreator;
    address[] public users;

    StakeTableHandlerTest public handler;

    function setUp() public {
        exampleTokenCreator = makeAddr("tokenCreator");
        vm.prank(exampleTokenCreator);
        token = new ExampleToken(INITIAL_BALANCE);

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
        uint32 maxHistorySeconds = 864000;
        uint64 churnRate = 10;

        lightClientContract = new LightClientMock(genesis, maxHistorySeconds);
        stakeTable = new S(address(token), address(lightClientContract), churnRate);
        handler =
            new StakeTableHandlerTest(stakeTable, exampleTokenCreator, token, lightClientContract);

        // Only test the handler
        targetContract(address(handler));
    }

    function invariant_BalancesAreConsistent() external view {
        uint256 totalBalanceUsers = 0;
        for (uint256 i = 0; i < users.length; i++) {
            totalBalanceUsers += token.balanceOf(users[i]);
        }
        uint256 balanceStakeTable = token.balanceOf(address(stakeTable));
        uint256 tokenCreatorBalance = token.balanceOf(address(exampleTokenCreator));
        assertEq(totalBalanceUsers + balanceStakeTable + tokenCreatorBalance, INITIAL_BALANCE);
    }

    function invariant_Register() external view {
        // Here we check that the queue state is updated in a consistent manner with the output
        // of nextExitEpoch.
        if (handler.registrationCalledAtLeastOnce()) {
            assertEq(
                handler.stakeTableFirstAvailableRegistrationEpoch(),
                handler.nextRegistrationEpochBefore()
            );
            assertEq(
                handler.stakeTableNumPendingRegistrations(),
                handler.pendingRegistrationsBefore() + 1
            );
        } else {
            assertEq(handler.stakeTableFirstAvailableRegistrationEpoch(), 0);
            assertEq(handler.stakeTableNumPendingRegistrations(), 0);
        }
    }

    function invariant_RequestExit() external view {
        // Here we check that the queue state is updated in a consistent manner with the output
        // of nextExitEpoch.
        if (handler.requestExitCalledAtLeastOnce()) {
            assertGe(handler.stakeTableFirstAvailableExitEpoch(), handler.currentEpoch() + 1);
            assertGe(handler.stakeTableNumPendingExits(), 1);
        } else {
            assertEq(handler.stakeTableFirstAvailableExitEpoch(), 0);
            assertEq(handler.stakeTableNumPendingExits(), 0);
        }
    }

    function invariant_Queue() external view {
        // Global invariants
        assertLe(stakeTable.numPendingRegistrations(), stakeTable.maxChurnRate());
        assertLe(stakeTable.numPendingExits(), stakeTable.maxChurnRate());
    }
}
