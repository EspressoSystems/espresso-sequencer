use std::cmp::min;
use std::collections::BTreeMap;
use std::{sync::Arc, time::Duration};

use anyhow::Context;
use async_std::task::sleep;
use async_trait::async_trait;
use committable::Commitment;
use derive_more::From;

use hotshot_types::data::ViewNumber;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use time::format_description::well_known::Rfc3339 as TimestampFormat;
use time::OffsetDateTime;

use vbs::version::Version;

use crate::{
    BlockMerkleTree, ChainConfig, FeeAccount, FeeMerkleCommitment, L1BlockInfo, ValidatedState,
};

use super::l1::L1Client;
use super::AccountQueryData;

use derive_more::{Display, Into};
use sequencer_utils::{impl_serde_from_string_or_integer, ser::FromStringOrInteger};

/// Information about the genesis state which feeds into the genesis block header.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct GenesisHeader {
    pub timestamp: Timestamp,
}

#[derive(Hash, Copy, Clone, Debug, Display, PartialEq, Eq, From, Into)]
#[display(fmt = "{}", "_0.format(&TimestampFormat).unwrap()")]
pub struct Timestamp(OffsetDateTime);

impl_serde_from_string_or_integer!(Timestamp);

impl Default for Timestamp {
    fn default() -> Self {
        Self::from_integer(0).unwrap()
    }
}

impl Timestamp {
    pub fn unix_timestamp(&self) -> u64 {
        self.0.unix_timestamp() as u64
    }
}

impl FromStringOrInteger for Timestamp {
    type Binary = u64;
    type Integer = u64;

    fn from_binary(b: Self::Binary) -> anyhow::Result<Self> {
        Self::from_integer(b)
    }

    fn from_integer(i: Self::Integer) -> anyhow::Result<Self> {
        let unix = i.try_into().context("timestamp out of range")?;
        Ok(Self(
            OffsetDateTime::from_unix_timestamp(unix).context("invalid timestamp")?,
        ))
    }

    fn from_string(s: String) -> anyhow::Result<Self> {
        Ok(Self(
            OffsetDateTime::parse(&s, &TimestampFormat).context("invalid timestamp")?,
        ))
    }

    fn to_binary(&self) -> anyhow::Result<Self::Binary> {
        Ok(self.unix_timestamp())
    }

    fn to_string(&self) -> anyhow::Result<String> {
        Ok(format!("{self}"))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[serde(rename_all = "snake_case")]
pub enum UpgradeType {
    // Note: Wrapping this in a tuple variant causes deserialization to fail because
    // the 'chain_config' name is also provided in the TOML input.
    ChainConfig { chain_config: ChainConfig },
}

const MIN_RETRY_DELAY: Duration = Duration::from_millis(500);
const MAX_RETRY_DELAY: Duration = Duration::from_secs(5);
const BACKOFF_FACTOR: u32 = 2;
// Exponential backoff jitter as a fraction of the backoff delay, (numerator, denominator).
const BACKOFF_JITTER: (u64, u64) = (1, 10);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Upgrade {
    pub view: u64,
    pub propose_window: u64,
    #[serde(flatten)]
    pub upgrade_type: UpgradeType,
}

#[derive(Debug, Clone)]
pub struct NodeState {
    pub node_id: u64,
    pub chain_config: ChainConfig,
    pub l1_client: L1Client,
    pub peers: Arc<dyn StateCatchup>,
    pub genesis_header: GenesisHeader,
    pub genesis_state: ValidatedState,
    pub l1_genesis: Option<L1BlockInfo>,
    pub upgrades: BTreeMap<Version, Upgrade>,
    pub current_version: Version,
}

#[must_use]
fn backoff(delay: Duration) -> Duration {
    if delay >= MAX_RETRY_DELAY {
        return MAX_RETRY_DELAY;
    }

    let mut rng = rand::thread_rng();

    // Increase the backoff by the backoff factor.
    let ms = (delay * BACKOFF_FACTOR).as_millis() as u64;

    // Sample a random jitter factor in the range [0, BACKOFF_JITTER.0 / BACKOFF_JITTER.1].
    let jitter_num = rng.gen_range(0..BACKOFF_JITTER.0);
    let jitter_den = BACKOFF_JITTER.1;

    // Increase the delay by the jitter factor.
    let jitter = ms * jitter_num / jitter_den;
    let delay = Duration::from_millis(ms + jitter);

    // Bound the delay by the maximum.
    min(delay, MAX_RETRY_DELAY)
}

#[async_trait]
pub trait StateCatchup: Send + Sync + std::fmt::Debug {
    /// Try to fetch the given account state, failing without retrying if unable.
    async fn try_fetch_account(
        &self,
        height: u64,
        view: ViewNumber,
        fee_merkle_tree_root: FeeMerkleCommitment,
        account: FeeAccount,
    ) -> anyhow::Result<AccountQueryData>;

    /// Fetch the given list of accounts, retrying on transient errors.
    async fn fetch_accounts(
        &self,
        height: u64,
        view: ViewNumber,
        fee_merkle_tree_root: FeeMerkleCommitment,
        accounts: Vec<FeeAccount>,
    ) -> anyhow::Result<Vec<AccountQueryData>> {
        let mut ret = vec![];
        for account in accounts {
            // Retry until we succeed.
            let mut delay = MIN_RETRY_DELAY;
            let account = loop {
                match self
                    .try_fetch_account(height, view, fee_merkle_tree_root, account)
                    .await
                {
                    Ok(account) => break account,
                    Err(err) => {
                        tracing::warn!(%account, ?delay, "Could not fetch account, retrying: {err:#}");
                        sleep(delay).await;
                        delay = backoff(delay);
                    }
                }
            };
            ret.push(account);
        }
        Ok(ret)
    }

    /// Try to fetch and remember the blocks frontier, failing without retrying if unable.
    async fn try_remember_blocks_merkle_tree(
        &self,
        height: u64,
        view: ViewNumber,
        mt: &mut BlockMerkleTree,
    ) -> anyhow::Result<()>;

    /// Fetch and remember the blocks frontier, retrying on transient errors.
    async fn remember_blocks_merkle_tree(
        &self,
        height: u64,
        view: ViewNumber,
        mt: &mut BlockMerkleTree,
    ) -> anyhow::Result<()> {
        // Retry until we succeed.
        let mut delay = MIN_RETRY_DELAY;
        loop {
            match self.try_remember_blocks_merkle_tree(height, view, mt).await {
                Ok(()) => break,
                Err(err) => {
                    tracing::warn!(
                        ?delay,
                        "Could not fetch frontier from any peer, retrying: {err:#}"
                    );
                    sleep(delay).await;
                    delay = backoff(delay);
                }
            }
        }

        Ok(())
    }

    async fn try_fetch_chain_config(
        &self,
        commitment: Commitment<ChainConfig>,
    ) -> anyhow::Result<ChainConfig>;

    async fn fetch_chain_config(&self, commitment: Commitment<ChainConfig>) -> ChainConfig {
        // Retry until we succeed.
        let mut delay = MIN_RETRY_DELAY;

        loop {
            match self.try_fetch_chain_config(commitment).await {
                Ok(cf) => return cf,
                Err(err) => {
                    tracing::warn!(
                        ?delay,
                        "Could not fetch chain config from any peer, retrying: {err:#}"
                    );
                    sleep(delay).await;
                    delay = backoff(delay);
                }
            }
        }
    }
}
