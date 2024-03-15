use async_compatibility_layer::art::async_spawn;
use async_trait::async_trait;
use futures::future::BoxFuture;
use hotshot::traits::BlockPayload;
use hotshot::types::SignatureKey;
use hotshot_example_types::{block_types::TestBlockPayload, node_types::TestTypes};
use hotshot_types::traits::block_contents::vid_commitment;
use hotshot_types::utils::BuilderCommitment;
use hotshot_types::{traits::node_implementation::NodeType, vid::VidCommitment};
use hs_builder_api::block_info::{
    AvailableBlockData, AvailableBlockHeaderInput, AvailableBlockInfo,
};
use hs_builder_api::{
    builder::{BuildError, Options},
    data_source::BuilderDataSource,
};
use tide_disco::{method::ReadState, App, Url};

/// The only block [`TestableBuilderSource`] provides
const EMPTY_BLOCK: TestBlockPayload = TestBlockPayload {
    transactions: vec![],
};

/// A mock implementation of the builder data source.
/// "Builds" only empty blocks.
pub struct TestableBuilderSource {
    priv_key: <<TestTypes as NodeType>::SignatureKey as SignatureKey>::PrivateKey,
    pub_key: <TestTypes as NodeType>::SignatureKey,
}

#[async_trait]
impl ReadState for TestableBuilderSource {
    type State = Self;

    async fn read<T>(
        &self,
        op: impl Send + for<'a> FnOnce(&'a Self::State) -> BoxFuture<'a, T> + 'async_trait,
    ) -> T {
        op(self).await
    }
}

#[async_trait]
impl BuilderDataSource<TestTypes> for TestableBuilderSource {
    async fn get_available_blocks(
        &self,
        _for_parent: &VidCommitment,
    ) -> Result<Vec<AvailableBlockInfo<TestTypes>>, BuildError> {
        Ok(vec![AvailableBlockInfo {
            sender: self.pub_key,
            signature: <TestTypes as NodeType>::SignatureKey::sign(
                &self.priv_key,
                EMPTY_BLOCK.builder_commitment(&()).as_ref(),
            )
            .unwrap(),
            block_hash: EMPTY_BLOCK.builder_commitment(&()),
            block_size: 0,
            offered_fee: 1,
            _phantom: std::marker::PhantomData,
        }])
    }

    async fn claim_block(
        &self,
        block_hash: &BuilderCommitment,
        _signature: &<<TestTypes as NodeType>::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    ) -> Result<AvailableBlockData<TestTypes>, BuildError> {
        if block_hash == &EMPTY_BLOCK.builder_commitment(&()) {
            Ok(AvailableBlockData {
                block_payload: EMPTY_BLOCK,
                metadata: (),
                signature: <TestTypes as NodeType>::SignatureKey::sign(
                    &self.priv_key,
                    EMPTY_BLOCK.builder_commitment(&()).as_ref(),
                )
                .unwrap(),
                sender: self.pub_key,
                _phantom: std::marker::PhantomData,
            })
        } else {
            Err(BuildError::Missing)
        }
    }

    async fn claim_block_header_input(
        &self,
        block_hash: &BuilderCommitment,
        _signature: &<<TestTypes as NodeType>::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    ) -> Result<AvailableBlockHeaderInput<TestTypes>, BuildError> {
        if block_hash == &EMPTY_BLOCK.builder_commitment(&()) {
            Ok(AvailableBlockHeaderInput {
                vid_commitment: vid_commitment(&vec![], 1),
                signature: <TestTypes as NodeType>::SignatureKey::sign(
                    &self.priv_key,
                    EMPTY_BLOCK.builder_commitment(&()).as_ref(),
                )
                .unwrap(),
                sender: self.pub_key,
                _phantom: std::marker::PhantomData,
            })
        } else {
            Err(BuildError::Missing)
        }
    }
}

/// Construct a tide disco app that mocks the builder API.
///
/// # Panics
/// If constructing and launching the builder fails for any reason
pub fn run_builder(url: Url) {
    let builder_api = hs_builder_api::builder::define_api::<TestableBuilderSource, TestTypes>(
        &Options::default(),
    )
    .expect("Failed to construct the builder API");
    let (pub_key, priv_key) =
        <TestTypes as NodeType>::SignatureKey::generated_from_seed_indexed([1; 32], 0);
    let mut app: App<TestableBuilderSource, hs_builder_api::builder::Error> =
        App::with_state(TestableBuilderSource { priv_key, pub_key });
    app.register_module("/", builder_api)
        .expect("Failed to register the builder API");

    async_spawn(app.serve(url));
}
