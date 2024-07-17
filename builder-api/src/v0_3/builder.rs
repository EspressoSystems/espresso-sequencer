use futures::FutureExt;
use hotshot_types::{traits::node_implementation::NodeType, utils::BuilderCommitment};
use snafu::ResultExt;
use tide_disco::{api::ApiError, method::ReadState, Api};

use super::{data_source::BuilderDataSource, Version};
/// No changes to these types
pub use crate::v0_1::builder::{
    submit_api, BlockAvailableSnafu, BlockClaimSnafu, BuildError, BuilderAddressSnafu, Error,
    Options,
};
use crate::{api::load_api, v0_1::builder::try_extract_param};

pub fn define_api<State, Types: NodeType>(
    options: &Options,
) -> Result<Api<State, Error, Version>, ApiError>
where
    State: 'static + Send + Sync + ReadState,
    <State as ReadState>::State: Send + Sync + BuilderDataSource<Types>,
{
    let mut api = load_api::<State, Error, Version>(
        options.api_path.as_ref(),
        include_str!("../../api/v0_3/builder.toml"),
        options.extensions.clone(),
    )?;
    api.with_version("0.0.3".parse().unwrap())
        .get("available_blocks", |req, state| {
            async move {
                let hash = req.blob_param("parent_hash")?;
                let view_number = req.integer_param("view_number")?;
                let signature = try_extract_param(&req, "signature")?;
                let sender = try_extract_param(&req, "sender")?;
                state
                    .available_blocks(&hash, view_number, sender, &signature)
                    .await
                    .context(BlockAvailableSnafu {
                        resource: hash.to_string(),
                    })
            }
            .boxed()
        })?
        .get("claim_block", |req, state| {
            async move {
                let block_hash: BuilderCommitment = req.blob_param("block_hash")?;
                let view_number = req.integer_param("view_number")?;
                let signature = try_extract_param(&req, "signature")?;
                let sender = try_extract_param(&req, "sender")?;
                state
                    .claim_block(&block_hash, view_number, sender, &signature)
                    .await
                    .context(BlockClaimSnafu {
                        resource: block_hash.to_string(),
                    })
            }
            .boxed()
        })?
        .get("builder_address", |_req, state| {
            async move { state.builder_address().await.context(BuilderAddressSnafu) }.boxed()
        })?;
    Ok(api)
}
