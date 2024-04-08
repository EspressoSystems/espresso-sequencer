// Copyright (c) 2022 Espresso Systems (espressosys.com)
// This file is part of the HotShot Query Service library.
//
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU
// General Public License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
// You should have received a copy of the GNU General Public License along with this program. If not,
// see <https://www.gnu.org/licenses/>.

//! Queries for node-specific state and uncommitted data.
//!
//! Unlike the [availability](crate::availability) and [node](crate::node) APIs, which deal only
//! with committed data (albeit with different consistency properties), the status API offers a
//! glimpse into internal consensus state and uncommitted data. Here you can find low-level
//! information about a particular node, such as consensus and networking metrics. You can also find
//! information about pending blocks and transactions in the mempool.
//!
//! The status API is intended to be a lightweight way to inspect the activities and health of a
//! consensus node. It is the only API that can be run without any persistent storage, and its
//! memory overhead is also very low. As a consequence, it only serves two types of data:
//! * snapshots of the state right now, with no way to query historical snapshots
//! * summary statistics

use crate::api::load_api;
use clap::Args;
use derive_more::From;
use futures::FutureExt;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::borrow::Cow;
use std::fmt::Display;
use std::path::PathBuf;
use tide_disco::{api::ApiError, method::ReadState, Api, RequestError, StatusCode};
use vbs::version::StaticVersionType;

pub(crate) mod data_source;
pub(crate) mod query_data;
pub use data_source::*;
pub use query_data::*;

#[derive(Args, Default)]
pub struct Options {
    #[arg(long = "status-api-path", env = "HOTSHOT_STATUS_API_PATH")]
    pub api_path: Option<PathBuf>,

    /// Additional API specification files to merge with `status-api-path`.
    ///
    /// These optional files may contain route definitions for application-specific routes that have
    /// been added as extensions to the basic status API.
    #[arg(
        long = "status-extension",
        env = "HOTSHOT_STATUS_EXTENSIONS",
        value_delimiter = ','
    )]
    pub extensions: Vec<toml::Value>,
}

#[derive(Clone, Debug, From, Snafu, Deserialize, Serialize)]
pub enum Error {
    Request { source: RequestError },
    Internal { reason: String },
}

impl Error {
    pub fn status(&self) -> StatusCode {
        match self {
            Self::Request { .. } => StatusCode::BadRequest,
            Self::Internal { .. } => StatusCode::InternalServerError,
        }
    }
}

fn internal<M: Display>(msg: M) -> Error {
    Error::Internal {
        reason: msg.to_string(),
    }
}

pub fn define_api<State, Ver: StaticVersionType + 'static>(
    options: &Options,
    _: Ver,
) -> Result<Api<State, Error, Ver>, ApiError>
where
    State: 'static + Send + Sync + ReadState,
    <State as ReadState>::State: Send + Sync + StatusDataSource,
{
    let mut api = load_api::<State, Error, Ver>(
        options.api_path.as_ref(),
        include_str!("../api/status.toml"),
        options.extensions.clone(),
    )?;
    api.with_version("0.0.1".parse().unwrap())
        .get("block_height", |_, state| {
            async { state.block_height().await.map_err(internal) }.boxed()
        })?
        .get("mempool_info", |_, state| {
            async { state.mempool_info().await.map_err(internal) }.boxed()
        })?
        .get("success_rate", |_, state| {
            async { state.success_rate().await.map_err(internal) }.boxed()
        })?
        .get("time_since_last_decide", |_, state| {
            async {
                state
                    .elapsed_time_since_last_decide()
                    .await
                    .map_err(internal)
            }
            .boxed()
        })?
        .metrics("metrics", |_, state| {
            async { Ok(Cow::Borrowed(state.metrics())) }.boxed()
        })?;
    Ok(api)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        data_source::ExtensibleDataSource,
        task::BackgroundTask,
        testing::{
            consensus::{MockDataSource, MockNetwork},
            mocks::MockTransaction,
            setup_test, sleep,
        },
        Error,
    };
    use async_std::sync::RwLock;
    use bincode::Options as _;
    use futures::FutureExt;
    use hotshot_types::constants::{Version01, STATIC_VER_0_1};
    use hotshot_types::utils::bincode_opts;
    use portpicker::pick_unused_port;
    use std::str::FromStr;
    use std::time::Duration;
    use surf_disco::Client;
    use tempfile::TempDir;
    use tide_disco::{App, Url};
    use toml::toml;

    #[async_std::test]
    async fn test_api() {
        setup_test();

        // Create the consensus network.
        let mut network = MockNetwork::<MockDataSource>::init().await;

        // Start the web server.
        let port = pick_unused_port().unwrap();
        let mut app = App::<_, Error>::with_state(network.data_source());
        app.register_module(
            "status",
            define_api(&Default::default(), STATIC_VER_0_1).unwrap(),
        )
        .unwrap();
        network.spawn(
            "server",
            app.serve(format!("0.0.0.0:{}", port), STATIC_VER_0_1),
        );

        // Start a client.
        let url = Url::from_str(&format!("http://localhost:{}/status", port)).unwrap();
        let client = Client::<Error, Version01>::new(url.clone());
        assert!(client.connect(Some(Duration::from_secs(60))).await);

        // Submit a transaction. We have not yet started the validators, so this transaction will
        // stay in the mempool, allowing us to check the mempool endpoint.
        let txn = MockTransaction::default();
        let txn_size = bincode_opts().serialized_size(&txn).unwrap();
        network.submit_transaction(txn.clone()).await;
        loop {
            let mempool = client
                .get::<MempoolQueryData>("mempool-info")
                .send()
                .await
                .unwrap();
            let expected = MempoolQueryData {
                transaction_count: 1,
                memory_footprint: txn_size,
            };
            if mempool == expected {
                break;
            }
            tracing::info!(
                "waiting for mempool to reflect transaction (currently {:?})",
                mempool
            );
            sleep(Duration::from_secs(1)).await;
        }
        // The block height is initially zero.
        assert_eq!(client.get::<u64>("block-height").send().await.unwrap(), 0);

        // Test Prometheus export.
        let mut res = surf::get(&format!("{url}/metrics")).send().await.unwrap();
        assert_eq!(res.status(), StatusCode::Ok);
        let prometheus = res.body_string().await.unwrap();
        let lines = prometheus.lines().collect::<Vec<_>>();
        assert!(
            lines.contains(&"consensus_outstanding_transactions 1"),
            "Missing consensus_outstanding_transactions in metrics:\n{}",
            prometheus
        );
        assert!(
            lines.contains(
                &format!(
                    "consensus_outstanding_transactions_memory_size {}",
                    txn_size
                )
                .as_str()
            ),
            "Missing consensus_outstanding_transactions_memory_size in metrics:\n{}",
            prometheus
        );

        // Start the validators and wait for the block to be finalized.
        network.start().await;
        while client
            .get::<MempoolQueryData>("mempool-info")
            .send()
            .await
            .unwrap()
            .transaction_count
            > 0
        {
            tracing::info!("waiting for transaction to be finalized");
            sleep(Duration::from_secs(1)).await;
        }

        // Check updated block height. There can be a brief delay between the mempool statistics
        // being updated and the decide event being published. Retry this a few times until it
        // succeeds.
        while client.get::<u64>("block-height").send().await.unwrap() == 1 {
            tracing::info!("waiting for block height to update");
            sleep(Duration::from_secs(1)).await;
        }
        let success_rate = client.get::<f64>("success-rate").send().await.unwrap();
        // If metrics are populating correctly, we should get a finite number. If not, we might get
        // NaN or infinity due to division by 0.
        assert!(success_rate.is_finite(), "{success_rate}");
        // We know at least some views have been successful, since we finalized a block.
        assert!(success_rate > 0.0, "{success_rate}");

        network.shut_down().await;
    }

    #[async_std::test]
    async fn test_extensions() {
        setup_test();

        let dir = TempDir::with_prefix("test_status_extensions").unwrap();
        let data_source = ExtensibleDataSource::new(
            MockDataSource::create(dir.path(), Default::default())
                .await
                .unwrap(),
            0,
        );

        let extensions = toml! {
            [route.post_ext]
            PATH = ["/ext/:val"]
            METHOD = "POST"
            ":val" = "Integer"

            [route.get_ext]
            PATH = ["/ext"]
            METHOD = "GET"
        };

        let mut api = define_api::<RwLock<ExtensibleDataSource<MockDataSource, u64>>, Version01>(
            &Options {
                extensions: vec![extensions.into()],
                ..Default::default()
            },
            STATIC_VER_0_1,
        )
        .unwrap();
        api.get("get_ext", |_, state| {
            async move { Ok(*state.as_ref()) }.boxed()
        })
        .unwrap()
        .post("post_ext", |req, state| {
            async move {
                *state.as_mut() = req.integer_param("val")?;
                Ok(())
            }
            .boxed()
        })
        .unwrap();

        let mut app = App::<_, Error>::with_state(RwLock::new(data_source));
        app.register_module("status", api).unwrap();

        let port = pick_unused_port().unwrap();
        let _server = BackgroundTask::spawn(
            "server",
            app.serve(format!("0.0.0.0:{}", port), STATIC_VER_0_1),
        );

        let client = Client::<Error, Version01>::new(
            format!("http://localhost:{}/status", port).parse().unwrap(),
        );
        assert!(client.connect(Some(Duration::from_secs(60))).await);

        assert_eq!(client.get::<u64>("ext").send().await.unwrap(), 0);
        client.post::<()>("ext/42").send().await.unwrap();
        assert_eq!(client.get::<u64>("ext").send().await.unwrap(), 42);

        // Ensure we can still access the built-in functionality.
        assert_eq!(client.get::<u64>("block-height").send().await.unwrap(), 0);
    }
}
