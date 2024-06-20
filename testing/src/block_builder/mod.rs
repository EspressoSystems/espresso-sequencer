use std::collections::HashMap;

use async_broadcast::Receiver;
use async_compatibility_layer::art::async_spawn;
use async_trait::async_trait;
use futures::Stream;
use hotshot::{traits::BlockPayload, types::Event};
use hotshot_builder_api::{
    block_info::{AvailableBlockData, AvailableBlockHeaderInput, AvailableBlockInfo},
    builder::{Error, Options},
    data_source::BuilderDataSource,
};
use hotshot_types::{
    constants::Base,
    traits::{
        block_contents::{precompute_vid_commitment, EncodeBytes},
        node_implementation::NodeType,
        signature_key::BuilderSignatureKey,
    },
};
use tide_disco::{method::ReadState, App, Url};
use vbs::version::StaticVersionType;

use crate::test_builder::BuilderChange;

pub mod random;
pub use random::RandomBuilderImplementation;

pub mod simple;
pub use simple::SimpleBuilderImplementation;

#[async_trait]
pub trait TestBuilderImplementation<TYPES: NodeType>
where
    <TYPES as NodeType>::InstanceState: Default,
{
    type Config: Default;

    async fn start(
        num_storage_nodes: usize,
        url: Url,
        options: Self::Config,
        changes: HashMap<u64, BuilderChange>,
    ) -> Box<dyn BuilderTask<TYPES>>;
}

pub trait BuilderTask<TYPES: NodeType>: Send + Sync {
    fn start(
        self: Box<Self>,
        stream: Box<dyn Stream<Item = Event<TYPES>> + std::marker::Unpin + Send + 'static>,
    );
}

/// Entry for a built block
#[derive(Debug, Clone)]
struct BlockEntry<TYPES: NodeType> {
    metadata: AvailableBlockInfo<TYPES>,
    payload: Option<AvailableBlockData<TYPES>>,
    header_input: Option<AvailableBlockHeaderInput<TYPES>>,
}

/// Construct a tide disco app that mocks the builder API.
///
/// # Panics
/// If constructing and launching the builder fails for any reason
pub fn run_builder_source<TYPES, Source>(
    url: Url,
    mut change_receiver: Receiver<BuilderChange>,
    source: Source,
) where
    TYPES: NodeType,
    <TYPES as NodeType>::InstanceState: Default,
    Source: Clone + Send + Sync + tide_disco::method::ReadState + 'static,
    <Source as ReadState>::State: Sync + Send + BuilderDataSource<TYPES>,
{
    async_spawn(async move {
        let start_builder = |url: Url, source: Source| -> _ {
            let builder_api = hotshot_builder_api::builder::define_api::<Source, TYPES, Base>(
                &Options::default(),
            )
            .expect("Failed to construct the builder API");
            let mut app: App<Source, Error> = App::with_state(source);
            app.register_module("block_info", builder_api)
                .expect("Failed to register the builder API");
            async_spawn(app.serve(url, Base::instance()))
        };

        let mut handle = Some(start_builder(url.clone(), source.clone()));

        while let Ok(event) = change_receiver.recv().await {
            match event {
                BuilderChange::Up if handle.is_none() => {
                    handle = Some(start_builder(url.clone(), source.clone()));
                }
                BuilderChange::Down => {
                    if let Some(handle) = handle.take() {
                        #[cfg(async_executor_impl = "tokio")]
                        handle.abort();
                        #[cfg(async_executor_impl = "async-std")]
                        handle.cancel().await;
                    }
                }
                _ => {}
            }
        }
    });
}

/// Helper function to construct all builder data structures from a list of transactions
async fn build_block<TYPES: NodeType>(
    transactions: Vec<TYPES::Transaction>,
    num_storage_nodes: usize,
    pub_key: TYPES::BuilderSignatureKey,
    priv_key: <TYPES::BuilderSignatureKey as BuilderSignatureKey>::BuilderPrivateKey,
) -> BlockEntry<TYPES>
where
    <TYPES as NodeType>::InstanceState: Default,
{
    let (block_payload, metadata) = TYPES::BlockPayload::from_transactions(
        transactions,
        &Default::default(),
        &Default::default(),
    )
    .await
    .expect("failed to build block payload from transactions");

    let commitment = block_payload.builder_commitment(&metadata);

    let (vid_commitment, precompute_data) =
        precompute_vid_commitment(&block_payload.encode(), num_storage_nodes);

    // Get block size from the encoded payload
    let block_size = block_payload.encode().len() as u64;

    let signature_over_block_info =
        TYPES::BuilderSignatureKey::sign_block_info(&priv_key, block_size, 123, &commitment)
            .expect("Failed to sign block info");

    let signature_over_builder_commitment =
        TYPES::BuilderSignatureKey::sign_builder_message(&priv_key, commitment.as_ref())
            .expect("Failed to sign commitment");

    let signature_over_vid_commitment =
        TYPES::BuilderSignatureKey::sign_builder_message(&priv_key, vid_commitment.as_ref())
            .expect("Failed to sign block vid commitment");

    let signature_over_fee_info =
        TYPES::BuilderSignatureKey::sign_fee(&priv_key, 123_u64, &metadata, &vid_commitment)
            .expect("Failed to sign fee info");

    let block = AvailableBlockData {
        block_payload,
        metadata,
        sender: pub_key.clone(),
        signature: signature_over_builder_commitment,
    };
    let metadata = AvailableBlockInfo {
        sender: pub_key.clone(),
        signature: signature_over_block_info,
        block_hash: commitment,
        block_size,
        offered_fee: 123,
        _phantom: std::marker::PhantomData,
    };
    let header_input = AvailableBlockHeaderInput {
        vid_commitment,
        vid_precompute_data: precompute_data,
        message_signature: signature_over_vid_commitment.clone(),
        fee_signature: signature_over_fee_info,
        sender: pub_key,
    };

    BlockEntry {
        metadata,
        payload: Some(block),
        header_input: Some(header_input),
    }
}
