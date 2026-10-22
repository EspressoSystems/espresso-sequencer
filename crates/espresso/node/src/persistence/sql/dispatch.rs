//! One persistence over both SQL backends, so consensus and everything above it compile once.
//!
//! Every method of every forwarded trait is listed, including those with default bodies: a default
//! would bypass an override on the backend.

use std::{collections::BTreeMap, sync::Arc};

use async_trait::async_trait;
use espresso_types::{
    AuthenticatedValidatorMap, BackoffParams, Header, Leaf2, NetworkConfig, NodeState, PubKey,
    StakeTableHash,
    traits::{EventsPersistenceRead, MembershipPersistence, StakeTuple},
    v0::traits::{EventConsumer, PersistenceOptions, SequencerPersistence, StateCatchup},
    v0_3::{EventKey, IndexedStake, RegisteredValidator, RewardAmount, StakeTableEvent},
};
use hotshot_libp2p_networking::network::behaviours::dht::store::persistent::{
    DhtPersistentStorage, SerializableRecord,
};
use hotshot_new_protocol::message::Certificate2;
use hotshot_types::{
    data::{
        DaProposal, DaProposal2, EpochNumber, QuorumProposalWrapper, VidCommitment,
        VidDisperseShare, ViewNumber,
    },
    drb::{DrbInput, DrbResult},
    event::{HotShotAction, LeafInfo},
    initializer::{HotShotInitializer, InitializerEpochInfo},
    message::Proposal,
    new_protocol::CoordinatorEvent,
    simple_certificate::{
        CertificatePair, LightClientStateUpdateCertificateV2, NextEpochQuorumCertificate2,
        QuorumCertificate2, UpgradeCertificate,
    },
    traits::metrics::Metrics,
};
use indexmap::IndexMap;
use versions::Upgrade;

use super::{Options, postgres, sqlite};
use crate::{
    SeqTypes,
    sql_dispatch::{Sql, dispatch},
};

/// Persistence on whichever backend [`Options::embedded_db`] selects.
pub type Persistence = Sql<postgres::Persistence, sqlite::Persistence>;

#[async_trait]
impl PersistenceOptions for Options {
    type Persistence = Persistence;

    fn set_view_retention(&mut self, view_retention: u64) {
        Options::set_view_retention(self, view_retention);
    }

    fn set_consensus_only(&mut self) {
        Options::set_consensus_only(self);
    }

    async fn create(&mut self) -> anyhow::Result<Self::Persistence> {
        if self.embedded_db {
            let mut opt = sqlite::Options(self.clone());
            let persistence = opt.create().await?;
            self.pool = opt.0.pool;
            Ok(Sql::Sqlite(persistence))
        } else {
            let mut opt = postgres::Options(self.clone());
            let persistence = opt.create().await?;
            self.pool = opt.0.pool;
            Ok(Sql::Postgres(persistence))
        }
    }

    async fn reset(self) -> anyhow::Result<()> {
        if self.embedded_db {
            sqlite::Options(self).reset().await
        } else {
            postgres::Options(self).reset().await
        }
    }
}

#[async_trait]
impl DhtPersistentStorage for Persistence {
    async fn save(&self, records: Vec<SerializableRecord>) -> anyhow::Result<()> {
        dispatch!(self, x => x.save(records).await)
    }

    async fn load(&self) -> anyhow::Result<Vec<SerializableRecord>> {
        dispatch!(self, x => x.load().await)
    }
}

#[async_trait]
impl MembershipPersistence for Persistence {
    async fn load_stake(&self, epoch: EpochNumber) -> anyhow::Result<Option<StakeTuple>> {
        dispatch!(self, x => x.load_stake(epoch).await)
    }

    async fn load_latest_stake(&self, limit: u64) -> anyhow::Result<Option<Vec<IndexedStake>>> {
        dispatch!(self, x => x.load_latest_stake(limit).await)
    }

    async fn load_drb_result(&self, epoch: EpochNumber) -> anyhow::Result<Option<DrbResult>> {
        dispatch!(self, x => x.load_drb_result(epoch).await)
    }

    async fn load_epoch_root(&self, epoch: EpochNumber) -> anyhow::Result<Option<Header>> {
        dispatch!(self, x => x.load_epoch_root(epoch).await)
    }

    async fn store_epoch_root(
        &self,
        epoch: EpochNumber,
        block_header: Header,
    ) -> anyhow::Result<()> {
        dispatch!(self, x => x.store_epoch_root(epoch, block_header).await)
    }

    async fn store_stake(
        &self,
        epoch: EpochNumber,
        stake: AuthenticatedValidatorMap,
        block_reward: Option<RewardAmount>,
        stake_table_hash: Option<StakeTableHash>,
    ) -> anyhow::Result<()> {
        dispatch!(self, x => x.store_stake(epoch, stake, block_reward, stake_table_hash).await)
    }

    async fn store_events(
        &self,
        l1_finalized: u64,
        events: Vec<(EventKey, StakeTableEvent)>,
    ) -> anyhow::Result<()> {
        dispatch!(self, x => x.store_events(l1_finalized, events).await)
    }

    async fn load_events(
        &self,
        from_l1_block: u64,
        l1_finalized: u64,
    ) -> anyhow::Result<(
        Option<EventsPersistenceRead>,
        Vec<(EventKey, StakeTableEvent)>,
    )> {
        dispatch!(self, x => x.load_events(from_l1_block, l1_finalized).await)
    }

    async fn delete_stake_tables(&self) -> anyhow::Result<()> {
        dispatch!(self, x => x.delete_stake_tables().await)
    }

    async fn store_all_validators(
        &self,
        epoch: EpochNumber,
        all_validators: IndexMap<alloy::primitives::Address, RegisteredValidator<PubKey>>,
    ) -> anyhow::Result<()> {
        dispatch!(self, x => x.store_all_validators(epoch, all_validators).await)
    }

    async fn load_all_validators(
        &self,
        epoch: EpochNumber,
        offset: u64,
        limit: u64,
    ) -> anyhow::Result<Vec<RegisteredValidator<PubKey>>> {
        dispatch!(self, x => x.load_all_validators(epoch, offset, limit).await)
    }
}

#[async_trait]
impl SequencerPersistence for Persistence {
    fn into_catchup_provider(
        self,
        backoff: BackoffParams,
    ) -> anyhow::Result<Arc<dyn StateCatchup>> {
        dispatch!(self, x => x.into_catchup_provider(backoff))
    }

    async fn load_config(&self) -> anyhow::Result<Option<NetworkConfig>> {
        dispatch!(self, x => x.load_config().await)
    }

    async fn save_config(&self, cfg: &NetworkConfig) -> anyhow::Result<()> {
        dispatch!(self, x => x.save_config(cfg).await)
    }

    async fn load_latest_acted_view(&self) -> anyhow::Result<Option<ViewNumber>> {
        dispatch!(self, x => x.load_latest_acted_view().await)
    }

    async fn load_restart_view(&self) -> anyhow::Result<Option<ViewNumber>> {
        dispatch!(self, x => x.load_restart_view().await)
    }

    async fn load_quorum_proposals(
        &self,
    ) -> anyhow::Result<BTreeMap<ViewNumber, Proposal<SeqTypes, QuorumProposalWrapper<SeqTypes>>>>
    {
        dispatch!(self, x => x.load_quorum_proposals().await)
    }

    async fn load_quorum_proposal(
        &self,
        view: ViewNumber,
    ) -> anyhow::Result<Proposal<SeqTypes, QuorumProposalWrapper<SeqTypes>>> {
        dispatch!(self, x => x.load_quorum_proposal(view).await)
    }

    async fn load_vid_share(
        &self,
        view: ViewNumber,
    ) -> anyhow::Result<Option<Proposal<SeqTypes, VidDisperseShare<SeqTypes>>>> {
        dispatch!(self, x => x.load_vid_share(view).await)
    }

    async fn load_da_proposal(
        &self,
        view: ViewNumber,
    ) -> anyhow::Result<Option<Proposal<SeqTypes, DaProposal2<SeqTypes>>>> {
        dispatch!(self, x => x.load_da_proposal(view).await)
    }

    async fn load_upgrade_certificate(
        &self,
    ) -> anyhow::Result<Option<UpgradeCertificate<SeqTypes>>> {
        dispatch!(self, x => x.load_upgrade_certificate().await)
    }

    async fn load_start_epoch_info(&self) -> anyhow::Result<Vec<InitializerEpochInfo<SeqTypes>>> {
        dispatch!(self, x => x.load_start_epoch_info().await)
    }

    async fn load_state_cert(
        &self,
    ) -> anyhow::Result<Option<LightClientStateUpdateCertificateV2<SeqTypes>>> {
        dispatch!(self, x => x.load_state_cert().await)
    }

    async fn get_state_cert_by_epoch(
        &self,
        epoch: u64,
    ) -> anyhow::Result<Option<LightClientStateUpdateCertificateV2<SeqTypes>>> {
        dispatch!(self, x => x.get_state_cert_by_epoch(epoch).await)
    }

    async fn insert_state_cert(
        &self,
        epoch: u64,
        cert: LightClientStateUpdateCertificateV2<SeqTypes>,
    ) -> anyhow::Result<()> {
        dispatch!(self, x => x.insert_state_cert(epoch, cert).await)
    }

    async fn load_consensus_state(
        &self,
        state: NodeState,
        upgrade: Upgrade,
    ) -> anyhow::Result<(HotShotInitializer<SeqTypes>, Option<ViewNumber>)> {
        dispatch!(self, x => x.load_consensus_state(state, upgrade).await)
    }

    async fn persist_event(
        &self,
        event: &CoordinatorEvent<SeqTypes>,
        consumer: &(impl EventConsumer + 'static),
    ) -> Option<ViewNumber> {
        dispatch!(self, x => x.persist_event(event, consumer).await)
    }

    async fn append_decided_leaves(
        &self,
        decided_view: ViewNumber,
        leaf_chain: impl IntoIterator<Item = (&LeafInfo<SeqTypes>, CertificatePair<SeqTypes>)> + Send,
        consumer: &(impl EventConsumer + 'static),
    ) -> anyhow::Result<()> {
        dispatch!(self, x => x.append_decided_leaves(decided_view, leaf_chain, consumer).await)
    }

    async fn persist_decided_leaves(
        &self,
        leaf_chain: impl IntoIterator<Item = (&LeafInfo<SeqTypes>, CertificatePair<SeqTypes>)> + Send,
        cert2: Option<&Certificate2<SeqTypes>>,
        consumer: &(impl EventConsumer + 'static),
    ) -> anyhow::Result<()> {
        dispatch!(self, x => x.persist_decided_leaves(leaf_chain, cert2, consumer).await)
    }

    async fn process_decided_events(
        &self,
        decided_view: ViewNumber,
        consumer: &(impl EventConsumer + 'static),
    ) -> anyhow::Result<Option<ViewNumber>> {
        dispatch!(self, x => x.process_decided_events(decided_view, consumer).await)
    }

    async fn load_anchor_leaf(&self) -> anyhow::Result<Option<(Leaf2, CertificatePair<SeqTypes>)>> {
        dispatch!(self, x => x.load_anchor_leaf().await)
    }

    async fn append_vid(
        &self,
        proposal: &Proposal<SeqTypes, VidDisperseShare<SeqTypes>>,
    ) -> anyhow::Result<()> {
        dispatch!(self, x => x.append_vid(proposal).await)
    }

    async fn append_da(
        &self,
        proposal: &Proposal<SeqTypes, DaProposal<SeqTypes>>,
        vid_commit: VidCommitment,
    ) -> anyhow::Result<()> {
        dispatch!(self, x => x.append_da(proposal, vid_commit).await)
    }

    async fn record_action(
        &self,
        view: ViewNumber,
        epoch: Option<EpochNumber>,
        action: HotShotAction,
    ) -> anyhow::Result<()> {
        dispatch!(self, x => x.record_action(view, epoch, action).await)
    }

    async fn append_quorum_proposal2(
        &self,
        proposal: &Proposal<SeqTypes, QuorumProposalWrapper<SeqTypes>>,
    ) -> anyhow::Result<()> {
        dispatch!(self, x => x.append_quorum_proposal2(proposal).await)
    }

    async fn append_cert2(
        &self,
        view: ViewNumber,
        cert2: Certificate2<SeqTypes>,
    ) -> anyhow::Result<()> {
        dispatch!(self, x => x.append_cert2(view, cert2).await)
    }

    async fn load_cert2(&self, view: ViewNumber) -> anyhow::Result<Option<Certificate2<SeqTypes>>> {
        dispatch!(self, x => x.load_cert2(view).await)
    }

    async fn append_high_qc2(&self, high_qc: QuorumCertificate2<SeqTypes>) -> anyhow::Result<()> {
        dispatch!(self, x => x.append_high_qc2(high_qc).await)
    }

    async fn load_high_qc2(&self) -> anyhow::Result<Option<QuorumCertificate2<SeqTypes>>> {
        dispatch!(self, x => x.load_high_qc2().await)
    }

    async fn store_eqc(
        &self,
        high_qc: QuorumCertificate2<SeqTypes>,
        next_epoch_high_qc: NextEpochQuorumCertificate2<SeqTypes>,
    ) -> anyhow::Result<()> {
        dispatch!(self, x => x.store_eqc(high_qc, next_epoch_high_qc).await)
    }

    async fn load_eqc(
        &self,
    ) -> Option<(
        QuorumCertificate2<SeqTypes>,
        NextEpochQuorumCertificate2<SeqTypes>,
    )> {
        dispatch!(self, x => x.load_eqc().await)
    }

    async fn store_upgrade_certificate(
        &self,
        decided_upgrade_certificate: Option<UpgradeCertificate<SeqTypes>>,
    ) -> anyhow::Result<()> {
        dispatch!(self, x => x.store_upgrade_certificate(decided_upgrade_certificate).await)
    }

    async fn load_anchor_view(&self) -> anyhow::Result<ViewNumber> {
        dispatch!(self, x => x.load_anchor_view().await)
    }

    async fn load_next_epoch_quorum_certificate(
        &self,
    ) -> anyhow::Result<Option<NextEpochQuorumCertificate2<SeqTypes>>> {
        dispatch!(self, x => x.load_next_epoch_quorum_certificate().await)
    }

    async fn append_next_epoch_high_qc2(
        &self,
        next_epoch_high_qc: NextEpochQuorumCertificate2<SeqTypes>,
    ) -> anyhow::Result<()> {
        dispatch!(self, x => x.append_next_epoch_high_qc2(next_epoch_high_qc).await)
    }

    async fn append_da2(
        &self,
        proposal: &Proposal<SeqTypes, DaProposal2<SeqTypes>>,
        vid_commit: VidCommitment,
    ) -> anyhow::Result<()> {
        dispatch!(self, x => x.append_da2(proposal, vid_commit).await)
    }

    async fn append_proposal2(
        &self,
        proposal: &Proposal<SeqTypes, QuorumProposalWrapper<SeqTypes>>,
    ) -> anyhow::Result<()> {
        dispatch!(self, x => x.append_proposal2(proposal).await)
    }

    async fn store_drb_result(
        &self,
        epoch: EpochNumber,
        drb_result: DrbResult,
    ) -> anyhow::Result<()> {
        dispatch!(self, x => x.store_drb_result(epoch, drb_result).await)
    }

    async fn store_drb_input(&self, drb_input: DrbInput) -> anyhow::Result<()> {
        dispatch!(self, x => x.store_drb_input(drb_input).await)
    }

    async fn load_drb_input(&self, epoch: u64) -> anyhow::Result<DrbInput> {
        dispatch!(self, x => x.load_drb_input(epoch).await)
    }

    async fn add_state_cert(
        &self,
        state_cert: LightClientStateUpdateCertificateV2<SeqTypes>,
    ) -> anyhow::Result<()> {
        dispatch!(self, x => x.add_state_cert(state_cert).await)
    }

    fn enable_metrics(&mut self, metrics: &dyn Metrics) {
        dispatch!(self, x => x.enable_metrics(metrics))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::persistence::sql::SqliteOptions;

    #[tokio::test]
    async fn embedded_db_creates_sqlite_and_keeps_the_pool() {
        let dir = tempfile::tempdir().unwrap();
        let mut opt = Options::from(SqliteOptions {
            path: Some(dir.path().join("db")),
        });

        let persistence = opt.create().await.unwrap();

        assert!(matches!(persistence, Sql::Sqlite(_)));
        assert!(opt.pool.is_some());
    }
}
