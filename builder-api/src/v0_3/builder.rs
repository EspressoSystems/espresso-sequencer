use futures::FutureExt;
use hotshot_types::traits::node_implementation::NodeType;
use snafu::ResultExt;
use tide_disco::{api::ApiError, method::ReadState, Api};

use super::{data_source::BuilderDataSource, Version};
use crate::api::load_api;
/// No changes to these types
pub use crate::v0_1::builder::{
    submit_api, BlockAvailableSnafu, BlockClaimSnafu, BuildError, BuilderAddressSnafu, Error,
    Options,
};

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
        .get("bundle", |req, state| {
            async move {
                let view_number = req.integer_param("view_number")?;
                state.bundle(view_number).await.context(BlockClaimSnafu {
                    resource: view_number.to_string(),
                })
            }
            .boxed()
        })?
        .get("builder_address", |_req, state| {
            async move { state.builder_address().await.context(BuilderAddressSnafu) }.boxed()
        })?;
    Ok(api)
}
