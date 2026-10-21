# Review participants

Who can interact with the network, how, and what each can influence. Use it to name who can trigger a bug and to set
its severity. Current versions: `data/genesis/{mainnet,decaf}.toml`. Contract owners and pausers:
`contracts/rust/deployment-info/deployments/{mainnet,decaf}.toml`.

## Consensus

- **Active validator** (staked). In the epoch's active set, chosen by `select_active_validator_set`
  (`crates/espresso/types/src/v0/impls/stake_table.rs`). Changes apply two epochs later
  (`doc/stake-table-fast-finality.md`).
  - Reaches peers over cliquenet (`crates/cliquenet/`), keys and addresses from StakeTable V3 or the latest version of StakeTable.
  - Votes, forms certificates, holds VID shares over the full stake table, signs light-client state, votes on upgrades.
  - As leader, builds the block in-process (`BlockBuilder::request_block`, `crates/hotshot/new-protocol/src/block.rs`)
    and picks header fields (timestamp, `l1_finalized`, namespace table). Every validator re-checks them in
    `validate_and_apply_header` (`crates/espresso/types/src/v0/impls/state.rs`).
  - No non-test code in `crates/hotshot/new-protocol/` reads the DA committee; VID uses the full stake table
    (`crates/hotshot/new-protocol/src/vid.rs`).
- **Registered, non-active validator** (permissionless, pausable). Calls StakeTable register, update-key,
  update-network-config, commission, metadata, deregister and exit functions (`contracts/src/StakeTableV3.sol`,
  `StakeTableV2.sol`). Its events change the candidate set, keys and peer addresses every node replays. Not a cliquenet
  party until active.
- **Delegator** (permissionless, pausable). `delegate`, `undelegate`, `claimWithdrawal` on StakeTable. Moves stake
  weight, so affects active-set membership, leader selection and rewards.
- **Builder** (legacy only). From `NEW_PROTOCOL_VERSION` leaders build blocks themselves and sign the builder fee with a
  fixed public key (`crates/hotshot/new-protocol/src/block.rs`), so the builder signature is not a security boundary.
  `NO_BUILDER_COMMITMENT_VERSION` drops the commitment. External builders only matter when replaying older blocks.

## Network and APIs

- **Unstaked network peer** (permissionless). Can reach the TCP accept, Noise handshake and Hello framing of cliquenet.
  Keys not in the stake table get `unknown party` (`crates/cliquenet/src/net/server.rs`).
- **Request-response catchup peer** (needs a stake-table key). Sends signed, TTL-checked, rate-limited requests
  (`crates/request-response/src/lib.rs`), answered from local storage
  (`crates/espresso/node/src/request_response/data_source.rs`). The requester verifies responses.
- **HTTP API client** (permissionless, no authentication). All v1 and v2 routes in
  `crates/espresso/api/src/axum/routes.rs`: availability, node, status, config, merklized state, catchup, submit,
  state-signature, light client, explorer, token. Catchup data a node fetches from `--state-peers` is verified
  (`crates/espresso/node/src/catchup.rs`). Network config fetched from `--config-peers` on first start is not
  (`crates/espresso/node/src/lib.rs`).
- **Transaction submitter / rollup** (permissionless). `POST /v1/submit/submit` or `/v2/submit/transaction` to
  `BlockBuilder::on_submit_transaction`. Chooses any namespace ID; namespaces have no owner. Reads back through
  availability and light-client endpoints or `LightClient.getHotShotCommitment`.
- **Query node / light-client follower** (permissionless). Serves the HTTP API without voting.
  `doc/light-client-follower.md`: trusts its upstream `--peers` for liveness only, verifies through the light client,
  reads the stake table from its own L1 RPC.
- **Light client** (`crates/light-client/`, permissionless). Fetches from untrusted query nodes and verifies against its
  `Genesis` root of trust.

## Light-client and prover path

- **State relay server** (`crates/espresso/node/src/state_signature/relay_server.rs`). Unauthenticated HTTP endpoint;
  only counts signatures from stake-table keys. Bundles signatures once the threshold is reached.
- **State prover** (`crates/hotshot-state-prover/`). Proves a quorum of state signatures and calls
  `LightClientV3.newFinalizedState`. Permissioned when `permissionedProver` is set (`contracts/src/LightClient.sol`);
  check the deployed contract for the current value. The `authRoot` it posts gates `RewardClaim`.

## L1

- **L1 RPC provider** (trusted by the operator who configures `--l1-provider-url`). Source of finalized blocks and
  StakeTable events (`crates/espresso/types/src/v0/impls/l1.rs`). Influences `l1_finalized` and the derived stake table.
- **FeeContract depositor** (permissionless). `deposit`. Only read by nodes before `NEW_PROTOCOL_VERSION`.
- **Reward claimer** (permissionless, pausable). `RewardClaim.claimRewards` with a Merkle proof against
  `LightClient.authRoot`, under a daily limit. Only `RewardClaim` can mint ESP (`contracts/src/EspTokenV2.sol`).

## Privileged

- **Contract owners and admins** (permissioned). OpsTimelock and SafeExitTimelock (OZ `TimelockController`), proposed by
  multisigs; holders and delays in `deployments/{mainnet,decaf}.toml`. They control contract upgrades, the permissioned
  prover, StakeTable escrow, commission and minimum-delegation parameters, `RewardClaim` daily limit, and
  `LightClientV2.updateEpochStartBlock`. Upgrade procedure: `doc/smart-contract-upgrades.md`.
- **Pausers** (permissioned). `PAUSER_ROLE` on StakeTable and RewardClaim stops staking and claim calls.
- **Node operator** (one per node). Holds the BLS, Schnorr and x25519 keys, the genesis TOML and CLI options
  (`crates/espresso/node/src/options.rs`): L1 RPC, state and config peers, orchestrator URL, telemetry endpoint.
- **Orchestrator** (whoever runs it). Supplies the full network config to nodes that start with no persisted config and
  no `--config-peers`. Bootstrap and test use.
- **Telemetry proxy** (outside this repo). Receives logs and metrics from nodes that opt in
  (`crates/espresso/telemetry/`); no consensus input.

