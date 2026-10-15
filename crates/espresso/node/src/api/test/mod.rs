use std::{
    collections::{BTreeSet, HashMap, HashSet},
    time::{Duration, Instant},
};

use ::light_client::{
    consensus::{
        header::HeaderProof,
        leaf::{LeafProof, LeafProofHint},
        payload::PayloadProof,
    },
    testing::EpochChangeQuorum,
};
use alloy::{
    eips::BlockId,
    network::EthereumWallet,
    primitives::{Address, U256},
    providers::{ProviderBuilder, ext::AnvilApi},
};
use async_lock::Mutex;
use committable::{Commitment, Committable};
use espresso_contract_deployer::{
    Contract, Contracts, builder::DeployerArgsBuilder,
    network_config::light_client_genesis_from_stake_table,
};
use espresso_types::{
    Certificate2, FeeAmount, GenesisHeader, Header, L1ClientOptions, NamespaceId,
    NamespaceProofQueryData, NsProof, RegisteredValidatorMap, RewardDistributor, StakeTableState,
    StateCertQueryDataV1, StateCertQueryDataV2, TEST_UPGRADE, ValidatedState,
    ValidatorLeaderCounts,
    config::PublicHotShotConfig,
    traits::{NullEventConsumer, PersistenceOptions},
    v0_3::{AuthenticatedValidator, COMMISSION_BASIS_POINTS, Fetcher, RewardAmount},
    v0_4::{RewardAccountV2, RewardMerkleProofV2},
    validators_from_l1_events,
};
use futures::{
    future::{self, join_all, try_join_all},
    stream::{StreamExt, TryStreamExt},
    try_join,
};
use hotshot::types::{Event, EventType};
use hotshot_contract_adapter::{
    reward::{RewardAuthData, RewardClaimInput, RewardProofSiblings},
    sol_types::{EspToken, StakeTableV3},
    stake_table::StakeTableContractVersion,
};
use hotshot_query_service::{
    availability::{
        BlockQueryData, BlockSummaryQueryData, LeafQueryData, TransactionQueryData,
        VidCommonQueryData,
    },
    data_source::{
        Transaction as _, VersionedDataSource,
        sql::Config,
        storage::{
            MerklizedStateHeightStorage, NodeStorage, SerializableRetry, SqlStorage,
            StorageConnectionType, UpdateAvailabilityStorage, pruning::PrunedHeightStorage,
        },
    },
    explorer::TransactionSummariesResponse,
    merklized_state::UpdateStateData,
    node::{NodeDataSource as _, SyncStatus, SyncStatusQueryData},
    serializable_retry,
    types::HeightIndexed,
};
use hotshot_types::{
    ValidatorConfig,
    addr::NetAddr,
    data::EpochNumber,
    epoch_membership::EpochMembershipCoordinator,
    event::LeafInfo,
    new_protocol::CoordinatorEvent,
    traits::{block_contents::BlockHeader, election::Membership, metrics::NoMetrics},
    utils::epoch_from_block_number,
    x25519,
};
use http_client::{
    Client, StatusCode,
    error::ClientErr,
    healthcheck::{AppHealth, HealthStatus},
};
use jf_merkle_tree_compat::{
    MerkleTreeScheme,
    prelude::{MerkleProof, Sha3Node},
};
use pretty_assertions::assert_matches;
use rand::{seq::SliceRandom, thread_rng};
use rstest::rstest;
use staking_cli::{
    Transaction as StakingTransaction, demo::DelegationConfig, fetch_commission, update_commission,
    update_network_config,
};
use test_helpers::{
    TestNetwork, TestNetworkConfigBuilder, catchup_test_helper, node_storage,
    state_signature_test_helper, status_test_helper, submit_test_helper, wait_for_committee,
};
use test_utils::reserve_tcp_port;
use tokio::time::sleep;
use vbs::version::StaticVersion;
use versions::{
    DRB_AND_HEADER_UPGRADE_VERSION, EPOCH_VERSION, LARGE_BLOCK_VERSION, NEW_PROTOCOL_VERSION,
    Upgrade,
};

use self::{
    data_source::{SequencerDataSource, testing::TestableSequencerDataSource},
    sql::DataSource as SqlDataSource,
};
use super::*;
use crate::{
    CatchupParams, L1Params,
    api::{
        options::Query,
        sql::{impl_testable_data_source::tmp_options, reconstruct_state},
        test_helpers::STAKE_TABLE_CAPACITY_FOR_TEST,
    },
    catchup::{NullStateCatchup, StatePeers},
    follower::{FollowerContext, FollowerOptions, FollowerParams, init_follower_node},
    genesis::{Genesis as NodeGenesis, L1Finalized, StakeTableConfig},
    legacy_chain::LegacyChain,
    persistence,
    persistence::no_storage,
    testing::{
        TestConfig, TestConfigBuilder, decided_leaves, wait_for_decide_on_handle, wait_for_epochs,
    },
};

mod catchup;
mod endpoints;
mod light_client;
mod namespace;
mod rewards;
mod stake_table;
mod upgrades;
mod v2_api;

async fn wait_until_block_height(
    client: &Client<ClientErr, StaticVersion<0, 1>>,
    endpoint: &str,
    height: u64,
) {
    for _retry in 0.. {
        let bh = client
            .get::<u64>(endpoint)
            .send()
            .await
            .expect("block height not found");

        if bh >= height {
            return;
        }
        sleep(Duration::from_secs(3)).await;
    }
}

type HttpClient = Client<ClientErr, StaticVersion<0, 1>>;

async fn fetch<T>(client: &HttpClient, route: &str) -> T
where
    T: serde::de::DeserializeOwned,
{
    client
        .get(route)
        .send()
        .await
        .unwrap_or_else(|err| panic!("{route}: {err}"))
}

async fn error_status(client: &HttpClient, route: &str) -> StatusCode {
    client
        .get::<serde_json::Value>(route)
        .send()
        .await
        .expect_err(route)
        .status
}
