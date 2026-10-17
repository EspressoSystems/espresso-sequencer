use std::{
    collections::{BTreeSet, HashMap, HashSet},
    sync::Arc,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use anyhow::Context;
use futures::{StreamExt, stream::FuturesUnordered};
use hotshot_types::{
    data::{
        EpochNumber, Leaf2, VidDisperse2, VidDisperseShare, ViewNumber,
        vid_disperse::vid_total_weight,
    },
    epoch_membership::EpochMembershipCoordinator,
    traits::{node_implementation::NodeType, signature_key::SignatureKey},
    vid::avidm_gf2::AvidmGf2Param,
    vote::HasViewNumber,
};
use parking_lot::Mutex;
use rand::seq::SliceRandom;
use tokio::{
    sync::{OwnedSemaphorePermit, Semaphore, mpsc, watch},
    task,
    time::{sleep, sleep_until},
};
use tracing::{debug, info, warn};

use super::{
    HealConfig, HealError, HealMetrics, HealTarget, heal_targets, rederive_own_share, serve,
};
use crate::{
    message::share::{ShareFetchRequest, ShareFetchResponse, ShareRequestBody, ShareResponseBody},
    storage::NewProtocolStorage,
    vid::{VidReconstructErrorKind, VidShareAccumulator, expected_vid_param, reconstruct},
};

impl<T: NodeType, S> HealCtx<T, S> {
    /// The leaf's epoch if this node is in that epoch's committee, and so holds a share of it.
    fn recipient_epoch(&self, leaf: &Leaf2<T>, epoch_height: u64) -> Option<EpochNumber> {
        let Some(epoch) = leaf.epoch(epoch_height) else {
            warn!(view = %leaf.view_number(), "share heal skips a leaf without an epoch");
            self.metrics.skipped("no_epoch");
            return None;
        };
        let committee = self.membership.stake_table_for_epoch(Some(epoch)).ok()?;
        let member = committee
            .stake_table()
            .any(|entry| T::SignatureKey::public_key(&entry.stake_table_entry) == self.public_key);
        member.then_some(epoch)
    }
}

pub type ShareResponse<T> = (<T as NodeType>::SignatureKey, ShareFetchResponse<T>);

/// Everything the healer reads, writes and sends.
pub struct HealCtx<T: NodeType, S> {
    pub config: HealConfig,
    pub storage: S,
    pub membership: EpochMembershipCoordinator<T>,
    pub public_key: T::SignatureKey,
    pub private_key: <T::SignatureKey as SignatureKey>::PrivateKey,
    pub requests: mpsc::Sender<(T::SignatureKey, ShareFetchRequest)>,
    /// The node's current view, which tells how far behind the tip a height is.
    pub view: watch::Receiver<ViewNumber>,
    pub metrics: HealMetrics,
}

enum Outcome {
    Healed,
    /// Peers did not hold enough shares; the height stays a target.
    Unreachable,
    /// No attempt can succeed: the leader committed to a non-codeword, or this node cannot
    /// re-create its share from the payload. The height is not asked for again.
    Terminal,
}

/// Heal missing shares: scan, heal the targets newest first, `heights_inflight` at a time, and scan
/// again after a sleep while a pass leaves work. Ends when nothing is left, so a settled node
/// reads no wal.
pub async fn run<T: NodeType, S: NewProtocolStorage<T>>(
    ctx: HealCtx<T, S>,
    mut responses: mpsc::Receiver<ShareResponse<T>>,
) {
    assert!(ctx.config.heights_inflight > 0, "heights_inflight is 0");
    assert!(ctx.config.peer_inflight > 0, "peer_inflight is 0");
    let limits = PeerLimits::new(ctx.config.peer_inflight);
    let mut terminal = HashSet::new();
    loop {
        match heal_pass(&ctx, &limits, &mut responses, &mut terminal).await {
            Ok(false) => return,
            Ok(true) => {},
            Err(err) => warn!(%err, "share heal pass failed"),
        }
        sleep(ctx.config.scan_interval).await;
    }
}

/// Caps the requests outstanding to one peer, across all heights.
struct PeerLimits<K> {
    cap: usize,
    peers: Mutex<HashMap<K, Arc<Semaphore>>>,
}

impl<K: std::hash::Hash + Eq + Clone> PeerLimits<K> {
    fn new(cap: usize) -> Self {
        Self {
            cap,
            peers: Mutex::default(),
        }
    }

    /// A slot for one request to `peer`, released on drop; `None` while the peer is at its cap.
    fn try_acquire(&self, peer: &K) -> Option<OwnedSemaphorePermit> {
        let semaphore = self
            .peers
            .lock()
            .entry(peer.clone())
            .or_insert_with(|| Arc::new(Semaphore::new(self.cap)))
            .clone();
        semaphore.try_acquire_owned().ok()
    }
}

/// Whether another height may start: the next target's `cost` has to fit what is left of the
/// budget, unless nothing runs.
fn admits(config: &HealConfig, running: usize, used: usize, cost: usize) -> bool {
    running == 0
        || (running < config.heights_inflight && used.saturating_add(cost) <= config.inflight_bytes)
}

/// One scan and heal. Returns whether a later pass can still find work: heights that stayed
/// unreachable or failed, or are missing but too recent to heal yet.
async fn heal_pass<T: NodeType, S: NewProtocolStorage<T>>(
    ctx: &HealCtx<T, S>,
    limits: &PeerLimits<T::SignatureKey>,
    responses: &mut mpsc::Receiver<ShareResponse<T>>,
    terminal: &mut HashSet<ViewNumber>,
) -> anyhow::Result<bool> {
    let now = unix_now();
    // The held views first: a node that cannot answer them has no use for the wal scan.
    let held = ctx.storage.held_vid_views().await?;
    let leaves = ctx
        .storage
        .decided_leaves(ctx.config.oldest_ts(now))
        .await?;
    let epoch_height = ctx.membership.epoch_height().u64();
    let (view, member) = (*ctx.view.borrow(), |leaf: &Leaf2<T>| {
        ctx.recipient_epoch(leaf, epoch_height)
    });
    let mut targets = heal_targets(&leaves, &held, view, now, member, &ctx.config);
    let recent = HealConfig {
        tip_margin: 0,
        ..ctx.config.clone()
    };
    let deferred = heal_targets(&leaves, &held, view, now, member, &recent).len() - targets.len();
    targets.retain(|target| !terminal.contains(&target.view));
    ctx.metrics.targets.set(targets.len());
    if targets.is_empty() {
        return Ok(deferred > 0);
    }
    info!(
        targets = targets.len(),
        "share heal: healing missing shares"
    );

    let (mut unreachable, mut skipped, mut failed) = (0, 0, 0);
    let mut healed = BTreeSet::new();
    let mut queue = targets.iter().peekable();
    let mut running = FuturesUnordered::new();
    let mut routes = HashMap::new();
    let mut used = 0;
    loop {
        while let Some(target) =
            queue.next_if(|target| admits(&ctx.config, running.len(), used, target.cost()))
        {
            let (route, height_responses) = mpsc::unbounded_channel();
            routes.insert(target.view, route);
            used += target.cost();
            running.push(heal_tracked(ctx, limits, height_responses, target));
        }
        ctx.metrics.inflight.set(running.len());
        if running.is_empty() {
            break;
        }
        tokio::select! {
            Some((target, result)) = running.next() => {
                routes.remove(&target.view);
                used -= target.cost();
                match result {
                    Ok(Outcome::Healed) => {
                        healed.insert(target.view);
                    },
                    Ok(Outcome::Unreachable) => unreachable += 1,
                    Ok(Outcome::Terminal) => {
                        skipped += 1;
                        terminal.insert(target.view);
                    },
                    Err(err) => {
                        failed += 1;
                        warn!(view = %target.view, %err, "share heal failed for height");
                    },
                }
                ctx.metrics.targets.set(targets.len() - healed.len());
            },
            response = responses.recv() => {
                let (from, response) = response.context("share responses closed")?;
                if let Some(route) = routes.get(&response.view_number()) {
                    // A closed route is a height that just ended; its response is stale.
                    let _ = route.send((from, response));
                }
            },
        }
    }
    ctx.metrics.unreachable.set(unreachable);
    // Best effort: the copies are an archive nothing reads back, and the healed views are no
    // longer targets, so a failure is not retried.
    if let Err(err) = ctx.storage.restore_leaf_copies(&healed).await {
        warn!(%err, "share heal could not restore leaf copies");
    }
    let healed = healed.len();
    info!(
        healed,
        unreachable, skipped, failed, "share heal: pass done"
    );
    Ok(unreachable + failed + deferred > 0)
}

/// `heal_one`, returning its target with the result.
async fn heal_tracked<'a, T: NodeType, S: NewProtocolStorage<T>>(
    ctx: &HealCtx<T, S>,
    limits: &PeerLimits<T::SignatureKey>,
    mut responses: mpsc::UnboundedReceiver<ShareResponse<T>>,
    target: &'a HealTarget<T>,
) -> (&'a HealTarget<T>, anyhow::Result<Outcome>) {
    let result = heal_one(ctx, limits, &mut responses, target).await;
    (target, result)
}

async fn heal_one<T: NodeType, S: NewProtocolStorage<T>>(
    ctx: &HealCtx<T, S>,
    limits: &PeerLimits<T::SignatureKey>,
    responses: &mut mpsc::UnboundedReceiver<ShareResponse<T>>,
    target: &HealTarget<T>,
) -> anyhow::Result<Outcome> {
    let started = Instant::now();
    let committee = ctx
        .membership
        .stake_table_for_epoch(Some(target.epoch))
        .map_err(|err| anyhow::anyhow!("{err}"))?;
    let total_weight = vid_total_weight::<T, _>(committee.stake_table(), Some(target.epoch));
    let param = expected_vid_param(&ctx.membership, target.epoch).context("no vid param")?;
    let mut peers: Vec<T::SignatureKey> = committee
        .stake_table()
        .map(|entry| T::SignatureKey::public_key(&entry.stake_table_entry))
        .filter(|key| *key != ctx.public_key)
        .collect();
    peers.shuffle(&mut rand::thread_rng());

    let mut fetch = Fetch {
        ctx,
        limits,
        target,
        peers,
        next_peer: 0,
        wave: wave_size(&param, committee.stake_table().len()),
        goal: param.recovery_threshold * 3 / 2,
        acc: VidShareAccumulator::new(
            target.commit,
            target.metadata.clone(),
            target.epoch,
            Some(param),
        ),
        bytes: 0,
        busy: Vec::new(),
        busy_retries: HashMap::new(),
    };
    let payload = loop {
        fetch.fetch_shares(responses).await?;
        if !fetch.acc.has_enough_shares() {
            return Ok(Outcome::Unreachable);
        }
        match fetch.decode().await? {
            Ok(payload) => break payload,
            Err(Outcome::Terminal) => return Ok(Outcome::Terminal),
            Err(_) => continue,
        }
    };
    ctx.metrics.phase("fetch", started.elapsed());
    let fetched_bytes = fetch.bytes;

    let decoded = Instant::now();
    let proposal = {
        let membership = ctx.membership.clone();
        let (own, key) = (ctx.public_key.clone(), ctx.private_key.clone());
        let target = target.clone();
        task::spawn_blocking(move || -> anyhow::Result<Result<_, HealError>> {
            let params = VidDisperse2::<T>::disperse_params(
                &payload,
                &membership,
                Some(target.epoch),
                &target.metadata,
            )
            .map_err(|err| anyhow::anyhow!("{err}"))?;
            let share =
                match rederive_own_share(&params, &own, target.view, target.epoch, target.commit) {
                    Ok(share) if share.verify(total_weight) => share,
                    Ok(_) => return Ok(Err(HealError::Unverified)),
                    Err(err) => return Ok(Err(err)),
                };
            let proposal = VidDisperseShare::V2(share)
                .to_proposal(&key)
                .context("signing rederived share")?;
            Ok(Ok(proposal))
        })
        .await??
    };
    let proposal = match proposal {
        Ok(proposal) => proposal,
        Err(err) => {
            warn!(view = %target.view, %err, "share heal cannot re-create the share");
            return Ok(Outcome::Terminal);
        },
    };
    ctx.metrics.phase("reencode", decoded.elapsed());

    let appended = Instant::now();
    let stamp = u32::try_from(target.header_ts).context("header timestamp over u32")?;
    ctx.storage.append_vid_stamped(&proposal, stamp).await?;
    ctx.metrics.phase("append", appended.elapsed());
    ctx.metrics.done.add(1);
    debug!(view = %target.view, "share healed");

    if let Some(rate) = ctx.config.max_fetch_bytes_per_s {
        sleep(Duration::from_secs_f64(fetched_bytes as f64 / rate as f64)).await;
    }
    Ok(Outcome::Healed)
}

/// How often a height blocked on peer caps looks again; slots free up in other heights' tasks.
const THROTTLE_POLL: Duration = Duration::from_millis(25);

/// Times a peer that answers `Busy` is asked again for one height.
const MAX_BUSY_RETRIES: usize = 3;

/// Peers to ask at once, so their shares together reach the fetch goal.
fn wave_size(param: &AvidmGf2Param, nodes: usize) -> usize {
    (param.recovery_threshold * 3 / 2 * nodes)
        .div_ceil(param.total_weights)
        .max(1)
}

/// Peer shares gathered for one height.
struct Fetch<'a, T: NodeType, S> {
    ctx: &'a HealCtx<T, S>,
    limits: &'a PeerLimits<T::SignatureKey>,
    target: &'a HealTarget<T>,
    peers: Vec<T::SignatureKey>,
    next_peer: usize,
    wave: usize,
    /// Shard coverage at which asking stops: spare shares for the ones that turn out bad.
    goal: usize,
    acc: VidShareAccumulator<T>,
    bytes: usize,
    /// Peers that answered `Busy`, with when to ask them again.
    busy: Vec<(tokio::time::Instant, T::SignatureKey)>,
    busy_retries: HashMap<T::SignatureKey, usize>,
}

impl<T: NodeType, S: NewProtocolStorage<T>> Fetch<'_, T, S> {
    /// Ask peers, at most one request each, until the shares cover `goal` or no peer is left.
    async fn fetch_shares(
        &mut self,
        responses: &mut mpsc::UnboundedReceiver<ShareResponse<T>>,
    ) -> anyhow::Result<()> {
        let mut inflight: HashMap<T::SignatureKey, (Instant, OwnedSemaphorePermit)> =
            HashMap::new();
        loop {
            self.requeue_ready_busy();
            let mut throttled = false;
            while self.acc.coverage() < self.goal
                && self.next_peer < self.peers.len()
                && (inflight.len() + self.acc.shares.len() < self.wave || inflight.is_empty())
            {
                let Some((peer, permit)) = self.take_free_peer() else {
                    throttled = true;
                    break;
                };
                let request = ShareFetchRequest::new(
                    self.target.view,
                    ShareRequestBody {
                        commitment: self.target.commit,
                    },
                );
                self.ctx.requests.send((peer.clone(), request)).await?;
                inflight.insert(peer, (Instant::now(), permit));
            }
            let timeout = self.ctx.config.req_timeout;
            let oldest = inflight
                .values()
                .map(|(started, _)| *started)
                .min()
                .map(|started| tokio::time::Instant::from_std(started + timeout));
            let poll = throttled.then(|| tokio::time::Instant::now() + THROTTLE_POLL);
            let retry = (self.acc.coverage() < self.goal)
                .then(|| self.busy.iter().map(|(at, _)| *at).min())
                .flatten();
            let Some(deadline) = oldest.into_iter().chain(retry).chain(poll).min() else {
                return Ok(());
            };
            tokio::select! {
                response = responses.recv() => {
                    let (from, response) = response.context("share responses closed")?;
                    if response.view_number() == self.target.view && inflight.remove(&from).is_some() {
                        self.admit(from, response.into_body());
                    }
                },
                () = sleep_until(deadline) => {
                    inflight.retain(|_, (started, _)| started.elapsed() < timeout);
                },
            }
        }
    }

    /// The next peer, from the unasked ones, that is under its in-flight cap, with its slot.
    fn take_free_peer(&mut self) -> Option<(T::SignatureKey, OwnedSemaphorePermit)> {
        let (index, permit) = (self.next_peer..self.peers.len())
            .find_map(|i| Some((i, self.limits.try_acquire(&self.peers[i])?)))?;
        self.peers.swap(self.next_peer, index);
        self.next_peer += 1;
        Some((self.peers[self.next_peer - 1].clone(), permit))
    }

    /// Queue the peers whose backoff has passed to be asked again.
    fn requeue_ready_busy(&mut self) {
        let now = tokio::time::Instant::now();
        let (ready, waiting) = std::mem::take(&mut self.busy)
            .into_iter()
            .partition(|(at, _)| *at <= now);
        self.busy = waiting;
        self.peers.extend(ready.into_iter().map(|(_, peer)| peer));
    }

    fn note_busy(&mut self, from: T::SignatureKey) {
        let retries = self.busy_retries.entry(from.clone()).or_default();
        if *retries < MAX_BUSY_RETRIES {
            *retries += 1;
            let at = tokio::time::Instant::now() + self.ctx.config.busy_backoff;
            self.busy.push((at, from));
        }
    }

    fn admit(&mut self, from: T::SignatureKey, body: ShareResponseBody<T>) {
        let share = match body {
            ShareResponseBody::Share(share) => share,
            ShareResponseBody::Busy => return self.note_busy(from),
            _ => return,
        };
        let len = serve::serialized_len(&*share);
        self.bytes += len;
        self.ctx.metrics.fetch_bytes.add(len);
        let view = self.target.view;
        let admitted = share.recipient_key == from && {
            self.acc.accept(view, from.clone(), *share);
            self.acc.shares.contains_key(&from)
        };
        if !admitted {
            self.ctx.metrics.verify_fail.add(1);
        }
    }

    /// Decode the admitted shares. A failed attempt weeds out the shares that fail verification:
    /// `Err(Unreachable)` asks the caller to fetch more, `Err(Unrecoverable)` to give up.
    async fn decode(&mut self) -> anyhow::Result<Result<T::BlockPayload, Outcome>> {
        let decoding = Instant::now();
        let common = self.acc.common.clone().context("shares without a common")?;
        let shares: Vec<_> = self
            .acc
            .shares
            .iter()
            .map(|(key, share)| (key.clone(), share.clone()))
            .collect();
        let target = self.target;
        let (view, epoch, commit, metadata) = (
            target.view,
            target.epoch,
            target.commit,
            target.metadata.clone(),
        );
        let result = task::spawn_blocking(move || {
            reconstruct::<T>(view, epoch, commit, common, shares, metadata, None)
        })
        .await?;
        self.ctx.metrics.phase("decode", decoding.elapsed());
        match result {
            Ok(obtained) => Ok(Ok(obtained.payload)),
            Err(err) => {
                self.ctx.metrics.verify_fail.add(err.bad_share_keys.len());
                for key in &err.bad_share_keys {
                    self.acc.shares.remove(key);
                }
                Ok(Err(match err.kind {
                    VidReconstructErrorKind::Unrecoverable => Outcome::Terminal,
                    VidReconstructErrorKind::AwaitingShares => Outcome::Unreachable,
                }))
            },
        }
    }
}

fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before the unix epoch")
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn admits_respects_count_and_budget() {
        let config = HealConfig {
            heights_inflight: 3,
            inflight_bytes: 100,
            ..HealConfig::default()
        };
        assert!(
            admits(&config, 0, 0, 1000),
            "an empty healer admits any height"
        );
        assert!(admits(&config, 1, 40, 60));
        assert!(!admits(&config, 1, 40, 61));
        assert!(!admits(&config, 3, 0, 1));
    }
}
