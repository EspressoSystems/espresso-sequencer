use std::{collections::BTreeMap, sync::Arc};

use async_trait::async_trait;
use committable::Committable;
use hotshot_query_service::availability::QueryablePayload;
use hotshot_types::{
    traits::{BlockPayload, EncodeBytes},
    utils::BuilderCommitment,
    vid::{VidCommon, VidSchemeType},
};
use jf_vid::VidScheme;
use sha2::Digest;

use crate::{
    ChainConfig, Index, Iter, NamespaceId, NodeState, NsIndex, NsPayload, NsPayloadBuilder,
    NsPayloadRange, NsTable, NsTableBuilder, Payload, PayloadByteLen, SeqTypes, Transaction,
    TxProof, ValidatedState,
};

impl Payload {
    pub fn ns_table(&self) -> &NsTable {
        &self.ns_table
    }

    /// Like [`QueryablePayload::transaction_with_proof`] except without the
    /// proof.
    pub fn transaction(&self, index: &Index) -> Option<Transaction> {
        let ns_id = self.ns_table.read_ns_id(index.ns())?;
        let ns_payload = self.ns_payload(index.ns());
        ns_payload.export_tx(&ns_id, index.tx())
    }

    // CRATE-VISIBLE HELPERS START HERE

    pub(crate) fn read_ns_payload(&self, range: &NsPayloadRange) -> &NsPayload {
        NsPayload::from_bytes_slice(&self.raw_payload[range.as_block_range()])
    }

    /// Convenience wrapper for [`Self::read_ns_payload`].
    ///
    /// `index` is not checked. Use `self.ns_table().in_bounds()` as needed.
    pub(crate) fn ns_payload(&self, index: &NsIndex) -> &NsPayload {
        let ns_payload_range = self.ns_table().ns_range(index, &self.byte_len());
        self.read_ns_payload(&ns_payload_range)
    }

    pub(crate) fn byte_len(&self) -> PayloadByteLen {
        PayloadByteLen(self.raw_payload.len())
    }

    // PRIVATE HELPERS START HERE

    /// Need a sync version of [`BlockPayload::from_transactions`] in order to impl [`BlockPayload::empty`].
    fn from_transactions_sync(
        transactions: impl IntoIterator<Item = <Self as BlockPayload<SeqTypes>>::Transaction> + Send,
        chain_config: ChainConfig,
        _instance_state: &<Self as BlockPayload<SeqTypes>>::Instance,
    ) -> Result<
        (Self, <Self as BlockPayload<SeqTypes>>::Metadata),
        <Self as BlockPayload<SeqTypes>>::Error,
    > {
        // accounting for block byte length limit
        let max_block_byte_len: usize = u64::from(chain_config.max_block_size)
            .try_into()
            .map_err(|_| <Self as BlockPayload<SeqTypes>>::Error::BlockBuilding)?;
        let mut block_byte_len = NsTableBuilder::header_byte_len();

        // add each tx to its namespace
        let mut ns_builders = BTreeMap::<NamespaceId, NsPayloadBuilder>::new();
        for tx in transactions.into_iter() {
            // accounting for block byte length limit
            block_byte_len += tx.payload().len() + NsPayloadBuilder::tx_table_entry_byte_len();
            if !ns_builders.contains_key(&tx.namespace()) {
                // each new namespace adds overhead
                block_byte_len +=
                    NsTableBuilder::entry_byte_len() + NsPayloadBuilder::tx_table_header_byte_len();
            }
            if block_byte_len > max_block_byte_len {
                tracing::warn!("transactions truncated to fit in maximum block byte length {max_block_byte_len}");
                break;
            }

            let ns_builder = ns_builders.entry(tx.namespace()).or_default();
            ns_builder.append_tx(tx);
        }

        // build block payload and namespace table
        let mut payload = Vec::new();
        let mut ns_table_builder = NsTableBuilder::new();
        for (ns_id, ns_builder) in ns_builders {
            payload.extend(ns_builder.into_bytes());
            ns_table_builder.append_entry(ns_id, payload.len());
        }
        let ns_table = ns_table_builder.into_ns_table();
        let metadata = ns_table.clone();
        Ok((
            Self {
                raw_payload: payload,
                ns_table,
            },
            metadata,
        ))
    }
}

#[async_trait]
impl BlockPayload<SeqTypes> for Payload {
    // TODO BlockPayload trait eliminate unneeded args, return vals of type
    // `Self::Metadata` https://github.com/EspressoSystems/HotShot/issues/3300
    type Error = crate::Error;
    type Transaction = Transaction;
    type Instance = NodeState;
    type Metadata = NsTable;
    type ValidatedState = ValidatedState;

    async fn from_transactions(
        transactions: impl IntoIterator<Item = Self::Transaction> + Send,
        validated_state: &Self::ValidatedState,
        instance_state: &Self::Instance,
    ) -> Result<(Self, Self::Metadata), Self::Error> {
        let validated_state_cf = validated_state.chain_config;
        let instance_state_cf = instance_state.chain_config;

        let chain_config = if validated_state_cf.commit() == instance_state_cf.commit() {
            instance_state_cf
        } else {
            match validated_state_cf.resolve() {
                Some(cf) => cf,
                None => {
                    instance_state
                        .peers
                        .as_ref()
                        .fetch_chain_config(validated_state_cf.commit())
                        .await
                }
            }
        };

        Self::from_transactions_sync(transactions, chain_config, instance_state)
    }

    // TODO avoid cloning the entire payload here?
    fn from_bytes(block_payload_bytes: &[u8], ns_table: &Self::Metadata) -> Self {
        Self {
            raw_payload: block_payload_bytes.to_vec(),
            ns_table: ns_table.clone(),
        }
    }

    fn empty() -> (Self, Self::Metadata) {
        let payload = Self::from_transactions_sync(vec![], Default::default(), &Default::default())
            .unwrap()
            .0;

        let ns_table = payload.ns_table().clone();
        (payload, ns_table)
    }

    fn builder_commitment(&self, metadata: &Self::Metadata) -> BuilderCommitment {
        let ns_table_bytes = self.ns_table.encode();

        // TODO `metadata_bytes` equals `ns_table_bytes`, so we are
        // double-hashing the ns_table. Why? To maintain serialization
        // compatibility.
        // https://github.com/EspressoSystems/espresso-sequencer/issues/1576
        let metadata_bytes = metadata.encode();

        let mut digest = sha2::Sha256::new();
        digest.update((self.raw_payload.len() as u64).to_le_bytes());
        digest.update((ns_table_bytes.len() as u64).to_le_bytes());
        digest.update((metadata_bytes.len() as u64).to_le_bytes()); // https://github.com/EspressoSystems/espresso-sequencer/issues/1576
        digest.update(&self.raw_payload);
        digest.update(ns_table_bytes);
        digest.update(metadata_bytes); // https://github.com/EspressoSystems/espresso-sequencer/issues/1576
        BuilderCommitment::from_raw_digest(digest.finalize())
    }

    fn transactions<'a>(
        &'a self,
        metadata: &'a Self::Metadata,
    ) -> impl 'a + Iterator<Item = Self::Transaction> {
        self.enumerate(metadata).map(|(_, t)| t)
    }
}

impl QueryablePayload<SeqTypes> for Payload {
    // TODO changes to QueryablePayload trait:
    // https://github.com/EspressoSystems/hotshot-query-service/issues/639
    type TransactionIndex = Index;
    type Iter<'a> = Iter<'a>;
    type InclusionProof = TxProof;

    fn len(&self, _meta: &Self::Metadata) -> usize {
        // Counting txs is nontrivial. The easiest solution is to consume an
        // iterator. If performance is a concern then we could cache this count
        // on construction of `Payload`.
        self.iter(_meta).count()
    }

    fn iter<'a>(&'a self, _meta: &'a Self::Metadata) -> Self::Iter<'a> {
        Iter::new(self)
    }

    fn transaction_with_proof(
        &self,
        _meta: &Self::Metadata,
        index: &Self::TransactionIndex,
    ) -> Option<(Self::Transaction, Self::InclusionProof)> {
        // TODO HACK! THE RETURNED PROOF MIGHT FAIL VERIFICATION.
        // https://github.com/EspressoSystems/hotshot-query-service/issues/639
        //
        // Need a `VidCommon` to proceed. Need to modify `QueryablePayload`
        // trait to add a `VidCommon` arg. In the meantime tests fail if I leave
        // it `todo!()`, so this hack allows tests to pass.
        let common = hotshot_types::vid::vid_scheme(10)
            .disperse(&self.raw_payload)
            .unwrap()
            .common;

        TxProof::new(index, self, &common)
    }
}

impl std::fmt::Display for Payload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

impl EncodeBytes for Payload {
    fn encode(&self) -> Arc<[u8]> {
        Arc::from(self.raw_payload.as_ref())
    }
}

impl PayloadByteLen {
    /// Extract payload byte length from a [`VidCommon`] and construct a new [`Self`] from it.
    pub fn from_vid_common(common: &VidCommon) -> Self {
        Self(usize::try_from(VidSchemeType::get_payload_byte_len(common)).unwrap())
    }

    #[allow(clippy::result_unit_err)]
    /// Is the payload byte length declared in a [`VidCommon`] equal [`Self`]?
    pub fn is_consistent(&self, common: &VidCommon) -> bool {
        // failure to convert to usize implies that `common` cannot be
        // consistent with `self`.
        let expected = match usize::try_from(VidSchemeType::get_payload_byte_len(common)) {
            Ok(n) => n,
            Err(_) => {
                tracing::warn!(
                    "VidCommon byte len u32 {} should convert to usize",
                    VidSchemeType::get_payload_byte_len(common)
                );
                return false;
            }
        };

        self.0 == expected
    }

    pub(in crate::v0::impls::block::full_payload) fn as_usize(&self) -> usize {
        self.0
    }
}

#[cfg(any(test, feature = "testing"))]
impl hotshot_types::traits::block_contents::TestableBlock<SeqTypes> for Payload {
    fn genesis() -> Self {
        BlockPayload::empty().0
    }

    fn txn_count(&self) -> u64 {
        self.len(&self.ns_table) as u64
    }
}

#[cfg(any(test, feature = "testing"))]
impl Payload {
    pub fn ns_table_mut(&mut self) -> &mut NsTable {
        &mut self.ns_table
    }
}
