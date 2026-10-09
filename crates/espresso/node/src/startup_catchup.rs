//! Startup stake-table catchup for new-protocol nodes.
//!
//! Cliquenet only connects to validators in the current epoch's stake table
//! window (`N-1`, `N`, `N+1`). On a fresh-join or cold-restart node, no
//! consensus messages can be received until those stake tables are populated,
//! so the existing reactive catchup (triggered by an unknown-epoch proposal)
//! never fires.
//!
//! [`bootstrap_epoch_window`] populates the window in two stages:
//!
//! 1. Skip: the L1 light-client contract vouches for a finalized header `F`. The epoch roots below `F` are
//!    fetched with Merkle proofs against `F` and installed directly.
//! 2. Walk: from the highest known epoch, one epoch at a time, up to the epoch the peers' chain tip
//!    needs. A failure below that target is retried until the deadline. Without a target, or past
//!    it, the first failure marks the live tip.

use std::{ops::RangeInclusive, time::Duration};

use anyhow::{Context, anyhow, bail, ensure};
use clap::ValueEnum;
use espresso_types::{Header, SeqTypes};
use futures::future::join_all;
use hotshot_contract_adapter::{sol_types::LightClientStateSol, u256_to_field};
use hotshot_query_service::node::BlockId;
use hotshot_types::{
    data::{EpochNumber, ViewNumber},
    epoch_membership::EpochMembershipCoordinator,
    light_client::CircuitField,
    traits::{
        block_contents::BlockHeader,
        election::Membership,
        metrics::{Gauge, Metrics},
    },
    utils::{epoch_from_block_number, root_block_in_epoch},
};
use light_client::client::{Client, QueryServiceClient, is_not_found};
use serde::Serialize;
use tokio::time::{Instant, sleep, timeout};
use url::Url;

const TARGET: &str = "announce::bootstrap";

const INITIAL_BACKOFF: Duration = Duration::from_secs(1);
const MAX_BACKOFF: Duration = Duration::from_secs(60);

/// A once-per-boot milestone: INFO on the announce target, or debug when the call is not the boot
/// one.
macro_rules! milestone {
    ($params:expr, $($arg:tt)+) => {
        if $params.boot {
            tracing::info!(target: TARGET, $($arg)+)
        } else {
            tracing::debug!($($arg)+)
        }
    };
}

/// Whether to skip the walk using a trusted anchor.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, ValueEnum, Serialize)]
pub enum SkipMode {
    #[default]
    Auto,
    Off,
}

#[derive(Clone, Copy, Debug)]
pub struct BootstrapParams {
    /// Bound on a single `wait_for_stake_table` call during the walk.
    pub step_timeout: Duration,
    /// How long the skip, and separately the walk toward the peers' chain tip, may keep retrying
    /// before startup gives up.
    pub deadline: Duration,
    pub skip: SkipMode,
    /// Set for the call at boot only: milestones log at INFO on the announce target.
    pub boot: bool,
}

/// A finalized HotShot block that the node trusts without asking the peers: the L1 light-client
/// contract's finalized state, or a leaf the follower's light client has verified.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct TrustedAnchor {
    pub height: u64,
    pub block_comm_root: CircuitField,
}

impl TrustedAnchor {
    /// The anchor at a header the caller has verified.
    pub(crate) fn from_header(header: &Header) -> anyhow::Result<Self> {
        let state = header.get_light_client_state(ViewNumber::genesis())?;
        Ok(Self {
            height: header.height(),
            block_comm_root: state.block_comm_root,
        })
    }
}

impl From<LightClientStateSol> for TrustedAnchor {
    fn from(state: LightClientStateSol) -> Self {
        Self {
            height: state.blockHeight,
            block_comm_root: u256_to_field(state.blockCommRoot),
        }
    }
}

pub(crate) struct BootstrapMetrics {
    window_epoch: Box<dyn Gauge>,
    anchor_epoch: Box<dyn Gauge>,
    skipped: Box<dyn Gauge>,
}

impl BootstrapMetrics {
    pub fn new(metrics: &dyn Metrics) -> Self {
        Self {
            window_epoch: metrics.create_gauge("bootstrap_window_epoch".into(), None),
            anchor_epoch: metrics.create_gauge("bootstrap_l1_anchor_epoch".into(), None),
            skipped: metrics.create_gauge("bootstrap_skipped".into(), None),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum SkipOutcome {
    Installed,
    Walk(String),
}

/// Populate the membership with stake tables for every epoch up through `N+1` (where `N` is the
/// current epoch). Returns `N`.
///
/// `fetch_anchor` runs only when the skip is enabled and there are peers, so `SkipMode::Off` makes
/// no L1 calls.
///
/// Preconditions: `reload_stake` should have run before this: it populates the membership from
/// local persistence so the walk skips epochs we already know.
pub(crate) async fn bootstrap_epoch_window<C: Client>(
    coordinator: &EpochMembershipCoordinator<SeqTypes>,
    epoch_height: u64,
    fetch_anchor: impl AsyncFnOnce() -> anyhow::Result<TrustedAnchor>,
    peers: &[(Url, C)],
    params: &BootstrapParams,
    metrics: &BootstrapMetrics,
) -> anyhow::Result<EpochNumber> {
    if epoch_height == 0 {
        // Pre-epoch chain: epochs aren't enabled yet, the non-epoch
        // committee path is what gets used.
        return Ok(EpochNumber::genesis());
    }

    let started = Instant::now();
    let first_epoch = coordinator
        .membership()
        .first_epoch()
        .context("first_epoch not seeded; genesis stake table missing")?;

    let outcome = if params.skip == SkipMode::Off {
        SkipOutcome::Walk("off".into())
    } else if peers.is_empty() {
        SkipOutcome::Walk("no state peers".into())
    } else {
        match fetch_anchor().await {
            Ok(anchor) => {
                let skip = skip_to_anchor(
                    coordinator,
                    peers,
                    &anchor,
                    epoch_height,
                    first_epoch,
                    params,
                    metrics,
                );
                skip.await?
            },
            Err(err) => SkipOutcome::Walk(format!("{err:#}")),
        }
    };
    let skipped = match outcome {
        SkipOutcome::Installed => true,
        SkipOutcome::Walk(reason) => {
            milestone!(params, %reason, "skip disabled or not applicable; walking");
            false
        },
    };
    metrics.skipped.set(skipped as usize);

    let target = target_epoch(peers, epoch_height).await;
    let current = walk_to_tip(coordinator, first_epoch, target, params).await?;
    metrics.window_epoch.set(current.u64() as usize);
    milestone!(
        params,
        current_epoch = %current,
        duration_ms = started.elapsed().as_millis() as u64,
        skipped,
        "window installed",
    );
    Ok(current)
}

pub(crate) fn query_clients(urls: &[Url]) -> Vec<(Url, QueryServiceClient)> {
    urls.iter()
        .map(|url| (url.clone(), QueryServiceClient::new(url.clone())))
        .collect()
}

/// Install the epochs the `anchor` determines, unless the stake tables on hand already reach it.
async fn skip_to_anchor<C: Client>(
    coordinator: &EpochMembershipCoordinator<SeqTypes>,
    peers: &[(Url, C)],
    anchor: &TrustedAnchor,
    epoch_height: u64,
    first_epoch: EpochNumber,
    params: &BootstrapParams,
    metrics: &BootstrapMetrics,
) -> anyhow::Result<SkipOutcome> {
    let membership = coordinator.membership();
    let known = membership.highest_known_epoch();
    // Catchup resumes only from a consecutive pair, so a lone table does not count as near the tip.
    let resume = known.filter(|known| {
        membership
            .snapshot(EpochNumber::new(known.saturating_sub(1)))
            .is_some()
    });
    let anchor_epoch = epoch_from_block_number(anchor.height, epoch_height);
    metrics.anchor_epoch.set(anchor_epoch as usize);

    let range = match plan_skip(anchor, epoch_height, first_epoch, resume) {
        Ok(range) => range,
        Err(reason) => return Ok(SkipOutcome::Walk(reason.into())),
    };
    milestone!(
        params,
        finalized_height = anchor.height,
        epochs = ?(range.start().u64()..=range.end().u64()),
        highest_known = ?known.map(|e| e.u64()),
        "skipping to anchor",
    );

    let fetch = fetch_epoch_roots(peers, anchor, &range, epoch_height, params.deadline);
    let Some(headers) = fetch.await? else {
        return Ok(SkipOutcome::Walk(
            "peers do not serve light-client proofs".into(),
        ));
    };
    for header in headers {
        let height = header.height();
        coordinator
            .add_epoch_root(header)
            .await
            .map_err(|err| anyhow!("installing the epoch root at {height}: {err}"))?;
    }
    Ok(SkipOutcome::Installed)
}

/// The epochs to install, or the reason to walk instead. The skip runs only when the stake tables
/// on hand fall short of the install range. `resume` is the highest epoch that has its predecessor
/// too.
fn plan_skip(
    anchor: &TrustedAnchor,
    epoch_height: u64,
    first_epoch: EpochNumber,
    resume: Option<EpochNumber>,
) -> Result<RangeInclusive<EpochNumber>, &'static str> {
    let range = install_range(anchor.height, epoch_height, first_epoch)
        .ok_or("anchor too early for epochs")?;
    if resume.is_some_and(|resume| resume >= *range.start()) {
        return Err("near tip");
    }
    Ok(range)
}

/// The epochs whose stake tables the skip installs: `n-3 ..= n+1` for `n = epoch(F)`.
///
/// `block_reward(n)` and `block_reward(n+1)` need the stake table of three epochs earlier, and the
/// rest is the window. `None` when `n-3` is within the epochs seeded from genesis, where the walk
/// is short anyway.
fn install_range(
    anchor_height: u64,
    epoch_height: u64,
    first_epoch: EpochNumber,
) -> Option<RangeInclusive<EpochNumber>> {
    if epoch_height == 0 {
        return None;
    }
    let n = epoch_from_block_number(anchor_height, epoch_height);
    let start = n.checked_sub(3).filter(|start| *start > *first_epoch + 1)?;
    Some(EpochNumber::new(start)..=EpochNumber::new(n + 1))
}

/// Fetch and verify the epoch roots of `range` from the peers in rotation, backing off after each
/// full pass, until `limit` has passed.
///
/// `None` when every failure up to the deadline was NOT_FOUND: peers without the opt-in
/// light-client module cannot serve the skip, and the walk works without it.
async fn fetch_epoch_roots<C: Client>(
    peers: &[(Url, C)],
    anchor: &TrustedAnchor,
    range: &RangeInclusive<EpochNumber>,
    epoch_height: u64,
    limit: Duration,
) -> anyhow::Result<Option<Vec<Header>>> {
    let deadline = Instant::now() + limit;
    let mut backoff = INITIAL_BACKOFF;
    let mut attempt = 0usize;
    let mut all_not_found = true;
    loop {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() && all_not_found {
            tracing::warn!(
                attempt,
                "bootstrap: peers do not serve light-client proofs; walking"
            );
            return Ok(None);
        } else if remaining.is_zero() {
            tracing::error!(attempt, "bootstrap: deadline exceeded");
            bail!("bootstrap deadline exceeded while skipping to the anchor");
        }

        let (url, peer) = &peers[attempt % peers.len()];
        let fetch = fetch_roots_from(peer, anchor, range, epoch_height);
        let err = match timeout(remaining, fetch).await {
            Ok(Ok(headers)) => return Ok(Some(headers)),
            Ok(Err(err)) => err,
            Err(_) => anyhow!("attempt timed out"),
        };
        all_not_found &= is_not_found(&err);
        attempt += 1;
        if attempt.is_power_of_two() {
            tracing::warn!(
                attempt,
                %url,
                err = %format!("{err:#}"),
                "bootstrap: skip attempt failed; retrying"
            );
        }

        if attempt.is_multiple_of(peers.len()) {
            let remaining = deadline.saturating_duration_since(Instant::now());
            sleep(backoff.min(remaining)).await;
            backoff = (backoff * 2).min(MAX_BACKOFF);
        }
    }
}

/// Fetch the epoch roots of `range` from one `peer`, in ascending order.
///
/// Every header is verified against header `F` of the anchor, which is itself verified against
/// the anchor's commitment, so a lying peer fails here and nothing it cannot prove gets installed.
async fn fetch_roots_from<C: Client>(
    peer: &C,
    anchor: &TrustedAnchor,
    range: &RangeInclusive<EpochNumber>,
    epoch_height: u64,
) -> anyhow::Result<Vec<Header>> {
    let header = peer
        .header(anchor.height)
        .await
        .context("fetching the anchor header")?;
    verify_anchor_header(&header, anchor)?;
    let root = header.block_merkle_tree_root();

    let mut headers =
        Vec::with_capacity(range.end().u64().saturating_sub(range.start().u64()) as usize + 1);
    for epoch in range.start().u64()..=range.end().u64() {
        // The root of epoch `e - 2` derives the stake table of epoch `e`.
        let height = root_block_in_epoch(epoch - 2, epoch_height);
        let proof = peer
            .header_proof(anchor.height, BlockId::Number(height as usize))
            .await
            .with_context(|| format!("fetching the epoch root at height {height}"))?;
        let header = proof
            .verify(root)
            .with_context(|| format!("verifying the epoch root at height {height}"))?;
        ensure!(
            header.height() == height,
            "peer returned height {} for the epoch root at {height}",
            header.height()
        );
        headers.push(header);
    }
    Ok(headers)
}

/// Accept `header` only if it is the block the anchor commits to.
///
/// The view number does not enter `block_comm_root`, so any view serves.
fn verify_anchor_header(header: &Header, anchor: &TrustedAnchor) -> anyhow::Result<()> {
    ensure!(
        header.height() == anchor.height,
        "anchor header has height {}, expected {}",
        header.height(),
        anchor.height
    );
    let state = header.get_light_client_state(ViewNumber::genesis())?;
    ensure!(
        state.block_comm_root == anchor.block_comm_root,
        "anchor header at height {} does not match the block commitment root",
        anchor.height
    );
    Ok(())
}

/// The epoch whose stake table the window needs for the highest chain tip any peer reports:
/// `epoch(tip) + 1`, which the epoch root of `epoch(tip) - 1` derives. `None` when no peer answers.
async fn target_epoch<C: Client>(peers: &[(Url, C)], epoch_height: u64) -> Option<EpochNumber> {
    let heights = join_all(peers.iter().map(|(_, peer)| peer.block_height())).await;
    let tip = heights.into_iter().flatten().max()?.checked_sub(1)?;
    Some(EpochNumber::new(
        epoch_from_block_number(tip, epoch_height) + 1,
    ))
}

/// Walk forward from the highest known epoch. Each step drives `add_epoch_root` through the
/// existing catchup machinery, persisting the new stake table.
///
/// Steps below the `target` are retried until the deadline. From the target up, the first step
/// that peers cannot serve marks the live tip.
async fn walk_to_tip(
    coordinator: &EpochMembershipCoordinator<SeqTypes>,
    first_epoch: EpochNumber,
    target: Option<EpochNumber>,
    params: &BootstrapParams,
) -> anyhow::Result<EpochNumber> {
    let membership = coordinator.membership();
    // Catchup resumes from the latest consecutive pair of stake tables we
    // hold, so a gap near the tip is bridged by the step that runs into it
    // rather than having to be found here.
    let mut highest = membership.highest_known_epoch().unwrap_or(first_epoch + 1);

    milestone!(
        params,
        %first_epoch,
        starting_from = %highest,
        target = ?target.map(|e| e.u64()),
        "bootstrap: walking forward",
    );

    let deadline = Instant::now() + params.deadline;
    let mut backoff = INITIAL_BACKOFF;
    let mut failures = 0usize;
    loop {
        let next = highest + 1;
        let step = timeout(params.step_timeout, coordinator.wait_for_stake_table(next)).await;
        let timed_out = step.is_err();
        let err = match step {
            Ok(Ok(_)) => {
                tracing::info!(%next, "bootstrap: derived stake table");
                highest = next;
                continue;
            },
            Ok(Err(err)) => format!("{err:#}"),
            Err(_) => format!("timed out after {}s", params.step_timeout.as_secs()),
        };

        if target.is_none_or(|target| highest >= target) {
            if timed_out {
                tracing::warn!(%next, %err, "bootstrap: catchup timed out; proceeding with a possibly stale epoch window");
            } else {
                milestone!(params, %next, %err, "bootstrap: no further epoch to derive; at the live tip");
            }
            break;
        }
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            tracing::error!(%next, %err, "bootstrap: deadline exceeded");
            bail!("bootstrap deadline exceeded below the chain tip's epoch");
        }
        failures += 1;
        if failures.is_power_of_two() {
            tracing::warn!(%next, failures, %err, "bootstrap: catchup below the chain tip failed; retrying");
        }
        sleep(backoff.min(remaining)).await;
        backoff = (backoff * 2).min(MAX_BACKOFF);
    }

    // `highest` corresponds to N+1 (the leaf at root_block_in_epoch(N-1) is
    // the last finalized one peers can serve). So current epoch N = highest - 1.
    let current = EpochNumber::new(highest.saturating_sub(1));

    ensure!(
        membership.snapshot(current).is_some(),
        "missing stake table for current epoch {current} after bootstrap"
    );
    ensure!(
        membership.snapshot(highest).is_some(),
        "missing stake table for next epoch {highest} after bootstrap"
    );
    for epoch in [current, highest] {
        prepare_drb(coordinator, epoch, params.step_timeout).await;
    }
    Ok(current)
}

/// Make the DRB of `epoch` available before consensus starts. A failure only delays it: consensus
/// retries through reactive catchup.
async fn prepare_drb(
    coordinator: &EpochMembershipCoordinator<SeqTypes>,
    epoch: EpochNumber,
    step_timeout: Duration,
) {
    // A missing DRB starts a catchup that ends in `drb_ready`; its error is the start notice.
    if coordinator.membership_for_epoch(Some(epoch)).is_ok() {
        return;
    }
    match timeout(step_timeout, coordinator.wait_for_catchup(epoch)).await {
        Ok(Ok(_)) => tracing::debug!(%epoch, "bootstrap: DRB ready"),
        Ok(Err(err)) => {
            tracing::warn!(%epoch, %err, "bootstrap: DRB not ready; consensus will catch up")
        },
        Err(_) => tracing::warn!(%epoch, "bootstrap: DRB timed out; consensus will catch up"),
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, sync::Arc};

    use alloy::primitives::{Address, U256};
    use async_lock::Mutex as AsyncMutex;
    use async_trait::async_trait;
    use espresso_types::{
        AuthenticatedValidatorMap, ChainConfig, EpochCommittees, L1Client, StakeTableHash,
        mock::MockStateCatchup,
        traits::{EventsPersistenceRead, MembershipPersistence, StakeTuple},
        v0_3::{
            EventKey, Fetcher, IndexedStake, RegisteredValidator, RewardAmount, StakeTableEvent,
        },
    };
    use hotshot_example_types::storage_types::TestStorage;
    use hotshot_types::{
        PeerConfig, ValidatorConfig, drb::DrbResult, signature_key::BLSPubKey,
        traits::metrics::NoMetrics,
    };
    use indexmap::IndexMap;
    use light_client::testing::{TestClient, random_validator};
    use rstest::rstest;

    use super::*;

    const EPOCH_HEIGHT: u64 = 10;
    const FIRST_EPOCH: u64 = 1;
    /// Epoch of the anchor: the install range is `N-3 ..= N+1`.
    const N: u64 = 8;
    /// In epoch `N`, past its root block (75).
    const ANCHOR_HEIGHT: u64 = 77;

    const PARAMS: BootstrapParams = BootstrapParams {
        step_timeout: Duration::from_secs(5),
        deadline: Duration::from_secs(30),
        skip: SkipMode::Auto,
        boot: true,
    };

    fn epochs(range: RangeInclusive<EpochNumber>) -> RangeInclusive<u64> {
        range.start().u64()..=range.end().u64()
    }

    #[rstest]
    #[case::decaf_anchor_at_root(3000, 3000 * 1000 - 5, Some(997..=1001))]
    #[case::decaf_after_root(3000, 3000 * 1000 - 4, Some(997..=1001))]
    #[case::decaf_epoch_end(3000, 3000 * 1000, Some(997..=1001))]
    #[case::decaf_next_epoch(3000, 3000 * 1000 + 1, Some(998..=1002))]
    #[case::mainnet_anchor_at_root(40000, 40000 * 100 - 5, Some(97..=101))]
    #[case::mainnet_next_epoch(40000, 40000 * 100 + 1, Some(98..=102))]
    #[case::start_is_seeded(100, 100 * 4, None)]
    #[case::start_after_seeded(100, 100 * 5, Some(2..=6))]
    #[case::pre_pos(100, 50, None)]
    #[case::genesis(100, 0, None)]
    #[case::no_epochs(0, 1000, None)]
    fn install_range_cases(
        #[case] epoch_height: u64,
        #[case] anchor_height: u64,
        #[case] expected: Option<RangeInclusive<u64>>,
    ) {
        // `first_epoch = 0` seeds epochs 0 and 1.
        let range = install_range(anchor_height, epoch_height, EpochNumber::new(0)).map(epochs);
        assert_eq!(range, expected);
    }

    /// `block_reward(E)` needs the stake table of `E - 3`, and `E` ranges over `n` and `n + 1`.
    #[test]
    fn install_range_covers_reward_inputs() {
        let range = install_range(ANCHOR_HEIGHT, EPOCH_HEIGHT, EpochNumber::new(FIRST_EPOCH));
        let range = epochs(range.unwrap());
        assert!(range.contains(&(N - 3)));
        assert_eq!(*range.end(), N + 1);
    }

    #[test]
    fn install_range_roots_precede_the_anchor() {
        let range = install_range(ANCHOR_HEIGHT, EPOCH_HEIGHT, EpochNumber::new(FIRST_EPOCH));
        for epoch in epochs(range.unwrap()) {
            assert!(root_block_in_epoch(epoch - 2, EPOCH_HEIGHT) < ANCHOR_HEIGHT);
        }
    }

    #[rstest]
    #[case::empty(None, true)]
    #[case::below(Some(4), true)]
    #[case::at_start(Some(5), false)]
    #[case::inside(Some(7), false)]
    #[case::above(Some(12), false)]
    fn plan_skip_by_resume_epoch(#[case] resume: Option<u64>, #[case] runs: bool) {
        let anchor = TrustedAnchor {
            height: ANCHOR_HEIGHT,
            block_comm_root: CircuitField::from(0u64),
        };
        let plan = plan_skip(
            &anchor,
            EPOCH_HEIGHT,
            EpochNumber::new(FIRST_EPOCH),
            resume.map(EpochNumber::new),
        );
        let range = EpochNumber::new(N - 3)..=EpochNumber::new(N + 1);
        assert_eq!(plan, if runs { Ok(range) } else { Err("near tip") });
    }

    #[test]
    fn plan_skip_walks_on_early_anchor() {
        let anchor = TrustedAnchor {
            height: EPOCH_HEIGHT,
            block_comm_root: CircuitField::from(0u64),
        };
        let first = EpochNumber::new(FIRST_EPOCH);
        assert_eq!(
            plan_skip(&anchor, EPOCH_HEIGHT, first, None),
            Err("anchor too early for epochs")
        );
    }

    /// Test chain of `ANCHOR_HEIGHT + 1` blocks and the anchor over it.
    ///
    /// Peers report that height, so the walk targets epoch `N + 1`. `mock_block_height(0)` leaves
    /// the walk without a target.
    async fn chain() -> (TestClient, TrustedAnchor) {
        let client = TestClient::with_epoch_height(EPOCH_HEIGHT);
        let header = client.header(ANCHOR_HEIGHT).await.unwrap();
        let anchor = TrustedAnchor::from_header(&header).unwrap();
        (client, anchor)
    }

    #[tokio::test]
    async fn anchor_header_verification() {
        let (client, anchor) = chain().await;
        let header = client.header(ANCHOR_HEIGHT).await.unwrap();
        verify_anchor_header(&header, &anchor).unwrap();

        let wrong_root = TrustedAnchor {
            block_comm_root: anchor.block_comm_root + CircuitField::from(1u64),
            ..anchor
        };
        let err = verify_anchor_header(&header, &wrong_root).unwrap_err();
        assert!(format!("{err:#}").contains("commitment root"), "{err:#}");

        let wrong_height = TrustedAnchor {
            height: ANCHOR_HEIGHT + 1,
            ..anchor
        };
        let err = verify_anchor_header(&header, &wrong_height).unwrap_err();
        assert!(format!("{err:#}").contains("height"), "{err:#}");
    }

    /// Stands in for the node's database: it holds a stake table and DRB result for each epoch the
    /// test lets it answer for, with the hash a verified epoch root header commits to.
    #[derive(Clone, Debug, Default)]
    struct SeededStore(Arc<std::sync::Mutex<HashMap<u64, StakeTuple>>>);

    impl SeededStore {
        async fn for_chain(client: &TestClient, epochs: RangeInclusive<u64>) -> Self {
            let validators: AuthenticatedValidatorMap = (0..4)
                .map(|_| random_validator())
                .map(|validator| (validator.account, validator))
                .collect();
            let mut tables = HashMap::new();
            for epoch in epochs {
                // The root of epoch `e - 1` commits to the stake table of epoch `e`.
                let root = root_block_in_epoch(epoch - 1, EPOCH_HEIGHT);
                let hash = client.header(root).await.unwrap().next_stake_table_hash();
                tables.insert(
                    epoch,
                    (validators.clone(), Some(RewardAmount::default()), hash),
                );
            }
            Self(Arc::new(std::sync::Mutex::new(tables)))
        }
    }

    #[async_trait]
    impl MembershipPersistence for SeededStore {
        async fn load_stake(&self, epoch: EpochNumber) -> anyhow::Result<Option<StakeTuple>> {
            Ok(self.0.lock().unwrap().get(&epoch.u64()).cloned())
        }

        async fn load_latest_stake(
            &self,
            _limit: u64,
        ) -> anyhow::Result<Option<Vec<IndexedStake>>> {
            Ok(None)
        }

        async fn load_drb_result(&self, epoch: EpochNumber) -> anyhow::Result<Option<DrbResult>> {
            Ok(self
                .0
                .lock()
                .unwrap()
                .contains_key(&epoch.u64())
                .then_some([0; 32]))
        }

        async fn load_epoch_root(&self, _epoch: EpochNumber) -> anyhow::Result<Option<Header>> {
            Ok(None)
        }

        async fn store_epoch_root(&self, _: EpochNumber, _: Header) -> anyhow::Result<()> {
            Ok(())
        }

        async fn store_stake(
            &self,
            _: EpochNumber,
            _: AuthenticatedValidatorMap,
            _: Option<RewardAmount>,
            _: Option<StakeTableHash>,
        ) -> anyhow::Result<()> {
            Ok(())
        }

        async fn store_events(
            &self,
            _: u64,
            _: Vec<(EventKey, StakeTableEvent)>,
        ) -> anyhow::Result<()> {
            Ok(())
        }

        async fn load_events(
            &self,
            _: u64,
            _: u64,
        ) -> anyhow::Result<(
            Option<EventsPersistenceRead>,
            Vec<(EventKey, StakeTableEvent)>,
        )> {
            Ok((None, vec![]))
        }

        async fn delete_stake_tables(&self) -> anyhow::Result<()> {
            Ok(())
        }

        async fn store_all_validators(
            &self,
            _: EpochNumber,
            _: IndexMap<Address, RegisteredValidator<BLSPubKey>>,
        ) -> anyhow::Result<()> {
            Ok(())
        }

        async fn load_all_validators(
            &self,
            _: EpochNumber,
            _: u64,
            _: u64,
        ) -> anyhow::Result<Vec<RegisteredValidator<BLSPubKey>>> {
            Ok(vec![])
        }
    }

    /// A real `EpochCommittees` and coordinator over `store`, seeded like a fresh node. No peer
    /// serves epoch roots, so the walk ends at the first epoch above the installed ones.
    fn coordinator(store: SeededStore) -> EpochMembershipCoordinator<SeqTypes> {
        let fetcher = Fetcher::new(
            Arc::new(MockStateCatchup::default()),
            Arc::new(AsyncMutex::new(store)),
            L1Client::new(vec!["http://localhost:3331".parse().unwrap()]).unwrap(),
            ChainConfig::default(),
        );
        let peers: Vec<PeerConfig<SeqTypes>> = (0..4)
            .map(|i| {
                ValidatorConfig::<SeqTypes>::generated_from_seed_indexed(
                    [42u8; 32],
                    i,
                    U256::from(100),
                    true,
                )
                .public_config()
            })
            .collect();
        let committees =
            EpochCommittees::new_stake(peers.clone(), peers, None, fetcher, EPOCH_HEIGHT);
        committees.set_first_epoch(EpochNumber::new(FIRST_EPOCH), [0u8; 32]);
        EpochMembershipCoordinator::new(
            committees,
            EPOCH_HEIGHT,
            &TestStorage::<SeqTypes>::default(),
        )
    }

    async fn fresh_coordinator(client: &TestClient) -> EpochMembershipCoordinator<SeqTypes> {
        coordinator(SeededStore::for_chain(client, (N - 3)..=(N + 1)).await)
    }

    fn peer_list(clients: &[TestClient]) -> Vec<(Url, TestClient)> {
        let url: Url = "http://peer.invalid".parse().unwrap();
        clients.iter().map(|c| (url.clone(), c.clone())).collect()
    }

    async fn bootstrap(
        coordinator: &EpochMembershipCoordinator<SeqTypes>,
        anchor: Option<TrustedAnchor>,
        peers: &[TestClient],
        params: BootstrapParams,
    ) -> anyhow::Result<EpochNumber> {
        let metrics = BootstrapMetrics::new(&NoMetrics);
        bootstrap_epoch_window(
            coordinator,
            EPOCH_HEIGHT,
            async || anchor.context("no anchor"),
            &peer_list(peers),
            &params,
            &metrics,
        )
        .await
    }

    fn has_table(coordinator: &EpochMembershipCoordinator<SeqTypes>, epoch: u64) -> bool {
        coordinator
            .membership()
            .snapshot(EpochNumber::new(epoch))
            .is_some()
    }

    #[test_log::test(tokio::test)]
    async fn skip_installs_window() {
        let (client, anchor) = chain().await;
        let coordinator = fresh_coordinator(&client).await;

        let current = bootstrap(&coordinator, Some(anchor), &[client], PARAMS)
            .await
            .unwrap();

        // The residual walk finds no root above the anchor, as when the prover is not lagging.
        assert_eq!(current, EpochNumber::new(N));
        for epoch in (N - 3)..=(N + 1) {
            assert!(has_table(&coordinator, epoch), "epoch {epoch}");
        }
        assert_eq!(
            coordinator.membership().highest_known_epoch(),
            Some(EpochNumber::new(N + 1))
        );
    }

    #[test_log::test(tokio::test)]
    async fn peer_with_wrong_chain_is_skipped() {
        let (client, anchor) = chain().await;
        let (other, _) = chain().await;
        let coordinator = fresh_coordinator(&client).await;

        let current = bootstrap(&coordinator, Some(anchor), &[other, client], PARAMS)
            .await
            .unwrap();

        assert_eq!(current, EpochNumber::new(N));
        assert!(has_table(&coordinator, N + 1));
    }

    #[test_log::test(tokio::test)]
    async fn invalid_proof_at_first_epoch_installs_nothing() {
        let (client, anchor) = chain().await;
        let coordinator = fresh_coordinator(&client).await;
        let range = install_range(anchor.height, EPOCH_HEIGHT, EpochNumber::new(FIRST_EPOCH));
        let range = range.unwrap();
        let first_root = root_block_in_epoch(range.start().u64() - 2, EPOCH_HEIGHT);
        client.return_invalid_proof(first_root as usize).await;

        let err = fetch_roots_from(&client, &anchor, &range, EPOCH_HEIGHT)
            .await
            .unwrap_err();

        assert!(
            format!("{err:#}").contains("verifying the epoch root"),
            "{err:#}"
        );
        assert!(!has_table(&coordinator, N - 3));
    }

    /// A verification failure is retried, not read as the tip, and the retry installs the window.
    #[test_log::test(tokio::test)]
    async fn invalid_proof_is_retried() {
        let (client, anchor) = chain().await;
        let coordinator = fresh_coordinator(&client).await;
        let root = root_block_in_epoch(N - 1, EPOCH_HEIGHT) as usize;
        client.return_invalid_proof(root).await;
        let heal = client.clone();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            heal.remember_leaf(root).await;
        });

        let current = bootstrap(
            &coordinator,
            Some(anchor),
            &[client.clone(), client],
            PARAMS,
        )
        .await
        .unwrap();

        assert_eq!(current, EpochNumber::new(N));
        assert!(has_table(&coordinator, N + 1));
    }

    #[test_log::test(tokio::test)]
    async fn wrong_anchor_root_installs_nothing() {
        let (client, anchor) = chain().await;
        let coordinator = fresh_coordinator(&client).await;
        let range = install_range(anchor.height, EPOCH_HEIGHT, EpochNumber::new(FIRST_EPOCH));
        let anchor = TrustedAnchor {
            block_comm_root: anchor.block_comm_root + CircuitField::from(1u64),
            ..anchor
        };

        let err = fetch_roots_from(&client, &anchor, &range.unwrap(), EPOCH_HEIGHT)
            .await
            .unwrap_err();

        assert!(format!("{err:#}").contains("commitment root"), "{err:#}");
        assert!(!has_table(&coordinator, N - 3));
    }

    /// An epoch root the peer cannot serve at first must be retried, not read as the tip.
    #[test_log::test(tokio::test)]
    async fn transient_failure_is_retried() {
        let (client, anchor) = chain().await;
        let coordinator = fresh_coordinator(&client).await;
        let flaky_root = root_block_in_epoch(N - 1, EPOCH_HEIGHT) as usize;
        client.forget_leaf(flaky_root).await;
        let heal = client.clone();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            heal.remember_leaf(flaky_root).await;
        });

        let current = bootstrap(&coordinator, Some(anchor), &[client], PARAMS)
            .await
            .unwrap();

        assert_eq!(current, EpochNumber::new(N));
        assert!(has_table(&coordinator, N + 1));
    }

    #[test_log::test(tokio::test)]
    async fn deadline_aborts_when_far_from_the_anchor() {
        let (client, anchor) = chain().await;
        let coordinator = fresh_coordinator(&client).await;
        client.forget_leaf(ANCHOR_HEIGHT as usize).await;

        let params = BootstrapParams {
            deadline: Duration::from_millis(300),
            ..PARAMS
        };
        let err = bootstrap(&coordinator, Some(anchor), &[client], params)
            .await
            .unwrap_err();

        assert!(format!("{err:#}").contains("deadline exceeded"), "{err:#}");
    }

    /// Storage already inside the install range walks as before, without contacting any peer.
    #[test_log::test(tokio::test)]
    async fn restart_inside_range_walks() {
        let (client, anchor) = chain().await;
        let coordinator = coordinator(SeededStore::for_chain(&client, (N - 4)..=(N + 1)).await);
        for epoch in [N - 4, N - 3] {
            assert!(
                coordinator
                    .membership()
                    .load_stake_table(EpochNumber::new(epoch))
                    .await
            );
        }

        let before = client.header_requests().await;

        let current = bootstrap(
            &coordinator,
            Some(anchor),
            std::slice::from_ref(&client),
            PARAMS,
        )
        .await
        .unwrap();

        assert_eq!(current, EpochNumber::new(N));
        for epoch in (N - 4)..=(N + 1) {
            assert!(has_table(&coordinator, epoch), "epoch {epoch}");
        }
        assert_eq!(client.header_requests().await, before);
    }

    /// A failure below the epoch the peers' chain tip needs is retried, not read as the tip.
    #[test_log::test(tokio::test)]
    async fn walk_retries_failures_below_the_target() {
        let (client, _) = chain().await;
        let store = SeededStore::for_chain(&client, (N - 4)..=(N + 1)).await;
        let withheld = store.0.lock().unwrap().remove(&(N - 1)).unwrap();
        let coordinator = coordinator(store.clone());
        for epoch in [N - 4, N - 3] {
            coordinator
                .membership()
                .load_stake_table(EpochNumber::new(epoch))
                .await;
        }
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            store.0.lock().unwrap().insert(N - 1, withheld);
        });

        let current = bootstrap(&coordinator, None, &[client], PARAMS)
            .await
            .unwrap();

        assert_eq!(current, EpochNumber::new(N));
        assert!(has_table(&coordinator, N + 1));
    }

    #[test_log::test(tokio::test)]
    async fn walk_below_the_target_fails_at_the_deadline() {
        let (client, _) = chain().await;
        let store = SeededStore::for_chain(&client, (N - 4)..=(N + 1)).await;
        store.0.lock().unwrap().remove(&(N - 1));
        let coordinator = coordinator(store);
        for epoch in [N - 4, N - 3] {
            coordinator
                .membership()
                .load_stake_table(EpochNumber::new(epoch))
                .await;
        }
        let params = BootstrapParams {
            deadline: Duration::from_millis(300),
            ..PARAMS
        };

        let err = bootstrap(&coordinator, None, &[client], params)
            .await
            .unwrap_err();

        assert!(format!("{err:#}").contains("deadline exceeded"), "{err:#}");
    }

    /// A bad proof after the first good ones fails the whole fetch, so nothing is installed.
    #[test_log::test(tokio::test)]
    async fn invalid_proof_mid_range_installs_nothing() {
        let (client, anchor) = chain().await;
        let coordinator = fresh_coordinator(&client).await;
        let range = install_range(anchor.height, EPOCH_HEIGHT, EpochNumber::new(FIRST_EPOCH));
        let range = range.unwrap();
        client
            .return_invalid_proof(root_block_in_epoch(N - 3, EPOCH_HEIGHT) as usize)
            .await;

        let err = fetch_roots_from(&client, &anchor, &range, EPOCH_HEIGHT)
            .await
            .unwrap_err();

        assert!(
            format!("{err:#}").contains("verifying the epoch root"),
            "{err:#}"
        );
        for epoch in range.start().u64()..=range.end().u64() {
            assert!(!has_table(&coordinator, epoch), "epoch {epoch}");
        }
    }

    /// A restart after an interrupted install holds a lone table at the start of the range. That is
    /// not near the tip, because catchup needs a consecutive pair, so the skip runs again.
    #[test_log::test(tokio::test)]
    async fn partial_install_reruns_the_skip() {
        let (client, anchor) = chain().await;
        let coordinator = fresh_coordinator(&client).await;
        let first_root = client
            .header(root_block_in_epoch(N - 5, EPOCH_HEIGHT))
            .await;
        coordinator
            .add_epoch_root(first_root.unwrap())
            .await
            .unwrap();
        assert!(has_table(&coordinator, N - 3));
        assert!(!has_table(&coordinator, N - 4));
        let before = client.header_requests().await;

        let current = bootstrap(
            &coordinator,
            Some(anchor),
            std::slice::from_ref(&client),
            PARAMS,
        )
        .await
        .unwrap();

        assert_eq!(current, EpochNumber::new(N));
        for epoch in (N - 3)..=(N + 1) {
            assert!(has_table(&coordinator, epoch), "epoch {epoch}");
        }
        assert!(client.header_requests().await > before);
    }

    /// When the prover lags, the chain runs past the anchor's window and the walk continues there.
    #[test_log::test(tokio::test)]
    async fn walk_continues_above_the_window() {
        let (client, anchor) = chain().await;
        let coordinator = coordinator(SeededStore::for_chain(&client, (N - 3)..=(N + 2)).await);

        let current = bootstrap(&coordinator, Some(anchor), &[client], PARAMS)
            .await
            .unwrap();

        assert_eq!(current, EpochNumber::new(N + 1));
        assert!(has_table(&coordinator, N + 2));
    }

    /// `Off` walks without fetching the anchor, so it makes no L1 calls.
    #[test_log::test(tokio::test)]
    async fn skip_off_fetches_no_anchor() {
        let (client, _) = chain().await;
        client.mock_block_height(0).await;
        let coordinator = fresh_coordinator(&client).await;
        let params = BootstrapParams {
            skip: SkipMode::Off,
            ..PARAMS
        };
        let metrics = BootstrapMetrics::new(&NoMetrics);

        let current = bootstrap_epoch_window(
            &coordinator,
            EPOCH_HEIGHT,
            async || -> anyhow::Result<TrustedAnchor> {
                panic!("anchor fetched with the skip off")
            },
            &peer_list(&[client]),
            &params,
            &metrics,
        )
        .await
        .unwrap();

        assert_eq!(current, EpochNumber::new(FIRST_EPOCH));
        assert!(!has_table(&coordinator, N - 3));
    }

    #[test_log::test(tokio::test)]
    async fn no_anchor_walks() {
        let (client, _) = chain().await;
        client.mock_block_height(0).await;
        let coordinator = fresh_coordinator(&client).await;

        let current = bootstrap(&coordinator, None, &[client], PARAMS)
            .await
            .unwrap();

        assert_eq!(current, EpochNumber::new(FIRST_EPOCH));
        assert!(!has_table(&coordinator, N - 3));
    }

    /// Peers without the light-client module answer NOT_FOUND until the deadline; the node then
    /// walks instead of aborting.
    #[test_log::test(tokio::test)]
    async fn peers_without_light_client_walk() {
        let (client, anchor) = chain().await;
        client.mock_block_height(0).await;
        let coordinator = fresh_coordinator(&client).await;
        client.return_header_not_found().await;

        let params = BootstrapParams {
            deadline: Duration::from_millis(300),
            ..PARAMS
        };
        let current = bootstrap(
            &coordinator,
            Some(anchor),
            &[client.clone(), client],
            params,
        )
        .await
        .unwrap();

        assert_eq!(current, EpochNumber::new(FIRST_EPOCH));
        assert!(!has_table(&coordinator, N - 3));
    }

    /// One peer failing for another reason than NOT_FOUND keeps the deadline fatal.
    #[test_log::test(tokio::test)]
    async fn mixed_failures_abort_at_the_deadline() {
        let (client, anchor) = chain().await;
        let coordinator = fresh_coordinator(&client).await;
        let not_found = client.clone();
        not_found.return_header_not_found().await;
        let broken = TestClient::with_epoch_height(EPOCH_HEIGHT);
        broken.forget_leaf(ANCHOR_HEIGHT as usize).await;

        let params = BootstrapParams {
            deadline: Duration::from_millis(300),
            ..PARAMS
        };
        let err = bootstrap(&coordinator, Some(anchor), &[not_found, broken], params)
            .await
            .unwrap_err();

        assert!(format!("{err:#}").contains("deadline exceeded"), "{err:#}");
    }

    #[test_log::test(tokio::test)]
    async fn pre_pos_anchor_walks() {
        let (client, _) = chain().await;
        client.mock_block_height(0).await;
        let coordinator = fresh_coordinator(&client).await;
        let early = client.header(EPOCH_HEIGHT).await.unwrap();
        let anchor = TrustedAnchor {
            height: early.height(),
            block_comm_root: early
                .get_light_client_state(ViewNumber::genesis())
                .unwrap()
                .block_comm_root,
        };

        let current = bootstrap(&coordinator, Some(anchor), &[client], PARAMS)
            .await
            .unwrap();

        assert_eq!(current, EpochNumber::new(FIRST_EPOCH));
        assert!(!has_table(&coordinator, N - 3));
    }
}
