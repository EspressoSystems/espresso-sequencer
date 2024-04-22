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
    uint32 public constant BLOCKS_PER_EPOCH_TEST = 3;
    LC.LightClientState public genesis;
    // this constant should be consistent with `hotshot_contract::light_client.rs`
    uint64 internal constant STAKE_TABLE_CAPACITY = 10;
    DeployLightClientTestScript public deployer = new DeployLightClientTestScript();
    address payable public lcTestProxy;
    address public admin = makeAddr("admin");
    address public permissionedProver = makeAddr("prover");

    function initLC(LC.LightClientState memory _genesis, uint32 _blocksPerEpoch) public {
        lc = new LCMock(_genesis, _blocksPerEpoch);
    }

    function deployAndInitProxy(LC.LightClientState memory state, uint32 numBlocksPerEpoch)
        public
        returns (address payable, address)
    {
        //deploy light client test with a proxy
        (lcTestProxy, admin, state) = deployer.deployContract(state, numBlocksPerEpoch, admin);

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
        string[] memory cmds = new string[](4);
        cmds[0] = "diff-test";
        cmds[1] = "mock-genesis";
        cmds[2] = vm.toString(BLOCKS_PER_EPOCH_TEST);
        cmds[3] = vm.toString(STAKE_TABLE_CAPACITY / 2);

        bytes memory result = vm.ffi(cmds);
        (LC.LightClientState memory state, bytes32 votingSTComm, bytes32 frozenSTComm) =
            abi.decode(result, (LC.LightClientState, bytes32, bytes32));
        genesis = state;

        (lcTestProxy, admin) = deployAndInitProxy(genesis, BLOCKS_PER_EPOCH_TEST);

        bytes32 expectedStakeTableComm = lc.computeStakeTableComm(state);
        assertEq(votingSTComm, expectedStakeTableComm);
        assertEq(frozenSTComm, expectedStakeTableComm);
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
        assert(lc.blocksPerEpoch() == BLOCKS_PER_EPOCH_TEST);
        assertEq(abi.encode(lc.getGenesisState()), abi.encode(genesis));
        assertEq(abi.encode(lc.getFinalizedState()), abi.encode(genesis));
        assert(lc.currentEpoch() == 0);

        bytes32 stakeTableComm = lc.computeStakeTableComm(genesis);
        assertEq(lc.votingStakeTableCommitment(), stakeTableComm);
        assertEq(lc.frozenStakeTableCommitment(), stakeTableComm);
        assertEq(lc.votingThreshold(), genesis.threshold);
        assertEq(lc.frozenThreshold(), genesis.threshold);
    }

    // @dev helper function to be able to initialize the contract and capture the revert error
    function initWithExpectRevert(LC.LightClientState memory _genesis, uint32 _blocksPerEpoch)
        private
    {
        vm.expectRevert(LC.InvalidArgs.selector);
        lc = new LCMock(_genesis, _blocksPerEpoch);
    }

    function test_RevertWhen_InvalidGenesis() external {
        LC.LightClientState memory badGenesis = genesis;

        // wrong viewNum would revert
        badGenesis.viewNum = 1;
        initWithExpectRevert(badGenesis, BLOCKS_PER_EPOCH_TEST);
        badGenesis.viewNum = genesis.viewNum; // revert to correct

        // wrong blockHeight would revert
        badGenesis.blockHeight = 1;
        initWithExpectRevert(badGenesis, BLOCKS_PER_EPOCH_TEST);
        badGenesis.blockHeight = genesis.blockHeight; // revert to correct

        // zero-valued stake table commitments would revert
        badGenesis.stakeTableBlsKeyComm = BN254.ScalarField.wrap(0);
        initWithExpectRevert(badGenesis, BLOCKS_PER_EPOCH_TEST);
        badGenesis.stakeTableBlsKeyComm = genesis.stakeTableBlsKeyComm; // revert to correct
        badGenesis.stakeTableSchnorrKeyComm = BN254.ScalarField.wrap(0);
        initWithExpectRevert(badGenesis, BLOCKS_PER_EPOCH_TEST);
        badGenesis.stakeTableSchnorrKeyComm = genesis.stakeTableSchnorrKeyComm; // revert to correct
        badGenesis.stakeTableAmountComm = BN254.ScalarField.wrap(0);

        initWithExpectRevert(badGenesis, BLOCKS_PER_EPOCH_TEST);
        badGenesis.stakeTableAmountComm = genesis.stakeTableAmountComm; // revert to correct

        // zero-valued threshold would revert
        badGenesis.threshold = 0;
        initWithExpectRevert(badGenesis, BLOCKS_PER_EPOCH_TEST);
        badGenesis.threshold = genesis.threshold; // revert to correct

        // zero-valued BLOCK_PER_EPOCH would revert
        initWithExpectRevert(genesis, 0);
    }
}

contract LightClient_permissionedProver_Test is LightClientCommonTest {
    LC.LightClientState internal newState;
    V.PlonkProof internal newProof;

    function setUp() public {
        init();

        string[] memory cmds = new string[](6);
        cmds[0] = "diff-test";
        cmds[1] = "mock-consecutive-finalized-states";
        cmds[2] = vm.toString(BLOCKS_PER_EPOCH_TEST);
        cmds[3] = vm.toString(STAKE_TABLE_CAPACITY / 2);
        cmds[4] = vm.toString(uint64(1));
        cmds[5] = vm.toString(uint64(1));

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

    function test_EnablePermissionedProverModeActivationWithValidAddress() public {
        //deploy a fresh light client proxy, so that we can set enabled mode to true
        (address newLcTestProxy,,) = deployer.deployContract(genesis, BLOCKS_PER_EPOCH_TEST, admin);

        //cast the proxy to be of type light client test
        LCMock newLc = LCMock(newLcTestProxy);

        vm.prank(admin);
        address newProver = makeAddr("completelyNewProver");
        vm.expectEmit(true, true, true, true);
        emit LC.PermissionedProverRequired(newProver);
        newLc.setPermissionedProver(newProver);
        assertEq(newLc.permissionedProver(), newProver);
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
        vm.assume(newProver != address(0));
        assert(lc.permissionedProverEnabled());
        assertEq(lc.permissionedProver(), permissionedProver);

        vm.expectEmit(true, true, true, true);
        emit LC.PermissionedProverRequired(newProver);
        vm.prank(admin);
        lc.setPermissionedProver(newProver);
        assertEq(newProver, lc.permissionedProver());
    }

    function test_OldProverNoLongerWorks() public {
        //deploy a fresh light client proxy, so that we can set enabled mode to true
        (address newLcTestProxy,,) = deployer.deployContract(genesis, BLOCKS_PER_EPOCH_TEST, admin);

        //cast the proxy to be of type light client test
        LCMock newLc = LCMock(newLcTestProxy);

        vm.startPrank(admin);
        address prover1 = makeAddr("prover1");
        vm.expectEmit(true, true, true, true);
        emit LC.PermissionedProverRequired(prover1);
        newLc.setPermissionedProver(prover1);
        assertEq(newLc.permissionedProver(), prover1);

        address prover2 = makeAddr("prover2");
        vm.expectEmit(true, true, true, true);
        emit LC.PermissionedProverRequired(prover2);
        newLc.setPermissionedProver(prover2);
        assertEq(newLc.permissionedProver(), prover2);
        vm.stopPrank();

        //confirm that the old prover doesn't work
        vm.prank(prover1);
        vm.expectRevert(LC.ProverNotPermissioned.selector);
        newLc.newFinalizedState(newState, newProof);

        //confirm that the new prover works
        vm.prank(prover2);
        vm.expectEmit(true, true, true, true);
        emit LC.NewState(newState.viewNum, newState.blockHeight, newState.blockCommRoot);
        newLc.newFinalizedState(newState, newProof);
    }

    function test_RevertWhen_sameProverModeSentInUpdate() public {
        assertEq(lc.permissionedProverEnabled(), true);
        address currentProver = lc.permissionedProver();
        vm.prank(admin);
        vm.expectRevert(LC.NoChangeRequired.selector);
        lc.setPermissionedProver(currentProver);
    }

    function test_RevertWhen_NoPermissionedProverSetWhenTryingToEnablePermissionedMode() external {
        //deploy a fresh light client proxy
        (address newLcTestProxy,,) = deployer.deployContract(genesis, BLOCKS_PER_EPOCH_TEST, admin);

        //cast the proxy to be of type light client test
        LCMock newLc = LCMock(newLcTestProxy);

        // you cannot enable permissioned mode if you don't specify the prover that gets permissions
        vm.prank(admin);
        vm.expectRevert(LC.InvalidAddress.selector);
        newLc.setPermissionedProver(address(0));
    }

    function test_RevertWhen_UpdatePermissionedProverToZeroAddress() external {
        vm.expectRevert(LC.InvalidAddress.selector);
        vm.prank(admin);
        lc.setPermissionedProver(address(0));
    }

    function test_RevertWhen_NonAdminTriesToupdatePermissionedProver() external {
        vm.expectRevert();
        vm.prank(makeAddr("not an admin"));
        lc.setPermissionedProver(makeAddr("new prover"));
    }

    function test_RevertWhen_ProverNotPermissioned() external {
        vm.expectRevert(LC.ProverNotPermissioned.selector);
        vm.prank(makeAddr("ProverWithNoPermissions"));
        lc.newFinalizedState(newState, newProof);
    }

    function test_RevertWhen_ProverNotPermissionedEvenIfAdmin() external {
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
        string[] memory cmds = new string[](6);
        cmds[0] = "diff-test";
        cmds[1] = "mock-consecutive-finalized-states";
        cmds[2] = vm.toString(BLOCKS_PER_EPOCH_TEST);
        cmds[3] = vm.toString(STAKE_TABLE_CAPACITY / 2);
        cmds[4] = vm.toString(uint64(3));
        cmds[5] = vm.toString(uint64(3));

        bytes memory result = vm.ffi(cmds);
        (LC.LightClientState[] memory states, V.PlonkProof[] memory proofs) =
            abi.decode(result, (LC.LightClientState[], V.PlonkProof[]));
        vm.expectEmit(true, true, true, true);
        emit LC.NewState(states[0].viewNum, states[0].blockHeight, states[0].blockCommRoot);
        vm.prank(permissionedProver);
        lc.newFinalizedState(states[0], proofs[0]);
    }

    /// @dev for benchmarking purposes only
    function testCorrectUpdateWithNewPermissionedProver() external {
        // Generating a few consecutive states and proofs
        string[] memory cmds = new string[](6);
        cmds[0] = "diff-test";
        cmds[1] = "mock-consecutive-finalized-states";
        cmds[2] = vm.toString(BLOCKS_PER_EPOCH_TEST);
        cmds[3] = vm.toString(STAKE_TABLE_CAPACITY / 2);
        cmds[4] = vm.toString(uint64(3));
        cmds[5] = vm.toString(uint64(3));

        bytes memory result = vm.ffi(cmds);
        (LC.LightClientState[] memory states, V.PlonkProof[] memory proofs) =
            abi.decode(result, (LC.LightClientState[], V.PlonkProof[]));

        //update permissioned prover
        address newProver = makeAddr("newProver");
        vm.expectEmit(true, true, true, true);
        emit LC.PermissionedProverRequired(newProver);
        vm.prank(admin);
        lc.setPermissionedProver(newProver);

        //send newFinalizedState with the new prover
        vm.expectEmit(true, true, true, true);
        emit LC.NewState(states[0].viewNum, states[0].blockHeight, states[0].blockCommRoot);
        vm.prank(newProver);
        lc.newFinalizedState(states[0], proofs[0]);
    }

    /// @dev Test happy path for (BLOCK_PER_EPOCH + 1) consecutive new finalized blocks
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
        string[] memory cmds = new string[](4);
        cmds[0] = "diff-test";
        cmds[1] = "mock-genesis";
        cmds[2] = vm.toString(BLOCKS_PER_EPOCH_TEST);
        cmds[3] = vm.toString(numInitValidators);

        bytes memory result = vm.ffi(cmds);
        (LC.LightClientState memory state,,) =
            abi.decode(result, (LC.LightClientState, bytes32, bytes32));
        genesis = state;
        (lcTestProxy, admin) = deployAndInitProxy(genesis, BLOCKS_PER_EPOCH_TEST);

        genesis = state;

        // Generating a few consecutive states and proofs
        cmds = new string[](6);
        cmds[0] = "diff-test";
        cmds[1] = "mock-consecutive-finalized-states";
        cmds[2] = vm.toString(BLOCKS_PER_EPOCH_TEST);
        cmds[3] = vm.toString(numInitValidators);
        cmds[4] = vm.toString(numRegistrations);
        cmds[5] = vm.toString(numExits);

        result = vm.ffi(cmds);
        (LC.LightClientState[] memory states, V.PlonkProof[] memory proofs) =
            abi.decode(result, (LC.LightClientState[], V.PlonkProof[]));
        assert(
            states.length == BLOCKS_PER_EPOCH_TEST + 1 && proofs.length == BLOCKS_PER_EPOCH_TEST + 1
        );

        for (uint256 i = 0; i < BLOCKS_PER_EPOCH_TEST + 1; i++) {
            vm.expectEmit(true, true, true, true);
            emit LC.NewState(states[i].viewNum, states[i].blockHeight, states[i].blockCommRoot);
            vm.prank(permissionedProver);
            lc.newFinalizedState(states[i], proofs[i]);

            // check if LightClient.sol states are updated correctly
            assertEq(abi.encode(lc.getFinalizedState()), abi.encode(states[i]));
            // check against hardcoded epoch advancement expectation
            if (i == BLOCKS_PER_EPOCH_TEST) {
                // first block of a new epoch (from epoch 2) should update the following
                assertEq(lc.currentEpoch(), 2);
                bytes32 genesisComm = lc.computeStakeTableComm(genesis);
                LC.LightClientState memory lastBlockInFirstEpoch = states[i - 1];
                bytes32 firstEpochComm = lc.computeStakeTableComm(lastBlockInFirstEpoch);
                assertEq(lc.votingStakeTableCommitment(), genesisComm);
                assertEq(lc.frozenStakeTableCommitment(), firstEpochComm);
                assertEq(lc.votingThreshold(), genesis.threshold);
                assertEq(lc.frozenThreshold(), lastBlockInFirstEpoch.threshold);
            } else {
                assertEq(lc.currentEpoch(), 1);
                bytes32 stakeTableComm = lc.computeStakeTableComm(genesis);
                assertEq(lc.votingStakeTableCommitment(), stakeTableComm);
                assertEq(lc.frozenStakeTableCommitment(), stakeTableComm);
                assertEq(lc.votingThreshold(), genesis.threshold);
                assertEq(lc.frozenThreshold(), genesis.threshold);
            }
        }
    }

    /// @dev Test happy path for updating after skipping a few blocks (but not an epoch)
    /// forge-config: default.fuzz.runs = 4
    /// forge-config: quick.fuzz.runs = 1
    /// forge-config: ci.fuzz.runs = 10
    function test_UpdateAfterSkippedBlocks(uint32 numBlockSkipped, uint32 numBlockPerEpoch)
        external
    {
        numBlockPerEpoch = uint32(bound(numBlockPerEpoch, 2, 10));
        numBlockSkipped = uint32(bound(numBlockSkipped, 1, numBlockPerEpoch - 1));

        // re-assign LightClient with the same genesis but different numBlockPerEpoch
        deployAndInitProxy(genesis, numBlockPerEpoch);

        string[] memory cmds = new string[](4);
        cmds[0] = "diff-test";
        cmds[1] = "mock-skip-blocks";
        cmds[2] = vm.toString(numBlockPerEpoch);
        cmds[3] = vm.toString(numBlockSkipped);

        bytes memory result = vm.ffi(cmds);
        (LC.LightClientState memory state, V.PlonkProof memory proof) =
            abi.decode(result, (LC.LightClientState, V.PlonkProof));

        vm.expectEmit(true, true, true, true);
        emit LC.NewState(state.viewNum, state.blockHeight, state.blockCommRoot);
        vm.prank(permissionedProver);
        lc.newFinalizedState(state, proof);

        assertEq(lc.currentEpoch(), 1);
        bytes32 stakeTableComm = lc.computeStakeTableComm(genesis);
        assertEq(lc.votingStakeTableCommitment(), stakeTableComm);
        assertEq(lc.frozenStakeTableCommitment(), stakeTableComm);
        assertEq(lc.votingThreshold(), genesis.threshold);
        assertEq(lc.frozenThreshold(), genesis.threshold);
    }

    /// @dev Test unhappy path when a valid but oudated finalized state is submitted
    function test_RevertWhen_OutdatedStateSubmitted() external {
        uint32 numBlockSkipped = 1;
        string[] memory cmds = new string[](5);
        cmds[0] = "diff-test";
        cmds[1] = "mock-skip-blocks";
        cmds[2] = vm.toString(BLOCKS_PER_EPOCH_TEST);
        cmds[3] = vm.toString(numBlockSkipped);
        cmds[4] = vm.toString(false);

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

    /// @dev Test unhappy path when the last block of current epoch is skipped before block of the
    /// next/future epoch is submitted.
    function test_RevertWhen_EpochEndingBlockSkipped() external {
        string[] memory cmds = new string[](3);
        cmds[0] = "diff-test";
        cmds[1] = "mock-miss-ending-block";
        cmds[2] = vm.toString(BLOCKS_PER_EPOCH_TEST);

        bytes memory result = vm.ffi(cmds);
        (LC.LightClientState[] memory states, V.PlonkProof[] memory proofs) =
            abi.decode(result, (LC.LightClientState[], V.PlonkProof[]));

        // first update with the first block in epoch 1, which should pass
        vm.startPrank(permissionedProver);
        vm.expectEmit(true, true, true, true);
        emit LC.NewState(states[0].viewNum, states[0].blockHeight, states[0].blockCommRoot);
        lc.newFinalizedState(states[0], proofs[0]);

        // then directly update with the first block in epoch 2, which should fail
        vm.expectRevert(
            abi.encodeWithSelector(
                LC.MissingLastBlockForCurrentEpoch.selector, BLOCKS_PER_EPOCH_TEST
            )
        );
        lc.newFinalizedState(states[1], proofs[1]);
        vm.stopPrank();
    }

    /// @dev Test unhappy path when user inputs contain malformed field elements
    function test_RevertWhen_MalformedFieldElements() external {
        uint32 numBlockSkipped = 1;
        string[] memory cmds = new string[](5);
        cmds[0] = "diff-test";
        cmds[1] = "mock-skip-blocks";
        cmds[2] = vm.toString(BLOCKS_PER_EPOCH_TEST);
        cmds[3] = vm.toString(numBlockSkipped);
        cmds[4] = vm.toString(false);

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

        // invalid scalar for feeLedgerComm
        badState.feeLedgerComm = BN254.ScalarField.wrap(BN254.R_MOD + 1);
        vm.expectRevert("Bn254: invalid scalar field");
        lc.newFinalizedState(badState, proof);
        badState.feeLedgerComm = newState.feeLedgerComm;

        // invalid scalar for stakeTableBlsKeyComm
        badState.stakeTableBlsKeyComm = BN254.ScalarField.wrap(BN254.R_MOD + 2);
        vm.expectRevert("Bn254: invalid scalar field");
        lc.newFinalizedState(badState, proof);
        badState.stakeTableBlsKeyComm = newState.stakeTableBlsKeyComm;

        // invalid scalar for stakeTableSchnorrKeyComm
        badState.stakeTableSchnorrKeyComm = BN254.ScalarField.wrap(BN254.R_MOD + 3);
        vm.expectRevert("Bn254: invalid scalar field");
        lc.newFinalizedState(badState, proof);
        badState.stakeTableSchnorrKeyComm = newState.stakeTableSchnorrKeyComm;

        // invalid scalar for stakeTableAmountComm
        badState.stakeTableAmountComm = BN254.ScalarField.wrap(BN254.R_MOD + 4);
        vm.expectRevert("Bn254: invalid scalar field");
        lc.newFinalizedState(badState, proof);
        badState.stakeTableAmountComm = newState.stakeTableAmountComm;
        vm.stopPrank();
    }

    /// @dev Test unhappy path when the plonk proof or the public inputs are wrong
    function test_RevertWhen_WrongProofOrWrongPublicInput() external {
        uint32 numBlockSkipped = 1;
        string[] memory cmds = new string[](5);
        cmds[0] = "diff-test";
        cmds[1] = "mock-skip-blocks";
        cmds[2] = vm.toString(BLOCKS_PER_EPOCH_TEST);
        cmds[3] = vm.toString(numBlockSkipped);
        cmds[4] = vm.toString(true);

        bytes memory result = vm.ffi(cmds);
        (LC.LightClientState memory newState, V.PlonkProof memory proof) =
            abi.decode(result, (LC.LightClientState, V.PlonkProof));

        BN254.ScalarField randScalar = BN254.ScalarField.wrap(1234);
        LC.LightClientState memory badState = newState;

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

        // wrong feeLedgerComm
        badState.feeLedgerComm = randScalar;
        vm.expectRevert(LC.InvalidProof.selector);
        lc.newFinalizedState(badState, proof);
        badState.feeLedgerComm = newState.feeLedgerComm;

        // wrong stakeTableBlsKeyComm
        badState.stakeTableBlsKeyComm = randScalar;
        vm.expectRevert(LC.InvalidProof.selector);
        lc.newFinalizedState(badState, proof);
        badState.stakeTableBlsKeyComm = newState.stakeTableBlsKeyComm;

        // wrong stakeTableSchnorrKeyComm
        badState.stakeTableSchnorrKeyComm = randScalar;
        vm.expectRevert(LC.InvalidProof.selector);
        lc.newFinalizedState(badState, proof);
        badState.stakeTableSchnorrKeyComm = newState.stakeTableSchnorrKeyComm;

        // wrong stakeTableAmountComm
        badState.stakeTableAmountComm = randScalar;
        vm.expectRevert(LC.InvalidProof.selector);
        lc.newFinalizedState(badState, proof);
        badState.stakeTableAmountComm = newState.stakeTableAmountComm;

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
        string[] memory cmds = new string[](3);
        cmds[0] = "diff-test";
        cmds[1] = "mock-wrong-stake-table";
        cmds[2] = vm.toString(BLOCKS_PER_EPOCH_TEST);

        bytes memory result = vm.ffi(cmds);
        (LC.LightClientState memory newState, V.PlonkProof memory proof) =
            abi.decode(result, (LC.LightClientState, V.PlonkProof));

        vm.expectRevert(LC.InvalidProof.selector);
        vm.prank(permissionedProver);
        lc.newFinalizedState(newState, proof);
    }
}
