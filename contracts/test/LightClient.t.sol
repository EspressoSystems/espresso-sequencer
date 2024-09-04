// SPDX-License-Identifier: Unlicensed

/* solhint-disable contract-name-camelcase, func-name-mixedcase, one-contract-per-file */

pragma solidity ^0.8.0;

// Libraries
import "forge-std/Test.sol";
import { IPlonkVerifier as V } from "../src/interfaces/IPlonkVerifier.sol";

// Target contract
import { LightClient as LC } from "../src/LightClient.sol";
import { LightClientMock as LCMock } from "./mocks/LightClientMock.sol";
import { DeployLightClientTestScript } from "./DeployLightClientTestScript.s.sol";
import { BN254 } from "bn254/BN254.sol";

/// @dev Common helpers for LightClient tests
contract LightClientCommonTest is Test {
    LCMock public lc;
    LC.LightClientState public genesis;
    LC.StakeState public genesisStakeState;
    uint32 public constant DELAY_THRESHOLD = 6;
    uint32 public constant MAX_HISTORY_SECONDS = 1 days;
    uint32 public initialBlockTimestamp = 1 days;
    // this constant should be consistent with `hotshot_contract::light_client.rs`
    uint64 internal constant STAKE_TABLE_CAPACITY = 10;
    DeployLightClientTestScript public deployer = new DeployLightClientTestScript();
    address payable public lcTestProxy;
    address public admin = makeAddr("admin");
    address public permissionedProver = makeAddr("prover");

    function deployAndInitProxy(
        LC.LightClientState memory state,
        LC.StakeState memory stakeState,
        uint32 stateHistoryRetentionPeriod
    ) public returns (address payable, address) {
        vm.warp(1 days);
        //deploy light client test with a proxy
        (lcTestProxy, admin, state) =
            deployer.deployContract(state, stakeState, stateHistoryRetentionPeriod, admin);

        //cast the proxy to be of type light client test
        lc = LCMock(lcTestProxy);

        //set permissioned flag
        vm.expectEmit(true, true, true, true);
        emit LC.PermissionedProverRequired(permissionedProver);
        vm.prank(admin);
        lc.setPermissionedProver(permissionedProver);
        return (lcTestProxy, admin);
    }

    /// @dev initialized ledger like genesis and system params
    function init() public {
        string[] memory cmds = new string[](3);
        cmds[0] = "diff-test";
        cmds[1] = "mock-genesis";
        cmds[2] = vm.toString(STAKE_TABLE_CAPACITY / 2);

        bytes memory result = vm.ffi(cmds);
        (
            LC.LightClientState memory state,
            bytes32 votingSTComm,
            bytes32 frozenSTComm,
            LC.StakeState memory stakeState
        ) = abi.decode(result, (LC.LightClientState, bytes32, bytes32, LC.StakeState));
        genesis = state;
        genesisStakeState = stakeState;

        (lcTestProxy, admin) = deployAndInitProxy(genesis, genesisStakeState, MAX_HISTORY_SECONDS);

        bytes32 expectedStakeTableComm = lc.computeStakeTableComm(genesisStakeState);
        assertEq(votingSTComm, expectedStakeTableComm);
        assertEq(frozenSTComm, expectedStakeTableComm);
        // assertEq(genesisStakeState, lc.genesisStakeState());
    }

    function assertEq(BN254.ScalarField a, BN254.ScalarField b) public pure {
        assertEq(BN254.ScalarField.unwrap(a), BN254.ScalarField.unwrap(b));
    }
}

contract LightClient_constructor_Test is LightClientCommonTest {
    function setUp() public {
        init();
    }

    /// @dev Test the constructor has initialized the contract state properly, especially genesis
    /// block.
    function test_CorrectInitialization() external view {
        assertEq(abi.encode(lc.getGenesisState()), abi.encode(genesis));
        assertEq(abi.encode(lc.getFinalizedState()), abi.encode(genesis));

        bytes32 stakeTableComm = lc.computeStakeTableComm(genesisStakeState);
        assertEq(lc.votingStakeTableCommitment(), stakeTableComm);
        assertEq(lc.frozenStakeTableCommitment(), stakeTableComm);
        assertEq(lc.votingThreshold(), genesisStakeState.threshold);
        assertEq(lc.frozenThreshold(), genesisStakeState.threshold);
    }

    // @dev helper function to be able to initialize the contract and capture the revert error
    function initWithExpectRevert(
        LC.LightClientState memory _genesis,
        LC.StakeState memory _genesisStakeState,
        uint32 _stateHistoryRetentionPeriod
    ) private {
        vm.expectRevert(LC.InvalidArgs.selector);

        lc = new LCMock(_genesis, _genesisStakeState, _stateHistoryRetentionPeriod);
    }

    function test_RevertWhen_InvalidGenesis() external {
        LC.LightClientState memory badGenesis = genesis;
        LC.StakeState memory badGenesisStakeState = genesisStakeState;

        // wrong viewNum would revert
        badGenesis.viewNum = 1;
        initWithExpectRevert(badGenesis, badGenesisStakeState, MAX_HISTORY_SECONDS);
        badGenesis.viewNum = genesis.viewNum; // revert to correct

        // wrong blockHeight would revert
        badGenesis.blockHeight = 1;
        initWithExpectRevert(badGenesis, badGenesisStakeState, MAX_HISTORY_SECONDS);
        badGenesis.blockHeight = genesis.blockHeight; // revert to correct

        // zero-valued stake table commitments would revert
        badGenesisStakeState.stakeTableBlsKeyComm = BN254.ScalarField.wrap(0);
        initWithExpectRevert(badGenesis, badGenesisStakeState, MAX_HISTORY_SECONDS);

        badGenesisStakeState.stakeTableBlsKeyComm = badGenesisStakeState.stakeTableBlsKeyComm; // revert
            // to correct
        badGenesisStakeState.stakeTableSchnorrKeyComm = BN254.ScalarField.wrap(0);
        initWithExpectRevert(badGenesis, badGenesisStakeState, MAX_HISTORY_SECONDS);

        badGenesisStakeState.stakeTableSchnorrKeyComm =
            badGenesisStakeState.stakeTableSchnorrKeyComm; // revert to correct
        badGenesisStakeState.stakeTableAmountComm = BN254.ScalarField.wrap(0);

        initWithExpectRevert(badGenesis, badGenesisStakeState, MAX_HISTORY_SECONDS);
        badGenesisStakeState.stakeTableAmountComm = badGenesisStakeState.stakeTableAmountComm; // revert
            // to correct

        // zero-valued threshold would revert
        badGenesisStakeState.threshold = 0;
        initWithExpectRevert(badGenesis, badGenesisStakeState, MAX_HISTORY_SECONDS);
        badGenesisStakeState.threshold = badGenesisStakeState.threshold; // revert to correct
    }
}

contract LightClient_permissionedProver_Test is LightClientCommonTest {
    LC.LightClientState internal newState;
    V.PlonkProof internal newProof;

    function setUp() public {
        init();

        string[] memory cmds = new string[](5);
        cmds[0] = "diff-test";
        cmds[1] = "mock-consecutive-finalized-states";
        cmds[2] = vm.toString(STAKE_TABLE_CAPACITY / 2);
        cmds[3] = vm.toString(uint64(1));
        cmds[4] = vm.toString(uint64(1));

        bytes memory result = vm.ffi(cmds);
        (LC.LightClientState[] memory states, V.PlonkProof[] memory proofs) =
            abi.decode(result, (LC.LightClientState[], V.PlonkProof[]));

        newState = states[0];
        newProof = proofs[0];
    }

    function test_NoProverPermissionsRequired() external {
        //ensure that the permissioned prover mode is set
        assert(lc.permissionedProverEnabled());

        //set permissioned flag to false
        vm.expectEmit(true, true, true, true);
        emit LC.PermissionedProverNotRequired();
        vm.prank(admin);
        lc.disablePermissionedProverMode();

        //assert that the contract is not permissioned
        assert(lc.permissionedProverEnabled() == false);

        // assert that the prover address is zero address when the contract is not permissioned
        assertEq(lc.permissionedProver(), address(0));

        //any prover can call the newFinalizedState method as the contract is not in permissioned
        // prover mode
        vm.expectEmit(true, true, true, true);
        emit LC.NewState(newState.viewNum, newState.blockHeight, newState.blockCommRoot);
        vm.prank(makeAddr("randomUser"));
        lc.newFinalizedState(newState, newProof);
    }

    function test_UpdatePermissionedProverWhenPermissionedProverModeDisabled() external {
        vm.startPrank(admin);
        vm.expectEmit(true, true, true, true);
        emit LC.PermissionedProverNotRequired();
        lc.disablePermissionedProverMode();
        assertEq(lc.permissionedProver(), address(0));

        address newProver = makeAddr("another prover");
        vm.expectEmit(true, true, true, true);
        emit LC.PermissionedProverRequired(newProver);
        lc.setPermissionedProver(newProver);
        assertEq(newProver, lc.permissionedProver());
        vm.stopPrank();
    }

    function test_UpdatePermissionedProverWhenPermissionedProverModeEnabled() external {
        assert(lc.permissionedProverEnabled());
        assertEq(lc.permissionedProver(), permissionedProver);

        address newProver = makeAddr("another prover");
        vm.expectEmit(true, true, true, true);
        emit LC.PermissionedProverRequired(newProver);
        vm.prank(admin);
        lc.setPermissionedProver(newProver);
        assertEq(newProver, lc.permissionedProver());
    }

    function testFuzz_UpdatePermissionedProverWhenPermissionedProverModeEnabled(address newProver)
        external
    {
        vm.assume(newProver != address(0)); //otherwise it would have reverted with
            // InvalidAddress()
        vm.assume(newProver != permissionedProver); //otherwise it would have reverted with
            // NoChangeRequired()
        assert(lc.permissionedProverEnabled());
        assertEq(lc.permissionedProver(), permissionedProver);

        vm.expectEmit(true, true, true, true);
        emit LC.PermissionedProverRequired(newProver);
        vm.prank(admin);
        lc.setPermissionedProver(newProver);
        assertEq(newProver, lc.permissionedProver());
    }

    function test_OldProverNoLongerWorks() public {
        assertEq(lc.permissionedProver(), permissionedProver);
        address oldPermissionedProver = permissionedProver;

        address prover2 = makeAddr("prover2");
        vm.expectEmit(true, true, true, true);
        emit LC.PermissionedProverRequired(prover2);
        vm.prank(admin);
        lc.setPermissionedProver(prover2);
        assertEq(lc.permissionedProver(), prover2);

        //confirm that the old prover doesn't work
        vm.prank(oldPermissionedProver);
        vm.expectRevert(LC.ProverNotPermissioned.selector);
        lc.newFinalizedState(newState, newProof);

        //confirm that the new prover works
        vm.prank(prover2);
        vm.expectEmit(true, true, true, true);
        emit LC.NewState(newState.viewNum, newState.blockHeight, newState.blockCommRoot);
        lc.newFinalizedState(newState, newProof);
    }

    function test_RevertWhen_sameProverSentInUpdate() public {
        assertEq(lc.permissionedProverEnabled(), true);
        address currentProver = lc.permissionedProver();
        vm.prank(admin);
        vm.expectRevert(LC.NoChangeRequired.selector);
        lc.setPermissionedProver(currentProver);
    }

    function test_RevertWhen_UpdatePermissionedProverToZeroAddress() external {
        vm.expectRevert(LC.InvalidAddress.selector);
        vm.prank(admin);
        lc.setPermissionedProver(address(0));
    }

    function test_RevertWhen_NonAdminTriesToUpdatePermissionedProver() external {
        vm.expectRevert();
        vm.prank(makeAddr("not an admin"));
        lc.setPermissionedProver(makeAddr("new prover"));
    }

    function test_RevertWhen_ProverDoesNotHavePermissions() external {
        vm.expectRevert(LC.ProverNotPermissioned.selector);
        vm.prank(makeAddr("ProverWithNoPermissions"));
        lc.newFinalizedState(newState, newProof);
    }

    function test_RevertWhen_ProverAddressNotPermissionedEvenIfAdminAddress() external {
        vm.expectRevert(LC.ProverNotPermissioned.selector);
        vm.prank(admin);
        lc.newFinalizedState(newState, newProof);
    }
}

contract LightClient_newFinalizedState_Test is LightClientCommonTest {
    function setUp() public {
        init();
    }

    /// @dev for benchmarking purposes only
    function testCorrectUpdate() external {
        // Generating a few consecutive states and proofs
        string[] memory cmds = new string[](5);
        cmds[0] = "diff-test";
        cmds[1] = "mock-consecutive-finalized-states";
        cmds[2] = vm.toString(STAKE_TABLE_CAPACITY / 2);
        cmds[3] = vm.toString(uint64(3));
        cmds[4] = vm.toString(uint64(3));

        bytes memory result = vm.ffi(cmds);
        (LC.LightClientState[] memory states, V.PlonkProof[] memory proofs) =
            abi.decode(result, (LC.LightClientState[], V.PlonkProof[]));
        vm.expectEmit(true, true, true, true);
        emit LC.NewState(states[0].viewNum, states[0].blockHeight, states[0].blockCommRoot);
        vm.prank(permissionedProver);
        lc.newFinalizedState(states[0], proofs[0]);
    }

    /// @dev Test happy path for (the number of states + 1) consecutive new finalized blocks
    /// forge-config: default.fuzz.runs = 1
    /// forge-config: quick.fuzz.runs = 1
    /// forge-config: ci.fuzz.runs = 10
    function testFuzz_ConsecutiveUpdate(
        uint64 numInitValidators,
        uint64 numRegistrations,
        uint64 numExits
    ) external {
        numInitValidators = uint64(bound(numInitValidators, 1, STAKE_TABLE_CAPACITY));
        numRegistrations =
            uint64(bound(numRegistrations, 0, STAKE_TABLE_CAPACITY - numInitValidators));
        numExits = uint64(bound(numExits, 0, numInitValidators));

        // since we have have a fuzzer-provided `numInitValidators`, we should instantiate light
        // client contract separately in each test run
        string[] memory cmds = new string[](3);
        cmds[0] = "diff-test";
        cmds[1] = "mock-genesis";
        cmds[2] = vm.toString(numInitValidators);

        bytes memory result = vm.ffi(cmds);
        (LC.LightClientState memory state,,,) =
            abi.decode(result, (LC.LightClientState, bytes32, bytes32, LC.StakeState));
        genesis = state;
        (lcTestProxy, admin) = deployAndInitProxy(genesis, genesisStakeState, MAX_HISTORY_SECONDS);

        genesis = state;

        // Generating a few consecutive states and proofs
        cmds = new string[](5);
        cmds[0] = "diff-test";
        cmds[1] = "mock-consecutive-finalized-states";
        cmds[2] = vm.toString(numInitValidators);
        cmds[3] = vm.toString(numRegistrations);
        cmds[4] = vm.toString(numExits);

        result = vm.ffi(cmds);
        (LC.LightClientState[] memory states, V.PlonkProof[] memory proofs) =
            abi.decode(result, (LC.LightClientState[], V.PlonkProof[]));

        uint256 statesLen = states.length;

        for (uint256 i = 0; i < statesLen; i++) {
            vm.expectEmit(true, true, true, true);
            emit LC.NewState(states[i].viewNum, states[i].blockHeight, states[i].blockCommRoot);
            vm.prank(permissionedProver);
            lc.newFinalizedState(states[i], proofs[i]);

            assertEq(abi.encode(lc.getFinalizedState()), abi.encode(states[i]));

            bytes32 stakeTableComm = lc.computeStakeTableComm(genesisStakeState);
            assertEq(lc.votingStakeTableCommitment(), stakeTableComm);
            assertEq(lc.frozenStakeTableCommitment(), stakeTableComm);
            assertEq(lc.votingThreshold(), genesisStakeState.threshold);
            assertEq(lc.frozenThreshold(), genesisStakeState.threshold);
        }
    }

    /// @dev Test happy path for updating after skipping a few blocks
    /// forge-config: default.fuzz.runs = 4
    /// forge-config: quick.fuzz.runs = 1
    /// forge-config: ci.fuzz.runs = 10
    function test_UpdateAfterSkippedBlocks(uint32 numBlockSkipped) external {
        numBlockSkipped = uint32(bound(numBlockSkipped, 1, 3));

        // re-assign LightClient with the same genesis
        deployAndInitProxy(genesis, genesisStakeState, MAX_HISTORY_SECONDS);

        string[] memory cmds = new string[](3);
        cmds[0] = "diff-test";
        cmds[1] = "mock-skip-blocks";
        cmds[2] = vm.toString(numBlockSkipped);

        bytes memory result = vm.ffi(cmds);
        (LC.LightClientState memory state, V.PlonkProof memory proof) =
            abi.decode(result, (LC.LightClientState, V.PlonkProof));

        vm.expectEmit(true, true, true, true);
        emit LC.NewState(state.viewNum, state.blockHeight, state.blockCommRoot);
        vm.prank(permissionedProver);
        lc.newFinalizedState(state, proof);

        bytes32 stakeTableComm = lc.computeStakeTableComm(genesisStakeState);
        assertEq(lc.votingStakeTableCommitment(), stakeTableComm);
        assertEq(lc.frozenStakeTableCommitment(), stakeTableComm);
        assertEq(lc.votingThreshold(), genesisStakeState.threshold);
        assertEq(lc.frozenThreshold(), genesisStakeState.threshold);
    }

    /// @dev Test unhappy path when a valid but oudated finalized state is submitted
    function test_RevertWhen_OutdatedStateSubmitted() external {
        uint32 numBlockSkipped = 1;
        string[] memory cmds = new string[](4);
        cmds[0] = "diff-test";
        cmds[1] = "mock-skip-blocks";
        cmds[2] = vm.toString(numBlockSkipped);
        cmds[3] = vm.toString(false);

        bytes memory result = vm.ffi(cmds);
        (LC.LightClientState memory newState, V.PlonkProof memory proof) =
            abi.decode(result, (LC.LightClientState, V.PlonkProof));

        LC.LightClientState memory state = genesis;
        state.viewNum = 10;
        vm.startPrank(permissionedProver);
        lc.setFinalizedState(state);

        // outdated view num
        vm.expectRevert(LC.OutdatedState.selector);
        lc.newFinalizedState(newState, proof);

        // outdated block height
        state.viewNum = genesis.viewNum;
        state.blockHeight = numBlockSkipped + 1;
        vm.expectRevert(LC.OutdatedState.selector);
        lc.newFinalizedState(newState, proof);
        vm.stopPrank();
    }

    /// @dev Test unhappy path when user inputs contain malformed field elements
    function test_RevertWhen_MalformedFieldElements() external {
        uint32 numBlockSkipped = 1;
        string[] memory cmds = new string[](4);
        cmds[0] = "diff-test";
        cmds[1] = "mock-skip-blocks";
        cmds[2] = vm.toString(numBlockSkipped);
        cmds[3] = vm.toString(false);

        bytes memory result = vm.ffi(cmds);
        (LC.LightClientState memory newState, V.PlonkProof memory proof) =
            abi.decode(result, (LC.LightClientState, V.PlonkProof));

        LC.LightClientState memory badState = newState;

        // invalid scalar for blockCommRoot
        vm.startPrank(permissionedProver);
        badState.blockCommRoot = BN254.ScalarField.wrap(BN254.R_MOD);
        vm.expectRevert("Bn254: invalid scalar field");
        lc.newFinalizedState(badState, proof);
        badState.blockCommRoot = newState.blockCommRoot;
    }

    /// @dev Test unhappy path when the plonk proof or the public inputs are wrong
    function test_RevertWhen_WrongProofOrWrongPublicInput() external {
        uint32 numBlockSkipped = 1;
        string[] memory cmds = new string[](4);
        cmds[0] = "diff-test";
        cmds[1] = "mock-skip-blocks";
        cmds[2] = vm.toString(numBlockSkipped);
        cmds[3] = vm.toString(true);

        bytes memory result = vm.ffi(cmds);
        (LC.LightClientState memory newState, V.PlonkProof memory proof) =
            abi.decode(result, (LC.LightClientState, V.PlonkProof));

        BN254.ScalarField randScalar = BN254.ScalarField.wrap(1234);
        LC.LightClientState memory badState = LC.LightClientState({
            viewNum: newState.viewNum,
            blockHeight: newState.blockHeight,
            blockCommRoot: newState.blockCommRoot
        });

        // wrong view num
        vm.startPrank(permissionedProver);
        badState.viewNum = newState.viewNum + 2;
        vm.expectRevert(LC.InvalidProof.selector);
        lc.newFinalizedState(badState, proof);
        badState.viewNum = newState.viewNum;

        // wrong block height
        badState.blockHeight = newState.blockHeight + 1;
        vm.expectRevert(LC.InvalidProof.selector);
        lc.newFinalizedState(badState, proof);
        badState.blockHeight = newState.blockHeight;

        // wrong blockCommRoot
        badState.blockCommRoot = randScalar;
        vm.expectRevert(LC.InvalidProof.selector);
        lc.newFinalizedState(badState, proof);
        badState.blockCommRoot = newState.blockCommRoot;

        cmds = new string[](3);
        cmds[0] = "diff-test";
        cmds[1] = "dummy-proof";
        cmds[2] = vm.toString(uint64(42));

        result = vm.ffi(cmds);
        (V.PlonkProof memory dummyProof) = abi.decode(result, (V.PlonkProof));
        vm.expectRevert(LC.InvalidProof.selector);
        lc.newFinalizedState(newState, dummyProof);

        vm.stopPrank();
    }

    /// @dev Test that update on finalized state will fail if a different stake table is used
    function test_revertWhenWrongStakeTableUsed() external {
        string[] memory cmds = new string[](2);
        cmds[0] = "diff-test";
        cmds[1] = "mock-wrong-stake-table";

        bytes memory result = vm.ffi(cmds);
        (LC.LightClientState memory newState, V.PlonkProof memory proof) =
            abi.decode(result, (LC.LightClientState, V.PlonkProof));

        LC.LightClientState memory newLCState = LC.LightClientState({
            viewNum: newState.viewNum,
            blockHeight: newState.blockHeight,
            blockCommRoot: newState.blockCommRoot
        });

        vm.expectRevert(LC.InvalidProof.selector);
        vm.prank(permissionedProver);
        lc.newFinalizedState(newLCState, proof);
    }
}

contract LightClient_StateUpdatesTest is LightClientCommonTest {
    LC.LightClientState internal newState;
    V.PlonkProof internal newProof;

    function assertInitialStateHistoryConditions() internal view {
        // assert that stateHistoryFirstIndex starts at 0.
        assertEq(lc.stateHistoryFirstIndex(), 0);
        // asset stateHistoryRetentionPeriod is greater or equal to at least one day in seconds.
        assertGe(lc.stateHistoryRetentionPeriod(), 1 days);
    }

    function setstateHistoryRetentionPeriod(uint32 duration) internal {
        // Set the new max block states allowed to half of the number of states available
        vm.prank(admin);
        lc.setstateHistoryRetentionPeriod(duration);
        assertEq(lc.stateHistoryRetentionPeriod(), duration);
    }

    /**
     * Liveness test cases to consider
     * Outside of HotShot threshold, revert
     * OnlyOneUpdate - HotShot is live
     * OnlyTwoUpdates - HotShot is live unless blockNumber is past the 2nd blockupdate and past the
     * threshold
     */
    function setUp() public {
        init();
        // Assert owner is correctly set, add this to check owner state
        assertEq(lc.owner(), admin, "Admin should be the owner.");

        string[] memory cmds = new string[](5);
        cmds[0] = "diff-test";
        cmds[1] = "mock-consecutive-finalized-states";
        cmds[2] = vm.toString(STAKE_TABLE_CAPACITY / 2);
        cmds[3] = vm.toString(uint64(1));
        cmds[4] = vm.toString(uint64(1));

        //assert initial conditions
        assertEq(lc.stateHistoryFirstIndex(), 0);
        assertGe(lc.stateHistoryRetentionPeriod(), 1 days);

        bytes memory result = vm.ffi(cmds);
        (LC.LightClientState[] memory states, V.PlonkProof[] memory proofs) =
            abi.decode(result, (LC.LightClientState[], V.PlonkProof[]));

        newState = states[1];
        newProof = proofs[1];
    }

    function test_1lBlockUpdatesIsUpdated() public {
        uint256 blockUpdatesCount = lc.getStateHistoryCount();

        // Update the state and thus the l1BlockUpdates array would be updated
        vm.prank(permissionedProver);
        vm.expectEmit(true, true, true, true);
        emit LC.NewState(newState.viewNum, newState.blockHeight, newState.blockCommRoot);
        lc.newFinalizedState(newState, newProof);

        assertEq(lc.getStateHistoryCount(), blockUpdatesCount + 1);
    }

    function testFuzz_setstateHistoryRetentionPeriod(uint32 stateHistoryRetentionPeriod) public {
        vm.prank(admin);
        vm.assume(stateHistoryRetentionPeriod > 1 days);
        lc.setstateHistoryRetentionPeriod(stateHistoryRetentionPeriod);
        assertEq(stateHistoryRetentionPeriod, lc.stateHistoryRetentionPeriod());
    }

    function test_revertNonAdminSetMaxStateHistoryAllowed() public {
        vm.expectRevert();
        lc.setstateHistoryRetentionPeriod(1 days);
    }

    function test_revertSetMaxStateHistoryAllowedWhenInvalidValueSent() public {
        // revert when a retention period less than the minimum of 1 hour is sent
        vm.prank(admin);
        vm.expectRevert(LC.InvalidMaxStateHistory.selector);
        lc.setstateHistoryRetentionPeriod(1 hours - 1);

        // revert when a smaller retention period is sent
        uint32 currentRetentionPeriod = lc.stateHistoryRetentionPeriod();
        vm.prank(admin);
        vm.expectRevert(LC.InvalidMaxStateHistory.selector);
        lc.setstateHistoryRetentionPeriod(currentRetentionPeriod - 1);
    }

    function test_stateHistoryHandlingWithOneDayMaxHistory() public {
        string[] memory cmds = new string[](5);
        cmds[0] = "diff-test";
        cmds[1] = "mock-consecutive-finalized-states";
        cmds[2] = vm.toString(STAKE_TABLE_CAPACITY / 2);
        cmds[3] = vm.toString(uint64(5));
        cmds[4] = vm.toString(uint64(5));

        uint32 numDays = 1;

        assertInitialStateHistoryConditions();

        bytes memory result = vm.ffi(cmds);
        (LC.LightClientState[] memory states, V.PlonkProof[] memory proofs) =
            abi.decode(result, (LC.LightClientState[], V.PlonkProof[]));

        // Add one numDays worth of a new state
        uint256 i;
        for (i = 0; i < numDays; i++) {
            vm.warp(initialBlockTimestamp + ((i + 1) * 1 days)); // increase the timestamp for each
            vm.prank(permissionedProver);
            vm.expectEmit(true, true, true, true);
            emit LC.NewState(states[i].viewNum, states[i].blockHeight, states[i].blockCommRoot);
            lc.newFinalizedState(states[i], proofs[i]);
        }

        // assert that the first index is still zero as
        // the number of duration between the 1st and last elements  are equal to the max state
        // history duration
        assertEq(lc.stateHistoryFirstIndex(), 0);

        // get oldest and newest state commitment info
        (, uint256 latestBlockTimestamp,) =
            lc.stateHistoryCommitments(lc.getStateHistoryCount() - 1);
        (, uint256 oldestBlockTimestamp,) = lc.stateHistoryCommitments(lc.stateHistoryFirstIndex());
        // assert that the latest Commitment timestamp - oldest Commitment timestamp is == the max
        // history allowed
        assertEq(latestBlockTimestamp - oldestBlockTimestamp, lc.stateHistoryRetentionPeriod());

        // Add a new state so that the state history duration is only surpassed by one element
        vm.prank(permissionedProver);
        vm.expectEmit(true, true, true, true);
        emit LC.NewState(states[i].viewNum, states[i].blockHeight, states[i].blockCommRoot);
        vm.warp(initialBlockTimestamp + ((i + 1) * 1 days)); // increase the timestamp for each
        lc.newFinalizedState(states[i], proofs[i]);
        i++;

        // the duration between the updates are more than stateHistoryRetentionPeriod,  so the first
        // index should be one
        assertEq(lc.stateHistoryFirstIndex(), 1);

        // continue updating the state
        for (uint256 j = i; j < states.length; j++) {
            vm.warp(initialBlockTimestamp + ((j + 1) * 1 days)); // increase the timestamp for each
            vm.prank(permissionedProver);
            vm.expectEmit(true, true, true, true);
            emit LC.NewState(states[j].viewNum, states[j].blockHeight, states[j].blockCommRoot);
            lc.newFinalizedState(states[j], proofs[j]);
        }

        // get stale commitments and assert that it has been reset to zero
        for (i = 0; i < lc.stateHistoryFirstIndex(); i++) {
            (, uint256 staleBlockTimestamp,) = lc.stateHistoryCommitments(i);
            assertEq(staleBlockTimestamp, 0);
        }

        // get the recent commitments and assert that the values are non-zero
        for (i = lc.stateHistoryFirstIndex(); i < lc.getStateHistoryCount(); i++) {
            (, uint256 activeBlockTimestamp,) = lc.stateHistoryCommitments(i);
            assertNotEq(activeBlockTimestamp, 0);
        }
    }

    function test_stateHistoryHandlingWithTwoDaysMaxHistory() public {
        string[] memory cmds = new string[](5);
        cmds[0] = "diff-test";
        cmds[1] = "mock-consecutive-finalized-states";
        cmds[2] = vm.toString(STAKE_TABLE_CAPACITY / 2);
        cmds[3] = vm.toString(uint64(5));
        cmds[4] = vm.toString(uint64(5));
        uint32 numDays = 2;

        bytes memory result = vm.ffi(cmds);
        (LC.LightClientState[] memory states, V.PlonkProof[] memory proofs) =
            abi.decode(result, (LC.LightClientState[], V.PlonkProof[]));

        assertInitialStateHistoryConditions();

        // set the new max block states allowed to half of the number of states available
        setstateHistoryRetentionPeriod(1 days * numDays);
        assertEq(lc.stateHistoryRetentionPeriod(), 1 days * numDays);

        // Update the states to max state history allowed
        uint256 i;
        for (i = 0; i < numDays; i++) {
            vm.warp(initialBlockTimestamp + ((i + 1) * 1 days)); // increase the timestamp for each
            vm.prank(permissionedProver);
            vm.expectEmit(true, true, true, true);
            emit LC.NewState(states[i].viewNum, states[i].blockHeight, states[i].blockCommRoot);
            lc.newFinalizedState(states[i], proofs[i]);
        }

        // the number of elements are equal to the max state history so the first index should still
        // be zero
        assertEq(lc.stateHistoryFirstIndex(), 0);

        // get oldest and newest state commitment info
        (, uint256 latestBlockTimestamp,) =
            lc.stateHistoryCommitments(lc.getStateHistoryCount() - 1);
        (, uint256 oldestBlockTimestamp,) = lc.stateHistoryCommitments(lc.stateHistoryFirstIndex());
        // assert that the latest Commitment timestamp - oldest Commitment timestamp is == the max
        // history allowed

        assertEq(latestBlockTimestamp - oldestBlockTimestamp, lc.stateHistoryRetentionPeriod());

        // Add a new state so that the state history duration is only surpassed by one element
        vm.prank(permissionedProver);
        vm.expectEmit(true, true, true, true);
        emit LC.NewState(states[i].viewNum, states[i].blockHeight, states[i].blockCommRoot);
        vm.warp(initialBlockTimestamp + ((i + 1) * 1 days)); // increase the timestamp for each
        lc.newFinalizedState(states[i], proofs[i]);
        i++;

        // the duration between the updates are more than stateHistoryRetentionPeriod,  so the first
        // index should be one
        assertEq(lc.stateHistoryFirstIndex(), 1);

        // continue updating the state
        for (uint256 j = i; j < states.length; j++) {
            vm.warp(initialBlockTimestamp + ((j + 1) * 1 days)); // increase the timestamp for each
            vm.prank(permissionedProver);
            vm.expectEmit(true, true, true, true);
            emit LC.NewState(states[j].viewNum, states[j].blockHeight, states[j].blockCommRoot);
            lc.newFinalizedState(states[j], proofs[j]);
        }

        // get stale commitments and assert that it has been reset to zero
        for (i = 0; i < lc.stateHistoryFirstIndex(); i++) {
            (, uint256 staleBlockTimestamp,) = lc.stateHistoryCommitments(i);
            assertEq(staleBlockTimestamp, 0);
        }

        // get the recent commitments and assert that the values are non-zero
        for (i = lc.stateHistoryFirstIndex(); i < lc.getStateHistoryCount(); i++) {
            (, uint256 activeBlockTimestamp,) = lc.stateHistoryCommitments(i);
            assertNotEq(activeBlockTimestamp, 0);
        }
    }

    function test_hotshotIsLiveFunctionWhenNoDelayOccurred() public {
        // DELAY_THRESHOLD = 6
        uint8 numUpdates = 5;

        uint64[] memory blockNumberUpdates = new uint64[](numUpdates);
        blockNumberUpdates[0] = 1;
        blockNumberUpdates[1] = blockNumberUpdates[0] + DELAY_THRESHOLD / 2; // 4
        blockNumberUpdates[2] = blockNumberUpdates[1] + DELAY_THRESHOLD / 2; // 7
        blockNumberUpdates[3] = blockNumberUpdates[2] + DELAY_THRESHOLD + 5; // 18
        blockNumberUpdates[4] = blockNumberUpdates[3] + DELAY_THRESHOLD / 2; // 21

        uint64[] memory blockTimestampUpdates = new uint64[](numUpdates);
        for (uint8 i = 0; i < numUpdates; i++) {
            blockTimestampUpdates[i] = initialBlockTimestamp + ((i + 1) * 1 days);
        }

        LC.HotShotCommitment memory hotShotCommitment =
            LC.HotShotCommitment(newState.blockHeight, newState.blockCommRoot);

        LC.StateHistoryCommitment[] memory stateHistoryCommitments =
            new LC.StateHistoryCommitment[](numUpdates);

        for (uint256 i = 0; i < blockNumberUpdates.length; i++) {
            stateHistoryCommitments[i] = LC.StateHistoryCommitment({
                l1BlockHeight: blockNumberUpdates[i],
                l1BlockTimestamp: blockTimestampUpdates[i],
                hotShotCommitment: hotShotCommitment
            });
        }

        lc.setStateHistory(stateHistoryCommitments);

        // set the current block to block number larger than the l1 block numbers used in this test
        vm.roll(blockNumberUpdates[4] + (DELAY_THRESHOLD * 5));

        assertEq(lc.getStateHistoryCount(), numUpdates);

        // Reverts as it's within the first two updates which aren't valid times to check since it
        // was just getting initialized
        vm.expectRevert(LC.InsufficientSnapshotHistory.selector);
        lc.lagOverEscapeHatchThreshold(blockNumberUpdates[1] - 1, DELAY_THRESHOLD);

        // Hotshot should be live (l1BlockNumber = 7)
        assertFalse(lc.lagOverEscapeHatchThreshold(blockNumberUpdates[2], DELAY_THRESHOLD));
    }

    function test_hotshotIsDownWhenADelayExists() public {
        // DELAY_THRESHOLD = 6
        uint8 numUpdates = 5;
        uint64[] memory blockNumberUpdates = new uint64[](numUpdates);
        blockNumberUpdates[0] = 1;
        blockNumberUpdates[1] = blockNumberUpdates[0] + DELAY_THRESHOLD / 2; // 4
        blockNumberUpdates[2] = blockNumberUpdates[1] + DELAY_THRESHOLD / 2; // 7
        blockNumberUpdates[3] = blockNumberUpdates[2] + DELAY_THRESHOLD + 5; // 18
        blockNumberUpdates[4] = blockNumberUpdates[3] + DELAY_THRESHOLD / 2; // 21

        LC.HotShotCommitment memory hotShotCommitment =
            LC.HotShotCommitment(newState.blockHeight, newState.blockCommRoot);

        LC.StateHistoryCommitment[] memory stateHistoryCommitments =
            new LC.StateHistoryCommitment[](numUpdates);

        uint64[] memory blockTimestampUpdates = new uint64[](numUpdates);
        for (uint8 i = 0; i < numUpdates; i++) {
            blockTimestampUpdates[i] = initialBlockTimestamp + ((i + 1) * 1 days);
        }

        for (uint256 i = 0; i < blockNumberUpdates.length; i++) {
            stateHistoryCommitments[i] = LC.StateHistoryCommitment({
                l1BlockHeight: blockNumberUpdates[i],
                l1BlockTimestamp: blockTimestampUpdates[i],
                hotShotCommitment: hotShotCommitment
            });
        }

        lc.setStateHistory(stateHistoryCommitments);

        // set the current block to block number larger than the l1 block numbers used in this test
        vm.roll(blockNumberUpdates[4] + (DELAY_THRESHOLD * 5));

        // Hotshot should be down (l1BlockNumber = 15)
        // for a block that should have been recorded but wasn't due to a delay
        assertTrue(
            lc.lagOverEscapeHatchThreshold(
                blockNumberUpdates[2] + DELAY_THRESHOLD + 2, DELAY_THRESHOLD
            )
        );
    }

    function test_revertWhenThereAreOnlyTwoUpdates() public {
        uint8 numUpdates = 2;

        uint64[] memory updates = new uint64[](numUpdates);
        updates[0] = 1;
        updates[1] = updates[0] + DELAY_THRESHOLD + 5; //12

        LC.HotShotCommitment memory hotShotCommitment =
            LC.HotShotCommitment(newState.blockHeight, newState.blockCommRoot);

        LC.StateHistoryCommitment[] memory stateHistoryCommitments =
            new LC.StateHistoryCommitment[](numUpdates);

        uint64[] memory blockTimestampUpdates = new uint64[](numUpdates);
        for (uint8 i = 0; i < numUpdates; i++) {
            blockTimestampUpdates[i] = initialBlockTimestamp + ((i + 1) * 1 days);
        }

        for (uint256 i = 0; i < updates.length; i++) {
            stateHistoryCommitments[i] = LC.StateHistoryCommitment({
                l1BlockHeight: updates[i],
                l1BlockTimestamp: blockTimestampUpdates[i],
                hotShotCommitment: hotShotCommitment
            });
        }

        lc.setStateHistory(stateHistoryCommitments);

        vm.roll(DELAY_THRESHOLD * 5);

        assertEq(lc.getStateHistoryCount(), numUpdates);

        vm.expectRevert(LC.InsufficientSnapshotHistory.selector);
        lc.lagOverEscapeHatchThreshold(updates[0] + 2, DELAY_THRESHOLD); //3
    }

    function test_revertWhenThereIsOnlyOneUpdate() public {
        uint8 numUpdates = 1;

        uint64[] memory updates = new uint64[](numUpdates);
        updates[0] = 1;

        LC.HotShotCommitment memory hotShotCommitment =
            LC.HotShotCommitment(newState.blockHeight, newState.blockCommRoot);

        LC.StateHistoryCommitment[] memory stateHistoryCommitments =
            new LC.StateHistoryCommitment[](numUpdates);

        uint64[] memory blockTimestampUpdates = new uint64[](numUpdates);
        for (uint8 i = 0; i < numUpdates; i++) {
            blockTimestampUpdates[i] = initialBlockTimestamp + ((i + 1) * 1 days);
        }

        for (uint256 i = 0; i < updates.length; i++) {
            stateHistoryCommitments[i] = LC.StateHistoryCommitment({
                l1BlockHeight: updates[i],
                l1BlockTimestamp: blockTimestampUpdates[i],
                hotShotCommitment: hotShotCommitment
            });
        }

        lc.setStateHistory(stateHistoryCommitments);

        vm.roll(DELAY_THRESHOLD * 3);

        assertEq(lc.getStateHistoryCount(), numUpdates);

        vm.expectRevert(LC.InsufficientSnapshotHistory.selector);
        lc.lagOverEscapeHatchThreshold(updates[0] + 2, DELAY_THRESHOLD); //3
    }

    function test_revertWhenBlockRequestedWithinFirstTwoUpdates() public {
        // DELAY_THRESHOLD = 6
        uint8 numUpdates = 3;

        uint64[] memory updates = new uint64[](numUpdates);
        updates[0] = 1;
        updates[1] = updates[0] + DELAY_THRESHOLD / 2; // 4
        updates[2] = updates[1] + DELAY_THRESHOLD / 2; // 7

        LC.HotShotCommitment memory hotShotCommitment =
            LC.HotShotCommitment(newState.blockHeight, newState.blockCommRoot);

        LC.StateHistoryCommitment[] memory stateHistoryCommitments =
            new LC.StateHistoryCommitment[](numUpdates);

        uint64[] memory blockTimestampUpdates = new uint64[](numUpdates);
        for (uint8 i = 0; i < numUpdates; i++) {
            blockTimestampUpdates[i] = initialBlockTimestamp + ((i + 1) * 1 days);
        }

        for (uint256 i = 0; i < updates.length; i++) {
            stateHistoryCommitments[i] = LC.StateHistoryCommitment({
                l1BlockHeight: updates[i],
                l1BlockTimestamp: blockTimestampUpdates[i],
                hotShotCommitment: hotShotCommitment
            });
        }

        lc.setStateHistory(stateHistoryCommitments);

        vm.roll(DELAY_THRESHOLD * 5);

        assertEq(lc.getStateHistoryCount(), numUpdates);

        vm.expectRevert(LC.InsufficientSnapshotHistory.selector);
        lc.lagOverEscapeHatchThreshold(updates[0] + 2, DELAY_THRESHOLD); //3
    }

    function test_revertWhenSetZeroMaxStateUpdatesAllowed() public {
        vm.prank(admin);
        vm.expectRevert(LC.InvalidMaxStateHistory.selector);
        lc.setstateHistoryRetentionPeriod(0);
    }

    function test_hotShotIsDownWhenBlockIsHigherThanLastRecordedAndTheDelayThresholdHasPassed()
        public
    {
        uint8 numUpdates = 3;

        // DELAY_THRESHOLD = 6
        uint64[] memory updates = new uint64[](numUpdates);
        updates[0] = 1;
        updates[1] = updates[0] + DELAY_THRESHOLD / 2; // 4
        updates[2] = updates[1] + DELAY_THRESHOLD / 2; // 7

        LC.HotShotCommitment memory hotShotCommitment =
            LC.HotShotCommitment(newState.blockHeight, newState.blockCommRoot);

        LC.StateHistoryCommitment[] memory stateHistoryCommitments =
            new LC.StateHistoryCommitment[](numUpdates);

        uint64[] memory blockTimestampUpdates = new uint64[](numUpdates);
        for (uint8 i = 0; i < numUpdates; i++) {
            blockTimestampUpdates[i] = initialBlockTimestamp + ((i + 1) * 1 days);
        }

        for (uint256 i = 0; i < updates.length; i++) {
            stateHistoryCommitments[i] = LC.StateHistoryCommitment({
                l1BlockHeight: updates[i],
                l1BlockTimestamp: blockTimestampUpdates[i],
                hotShotCommitment: hotShotCommitment
            });
        }

        lc.setStateHistory(stateHistoryCommitments);
        // set the current block to block number larger than the l1 block numbers used in this test
        vm.roll(updates[2] + (DELAY_THRESHOLD * 5));

        // Hotshot should be down (l1BlockNumber = 29)
        // in a block that's higher than the last recorded and past the delay threshold
        assertTrue(
            lc.lagOverEscapeHatchThreshold(updates[2] + DELAY_THRESHOLD + 3, DELAY_THRESHOLD)
        );
    }

    function test_hotShotIsLiveWhenBlockIsHigherThanLastRecordedAndTheDelayThresholdHasNotPassed()
        public
    {
        // DELAY_THRESHOLD = 6
        uint8 numUpdates = 3;

        uint64[] memory updates = new uint64[](numUpdates);
        updates[0] = 1;
        updates[1] = updates[0] + DELAY_THRESHOLD / 2; // 4
        updates[2] = updates[1] + DELAY_THRESHOLD / 2; // 7

        LC.HotShotCommitment memory hotShotCommitment =
            LC.HotShotCommitment(newState.blockHeight, newState.blockCommRoot);

        LC.StateHistoryCommitment[] memory stateHistoryCommitments =
            new LC.StateHistoryCommitment[](numUpdates);

        uint64[] memory blockTimestampUpdates = new uint64[](numUpdates);
        for (uint8 i = 0; i < numUpdates; i++) {
            blockTimestampUpdates[i] = initialBlockTimestamp + ((i + 1) * 1 days);
        }

        for (uint256 i = 0; i < updates.length; i++) {
            stateHistoryCommitments[i] = LC.StateHistoryCommitment({
                l1BlockHeight: updates[i],
                l1BlockTimestamp: blockTimestampUpdates[i],
                hotShotCommitment: hotShotCommitment
            });
        }

        lc.setStateHistory(stateHistoryCommitments);

        // set the current block to block number larger than the l1 block numbers used in this test
        vm.roll(updates[2] + (DELAY_THRESHOLD * 5));

        // Hotshot should be live (l1BlockNumber = 24)
        assertFalse(lc.lagOverEscapeHatchThreshold(updates[2] + 3, DELAY_THRESHOLD));
    }

    function test_revertWhenBlockInFuture() public {
        // DELAY_THRESHOLD = 6
        uint8 numUpdates = 2;

        uint64[] memory updates = new uint64[](numUpdates);
        updates[0] = 1;
        updates[1] = updates[0] + DELAY_THRESHOLD / 2; // 4

        LC.HotShotCommitment memory hotShotCommitment =
            LC.HotShotCommitment(newState.blockHeight, newState.blockCommRoot);

        LC.StateHistoryCommitment[] memory stateHistoryCommitments =
            new LC.StateHistoryCommitment[](numUpdates);

        uint64[] memory blockTimestampUpdates = new uint64[](numUpdates);
        for (uint8 i = 0; i < numUpdates; i++) {
            blockTimestampUpdates[i] = initialBlockTimestamp + ((i + 1) * 1 days);
        }

        for (uint256 i = 0; i < updates.length; i++) {
            stateHistoryCommitments[i] = LC.StateHistoryCommitment({
                l1BlockHeight: updates[i],
                l1BlockTimestamp: blockTimestampUpdates[i],
                hotShotCommitment: hotShotCommitment
            });
        }

        lc.setStateHistory(stateHistoryCommitments);

        // set the current block
        uint256 currBlock = 20;
        vm.roll(currBlock);

        vm.expectRevert(LC.InsufficientSnapshotHistory.selector);

        lc.lagOverEscapeHatchThreshold(currBlock + 5, DELAY_THRESHOLD);
    }

    function test_revertWhenRequestedBlockIsBeforeHotShotFirstBlock() public {
        // DELAY_THRESHOLD = 6
        uint8 numUpdates = 2;

        uint64[] memory updates = new uint64[](numUpdates);
        updates[0] = 1;
        updates[1] = updates[0] + DELAY_THRESHOLD / 2; // 4

        LC.HotShotCommitment memory hotShotCommitment =
            LC.HotShotCommitment(newState.blockHeight, newState.blockCommRoot);

        LC.StateHistoryCommitment[] memory stateHistoryCommitments =
            new LC.StateHistoryCommitment[](numUpdates);

        uint64[] memory blockTimestampUpdates = new uint64[](numUpdates);
        for (uint8 i = 0; i < numUpdates; i++) {
            blockTimestampUpdates[i] = initialBlockTimestamp + ((i + 1) * 1 days);
        }

        for (uint256 i = 0; i < updates.length; i++) {
            stateHistoryCommitments[i] = LC.StateHistoryCommitment({
                l1BlockHeight: updates[i],
                l1BlockTimestamp: blockTimestampUpdates[i],
                hotShotCommitment: hotShotCommitment
            });
        }

        lc.setStateHistory(stateHistoryCommitments);

        // set the current block
        uint256 currBlock = 20;
        vm.roll(currBlock);

        vm.expectRevert(LC.InsufficientSnapshotHistory.selector);

        lc.lagOverEscapeHatchThreshold(updates[0] - 1, DELAY_THRESHOLD);
    }
}

contract LightClient_HotShotCommUpdatesTest is LightClientCommonTest {
    LC.LightClientState internal newState;
    V.PlonkProof internal newProof;

    /**
     * Liveness test cases to consider
     * Outside of HotShot threshold, revert
     * OnlyOneUpdate - HotShot is live
     * OnlyTwoUpdates - HotShot is live unless blockNumber is past the 2nd blockupdate and past the
     * threshold
     */
    function setUp() public {
        init();
        // Assert owner is correctly set, add this to check owner state
        assertEq(lc.owner(), admin, "Admin should be the owner.");

        string[] memory cmds = new string[](5);
        cmds[0] = "diff-test";
        cmds[1] = "mock-consecutive-finalized-states";
        cmds[2] = vm.toString(STAKE_TABLE_CAPACITY / 2);
        cmds[3] = vm.toString(uint64(1));
        cmds[4] = vm.toString(uint64(1));

        bytes memory result = vm.ffi(cmds);
        (LC.LightClientState[] memory _states, V.PlonkProof[] memory _proofs) =
            abi.decode(result, (LC.LightClientState[], V.PlonkProof[]));

        newState = _states[1];
        newProof = _proofs[1];
    }

    function assertEqBN254(BN254.ScalarField a, BN254.ScalarField b) public pure {
        assertEq(BN254.ScalarField.unwrap(a), BN254.ScalarField.unwrap(b));
    }

    function assertNotEqBN254(BN254.ScalarField a, BN254.ScalarField b) public pure {
        assertNotEq(BN254.ScalarField.unwrap(a), BN254.ScalarField.unwrap(b));
    }

    function test_hotShotBlockCommIsUpdated() public {
        uint256 blockCommCount = lc.getStateHistoryCount();

        // Update the state and thus the l1BlockUpdates array would be updated
        vm.prank(permissionedProver);
        vm.expectEmit(true, true, true, true);
        emit LC.NewState(newState.viewNum, newState.blockHeight, newState.blockCommRoot);
        lc.newFinalizedState(newState, newProof);

        assertEq(lc.getStateHistoryCount(), blockCommCount + 1);
    }

    function test_hotShotBlockCommIsUpdatedXTimes() public {
        uint256 blockCommCount = lc.getStateHistoryCount();

        string[] memory cmds = new string[](5);
        cmds[0] = "diff-test";
        cmds[1] = "mock-consecutive-finalized-states";
        cmds[2] = vm.toString(STAKE_TABLE_CAPACITY / 2);
        cmds[3] = vm.toString(uint64(1));
        cmds[4] = vm.toString(uint64(1));

        bytes memory result = vm.ffi(cmds);
        (LC.LightClientState[] memory _states, V.PlonkProof[] memory _proofs) =
            abi.decode(result, (LC.LightClientState[], V.PlonkProof[]));

        uint256 statesCount = _states.length - 1;
        // Update the state and thus the l1BlockUpdates array would be updated
        for (uint8 i = 1; i <= statesCount; i++) {
            LC.LightClientState memory state = _states[i];
            V.PlonkProof memory proof = _proofs[i];
            vm.prank(permissionedProver);
            vm.expectEmit(true, true, true, true);
            emit LC.NewState(state.viewNum, state.blockHeight, state.blockCommRoot);
            lc.newFinalizedState(state, proof);
        }

        assertEq(lc.getStateHistoryCount(), blockCommCount + statesCount);
    }

    function test_GetHotShotCommitmentValid() public {
        vm.prank(permissionedProver);
        vm.expectEmit(true, true, true, true);
        emit LC.NewState(newState.viewNum, newState.blockHeight, newState.blockCommRoot);
        lc.newFinalizedState(newState, newProof);

        // Test for a smaller hotShotBlockHeight
        BN254.ScalarField blockComm =
            lc.getHotShotCommitment(newState.blockHeight - 1).blockCommRoot;
        assertEqBN254(blockComm, newState.blockCommRoot);
    }

    function test_revertWhenGetHotShotCommitmentInvalidHigh() public {
        // Get the highest HotShot blockheight recorded
        uint256 numCommitments = lc.getStateHistoryCount();
        (,, LC.HotShotCommitment memory comm) = lc.stateHistoryCommitments(numCommitments - 1);
        uint64 blockHeight = comm.blockHeight;
        // Expect revert when attempting to retrieve a block height higher than the highest one
        // recorded
        vm.expectRevert(LC.InvalidHotShotBlockForCommitmentCheck.selector);
        lc.getHotShotCommitment(blockHeight + 1);
    }
}
