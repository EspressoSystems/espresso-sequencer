//! Share heal against in-memory peers: a node with an empty share store and an intact chain
//! re-creates its shares from what the other nodes hold.

use std::{
    collections::{BTreeSet, HashMap},
    sync::Arc,
    time::Duration,
};

use hotshot_example_types::{node_types::TestTypes, storage_types::TestStorage};
use hotshot_types::{
    data::{VidCommitment, VidDisperseShare, VidDisperseShare2, ViewNumber},
    signature_key::BLSPubKey,
    traits::{signature_key::SignatureKey, storage::Storage as _},
    vid::avidm_gf2::AvidmGf2Scheme,
    vote::HasViewNumber,
};
use parking_lot::Mutex;
use tokio::{
    sync::{mpsc, watch},
    task::JoinHandle,
    time::{sleep, timeout},
};

use super::common::utils::{TestData, mock_membership_with_num_nodes};
use crate::{
    heal::{HealConfig, HealCtx, HealMetrics, run, serve::load_response},
    message::{
        fetch::Response,
        share::{ShareFetchRequest, ShareResponseBody},
    },
    storage::NewProtocolStorage,
};

const NODES: usize = 4;
const VIEWS: usize = 4;
const HEALER: usize = 0;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Peer {
    Honest,
    /// Answers every request with `NotHeld`.
    Empty,
    /// Answers with a share whose content does not belong to the commitment.
    Garbage,
}

struct Cluster {
    data: TestData,
    keys: Vec<(BLSPubKey, <BLSPubKey as SignatureKey>::PrivateKey)>,
    storages: Vec<TestStorage<TestTypes>>,
    peers: Arc<Mutex<Vec<Peer>>>,
    /// Views requested from peers, in order.
    requested: Arc<Mutex<Vec<ViewNumber>>>,
}

impl Cluster {
    /// Every node but the healer holds its share of every view; the healer's store knows the chain.
    async fn new() -> Self {
        let data = TestData::new_with_epoch_height_and_num_nodes(VIEWS, 10, NODES).await;
        let keys: Vec<_> = (0..NODES as u64)
            .map(|i| BLSPubKey::generated_from_seed_indexed([0u8; 32], i))
            .collect();
        let storages: Vec<_> = (0..NODES).map(|_| TestStorage::default()).collect();
        for (storage, key) in storages.iter().zip(&keys).skip(HEALER + 1) {
            for view in &data.views {
                let share = VidDisperseShare::V2(view.vid_share_for(&key.0));
                storage
                    .append_vid(&share.to_proposal(&key.1).unwrap())
                    .await
                    .unwrap();
            }
        }
        for view in &data.views {
            storages[HEALER]
                .insert_decided_leaf(view.leaf.clone())
                .await;
        }
        Self {
            data,
            keys,
            storages,
            peers: Arc::new(Mutex::new(vec![Peer::Honest; NODES])),
            requested: Arc::default(),
        }
    }

    fn config() -> HealConfig {
        HealConfig {
            w: Duration::from_secs(u32::MAX.into()),
            tip_margin: 0,
            min_remaining: Duration::ZERO,
            scan_interval: Duration::from_millis(100),
            req_timeout: Duration::from_millis(500),
            ..HealConfig::default()
        }
    }

    /// Start the healer over `storage`, with peers answered from the cluster's stores.
    fn start_healer(&self, storage: TestStorage<TestTypes>) -> JoinHandle<()> {
        let (public_key, private_key) = self.keys[HEALER].clone();
        let (membership, ..) = mock_membership_with_num_nodes(NODES, 10, public_key);
        let (requests, mut requests_rx) = mpsc::channel::<(BLSPubKey, ShareFetchRequest)>(64);
        let (responses_tx, responses) = mpsc::channel(64);
        let (_view_tx, view) = watch::channel(ViewNumber::new(1000));
        let ctx = HealCtx {
            config: Self::config(),
            storage,
            membership,
            public_key,
            private_key,
            requests,
            view,
            metrics: HealMetrics::new(&hotshot_types::traits::metrics::NoMetrics),
        };

        let (keys, storages) = (self.keys.clone(), self.storages.clone());
        let (peers, requested) = (self.peers.clone(), self.requested.clone());
        let data_shares: HashMap<_, _> = self
            .data
            .views
            .iter()
            .map(|view| (view.view_number, view.vid_shares.clone()))
            .collect();
        tokio::spawn(async move {
            let _view_tx = _view_tx;
            while let Some((peer, request)) = requests_rx.recv().await {
                let idx = keys.iter().position(|key| key.0 == peer).unwrap();
                let view = request.view_number();
                requested.lock().push(view);
                let behavior = peers.lock()[idx];
                let body = match behavior {
                    Peer::Honest => load_response(&storages[idx], &request, usize::MAX).await.0,
                    Peer::Empty => ShareResponseBody::NotHeld,
                    Peer::Garbage => {
                        let honest = data_shares[&view].iter().find(|s| s.recipient_key == peer);
                        ShareResponseBody::Share(Box::new(garbage(honest.unwrap().clone())))
                    },
                };
                responses_tx
                    .send((peer, Response::new(view, body)))
                    .await
                    .unwrap();
            }
        });
        tokio::spawn(run(ctx, responses))
    }

    async fn held(&self, storage: &TestStorage<TestTypes>) -> BTreeSet<ViewNumber> {
        storage.held_vid_views().await.unwrap()
    }

    fn all_views(&self) -> BTreeSet<ViewNumber> {
        self.data.views.iter().map(|v| v.view_number).collect()
    }

    /// Wait for the healer's store to hold exactly `views`.
    async fn wait_held(&self, storage: &TestStorage<TestTypes>, views: &BTreeSet<ViewNumber>) {
        let waited = timeout(Duration::from_secs(10), async {
            while self.held(storage).await != *views {
                sleep(Duration::from_millis(50)).await;
            }
        })
        .await;
        if waited.is_err() {
            panic!(
                "held views never became {views:?}: {:?}",
                self.held(storage).await
            );
        }
    }

    /// The share every view's live path stored for the healer.
    fn live_share(&self, view: ViewNumber) -> VidDisperseShare2<TestTypes> {
        let view = self
            .data
            .views
            .iter()
            .find(|v| v.view_number == view)
            .unwrap();
        view.vid_share_for(&self.keys[HEALER].0)
    }

    async fn assert_byte_equal(
        &self,
        storage: &TestStorage<TestTypes>,
        views: &BTreeSet<ViewNumber>,
    ) {
        for view in views {
            let healed = storage.load_vid_share(*view).await.unwrap().unwrap();
            let live = VidDisperseShare::V2(self.live_share(*view))
                .to_proposal(&self.keys[HEALER].1)
                .unwrap();
            assert_eq!(healed, live, "view {view}");
        }
    }
}

/// The share with its content swapped for another payload's at the same shard range, so it
/// passes intake but fails merkle verification.
fn garbage(mut share: VidDisperseShare2<TestTypes>) -> VidDisperseShare2<TestTypes> {
    let weights = [1u32; NODES];
    let other = vec![0xb7u8; 64];
    let (_, _, shares) = AvidmGf2Scheme::ns_disperse(
        &share.common.param,
        &weights,
        &other,
        std::iter::once(0..other.len()),
    )
    .unwrap();
    let range = share.share.range().unwrap();
    share.share = shares[range.start].clone();
    share
}

#[tokio::test(flavor = "multi_thread")]
async fn heal_in_memory_peers_ok() {
    let cluster = Cluster::new().await;
    let storage = cluster.storages[HEALER].clone();
    assert!(cluster.held(&storage).await.is_empty());

    let healer = cluster.start_healer(storage.clone());
    cluster.wait_held(&storage, &cluster.all_views()).await;
    cluster
        .assert_byte_equal(&storage, &cluster.all_views())
        .await;
    healer.abort();
}

#[tokio::test(flavor = "multi_thread")]
async fn heal_bad_share_weeded_ok() {
    let cluster = Cluster::new().await;
    cluster.peers.lock()[1] = Peer::Garbage;
    let storage = cluster.storages[HEALER].clone();

    let healer = cluster.start_healer(storage.clone());
    cluster.wait_held(&storage, &cluster.all_views()).await;
    cluster
        .assert_byte_equal(&storage, &cluster.all_views())
        .await;
    healer.abort();
}

#[tokio::test(flavor = "multi_thread")]
async fn heal_below_threshold_retries_ok() {
    let cluster = Cluster::new().await;
    {
        let mut peers = cluster.peers.lock();
        peers[1] = Peer::Empty;
        peers[2] = Peer::Empty;
        peers[3] = Peer::Empty;
    }
    let storage = cluster.storages[HEALER].clone();
    let healer = cluster.start_healer(storage.clone());

    // Several scans ask and find nothing.
    sleep(Duration::from_millis(600)).await;
    assert!(cluster.held(&storage).await.is_empty());
    assert!(cluster.requested.lock().len() > cluster.data.views.len());

    cluster.peers.lock().fill(Peer::Honest);
    cluster.wait_held(&storage, &cluster.all_views()).await;
    healer.abort();
}

#[tokio::test(flavor = "multi_thread")]
async fn heal_restart_resumes_ok() {
    let cluster = Cluster::new().await;
    let storage = cluster.storages[HEALER].clone();
    // A previous run healed the newest view before stopping.
    let done = cluster.data.views.last().unwrap().view_number;
    let share = VidDisperseShare::V2(cluster.live_share(done));
    storage
        .append_vid(&share.to_proposal(&cluster.keys[HEALER].1).unwrap())
        .await
        .unwrap();

    let healer = cluster.start_healer(storage.clone());
    cluster.wait_held(&storage, &cluster.all_views()).await;
    healer.abort();
    assert!(!cluster.requested.lock().contains(&done));
}

/// The leader of view 2 committed to a non-codeword: no share set decodes, so the height is
/// skipped while the others heal.
#[tokio::test(flavor = "multi_thread")]
async fn heal_unrecoverable_skipped_ok() {
    let mut cluster = Cluster::new().await;
    let poisoned = cluster.data.views[1].view_number;
    cluster.poison(poisoned).await;
    let storage = cluster.storages[HEALER].clone();

    let healer = cluster.start_healer(storage.clone());
    let mut expected = cluster.all_views();
    expected.remove(&poisoned);
    cluster.wait_held(&storage, &expected).await;
    sleep(Duration::from_millis(500)).await;
    assert_eq!(cluster.held(&storage).await, expected);
    healer.abort();
}

/// A height that cannot be healed is asked for once, not on every scan.
#[tokio::test(flavor = "multi_thread")]
async fn heal_terminal_failure_not_retried_ok() {
    let mut cluster = Cluster::new().await;
    let poisoned = cluster.data.views[1].view_number;
    cluster.poison(poisoned).await;
    let storage = cluster.storages[HEALER].clone();

    let healer = cluster.start_healer(storage.clone());
    let mut expected = cluster.all_views();
    expected.remove(&poisoned);
    cluster.wait_held(&storage, &expected).await;
    sleep(Duration::from_millis(600)).await;
    healer.abort();
    let asked = cluster
        .requested
        .lock()
        .iter()
        .filter(|view| **view == poisoned)
        .count();
    assert!(asked < NODES, "asked {asked} times");
}

/// A live append of a view the healer also heals leaves one identical share.
#[tokio::test(flavor = "multi_thread")]
async fn heal_duplicate_view_ok() {
    let cluster = Cluster::new().await;
    let storage = cluster.storages[HEALER].clone();
    let healer = cluster.start_healer(storage.clone());
    cluster.wait_held(&storage, &cluster.all_views()).await;

    let view = cluster.data.views[0].view_number;
    let live = VidDisperseShare::V2(cluster.live_share(view))
        .to_proposal(&cluster.keys[HEALER].1)
        .unwrap();
    storage.append_vid_stamped(&live, 1).await.unwrap();
    assert_eq!(cluster.held(&storage).await, cluster.all_views());
    cluster
        .assert_byte_equal(&storage, &cluster.all_views())
        .await;
    healer.abort();
}

impl Cluster {
    /// Replace `view`'s dispersal with one committing to a non-codeword, in the chain and in
    /// every peer's store.
    async fn poison(&mut self, view: ViewNumber) {
        let index = self
            .data
            .views
            .iter()
            .position(|v| v.view_number == view)
            .unwrap();
        let template = self.data.views[index].vid_shares[0].clone();
        let weights = [1u32; NODES];
        let payload = vec![0x5au8; 64];
        let (commit, common, shares) = AvidmGf2Scheme::ns_disperse_non_codeword(
            &template.common.param,
            &weights,
            &payload,
            std::iter::once(0..payload.len()),
        )
        .unwrap();

        self.data.views[index]
            .leaf
            .block_header_mut()
            .payload_commitment = VidCommitment::V2(commit);
        let leaf = self.data.views[index].leaf.clone();
        self.storages[HEALER].insert_decided_leaf(leaf).await;
        for (i, share) in shares.into_iter().enumerate() {
            let share = VidDisperseShare2::<TestTypes> {
                payload_commitment: commit,
                share,
                recipient_key: self.keys[i].0,
                common: common.clone(),
                ..template.clone()
            };
            self.data.views[index].vid_shares[i] = share.clone();
            if i != HEALER {
                let proposal = VidDisperseShare::V2(share)
                    .to_proposal(&self.keys[i].1)
                    .unwrap();
                self.storages[i].append_vid(&proposal).await.unwrap();
            }
        }
    }
}
