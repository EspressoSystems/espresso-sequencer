use anyhow::Context;
use async_std::task::sleep;
use espresso_types::{FeeAccount, FeeAmount, FeeMerkleTree};
use ethers::types::Address;
use jf_merkle_tree::{
    prelude::{MerkleProof, Sha3Node},
    MerkleTreeScheme,
};
use std::time::Duration;
use surf_disco::{
    error::ClientError,
    http::convert::DeserializeOwned,
    socket::{Connection, Unsupported},
    Url,
};
use vbs::version::StaticVersion;

pub type SequencerApiVersion = StaticVersion<0, 1>;
// pub type EspressoClient = surf_disco::Client<ClientError, SequencerApiVersion>;

#[derive(Clone, Debug)]
pub struct EspressoClient(surf_disco::Client<ClientError, SequencerApiVersion>);

pub type FeeMerkleProof = MerkleProof<FeeAmount, FeeAccount, Sha3Node, { FeeMerkleTree::ARITY }>;

impl EspressoClient {
    pub fn new(provider: Url) -> Self {
        Self(surf_disco::Client::new(provider))
    }

    /// GET Block Height from the node
    pub async fn get_height(&self) -> anyhow::Result<u64> {
        self.0
            .get::<u64>("node/block-height")
            .send()
            .await
            .context("getting Espresso block height")
    }

    /// Get the Number of Transactions
    pub async fn get_transaction_count(&self) -> anyhow::Result<u64> {
        self.0
            .get::<u64>("node/transactions/count")
            .send()
            .await
            .context("getting Espresso transaction count")
    }

    /// Subscribe to a stream to Block Headers
    pub async fn subscribe_headers<FromServer: DeserializeOwned>(
        &self,
        connect: &str,
    ) -> anyhow::Result<Connection<FromServer, Unsupported, ClientError, SequencerApiVersion>> {
        self.0
            .socket(connect)
            .subscribe()
            .await
            .context("subscribing to Espresso headers")
    }

    /// Get the balance for a given account at a given block height, defaulting to current balance.
    pub async fn get_espresso_balance(
        &self,
        address: Address,
        block: Option<u64>,
    ) -> anyhow::Result<FeeAmount> {
        // Get the block height to query at, defaulting to the latest block.
        let block = if let Some(block) = block {
            block - 1
        } else {
            self.0
                .get::<u64>("node/block-height")
                .send()
                .await
                .context("getting block height")?
                - 1
        };

        // Download the Merkle path for this fee account at the specified block height. Transient errors
        // are possible (for example, if we are fetching from the latest block, the block height might
        // get incremented slightly before the state becomes available) so retry a few times.
        let mut retry = 0;
        let max_retries = 5;
        let proof = loop {
            tracing::debug!(%address, block, retry, "fetching Espresso balance");
            match self
                .0
                .get::<FeeMerkleProof>(&format!("fee-state/{block}/{address:#x}"))
                .send()
                .await
            {
                Ok(proof) => break proof,
                Err(err) => {
                    tracing::warn!("error getting account balance: {err:#}");
                    retry += 1;

                    if retry == max_retries {
                        return Err(err).context("getting account balance");
                    } else {
                        sleep(Duration::from_millis(200)).await;
                    }
                }
            }
        };

        // If the element in the Merkle path is missing -- there is no account with this address -- the
        // balance is defined to be 0.
        let balance = proof.elem().copied().unwrap_or(0.into());
        Ok(balance)
    }
}
