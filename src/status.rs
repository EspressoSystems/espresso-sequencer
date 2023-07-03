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

use crate::api::load_api;
use clap::Args;
use derive_more::From;
use futures::FutureExt;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::fmt::Display;
use std::path::PathBuf;
use tide_disco::{api::ApiError, method::ReadState, Api, RequestError, StatusCode};

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

pub fn define_api<State>(options: &Options) -> Result<Api<State, Error>, ApiError>
where
    State: 'static + Send + Sync + ReadState,
    <State as ReadState>::State: Send + Sync + StatusDataSource,
{
    let mut api = load_api::<State, Error>(
        options.api_path.as_ref(),
        include_str!("../api/status.toml"),
        options.extensions.clone(),
    )?;
    api.with_version("0.0.1".parse().unwrap())
        .get("latest_block_height", |_, state| {
            async { state.block_height().map_err(internal) }.boxed()
        })?
        .get("mempool_info", |_, state| {
            async { state.mempool_info().map_err(internal) }.boxed()
        })?
        .get("success_rate", |_, state| {
            async { state.success_rate().map_err(internal) }.boxed()
        })?
        .get("metrics", |_, state| {
            async { state.export_metrics().map_err(internal) }.boxed()
        })?;
    Ok(api)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        data_source::QueryData,
        testing::{
            consensus::MockNetwork,
            mocks::{MockNodeImpl, MockTransaction, MockTypes},
            setup_test, sleep,
        },
        Error,
    };
    use async_std::{sync::RwLock, task::spawn};
    use bincode::Options as _;
    use futures::FutureExt;
    use hotshot_utils::bincode::bincode_opts;
    use portpicker::pick_unused_port;
    use std::time::Duration;
    use surf_disco::Client;
    use tempdir::TempDir;
    use tide_disco::App;
    use toml::toml;

    #[async_std::test]
    async fn test_api() {
        setup_test();

        // Create the consensus network.
        let network = MockNetwork::init(()).await;
        let hotshot = network.handle();

        // Start the web server.
        let port = pick_unused_port().unwrap();
        let mut app = App::<_, Error>::with_state(network.query_data());
        app.register_module("status", define_api(&Default::default()).unwrap())
            .unwrap();
        spawn(app.serve(format!("0.0.0.0:{}", port)));

        // Start a client.
        let client =
            Client::<Error>::new(format!("http://localhost:{}/status", port).parse().unwrap());
        assert!(client.connect(Some(Duration::from_secs(60))).await);

        // Submit a transaction. We have not yet started the validators, so this transaction will
        // stay in the mempool, allowing us to check the mempool endpoint.
        let txn = MockTransaction { nonce: 0 };
        let txn_size = bincode_opts().serialized_size(&txn).unwrap();
        hotshot.submit_transaction(txn.clone()).await.unwrap();
        loop {
            let mempool = client
                .get::<MempoolQueryData>("mempool_info")
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
        assert_eq!(
            client
                .get::<u64>("latest_block_height")
                .send()
                .await
                .unwrap(),
            0
        );

        // Test Prometheus export.
        let prometheus = client.get::<String>("metrics").send().await.unwrap();
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
            .get::<MempoolQueryData>("mempool_info")
            .send()
            .await
            .unwrap()
            .transaction_count
            > 0
        {
            tracing::info!("waiting for transaction to be finalized");
            sleep(Duration::from_secs(1)).await;
        }
        assert!(
            client
                .get::<u64>("latest_block_height")
                .send()
                .await
                .unwrap()
                > 0
        );
        assert!(client.get::<f64>("success_rate").send().await.unwrap() > 0.0);

        network.shut_down().await;
    }

    #[async_std::test]
    async fn test_extensions() {
        setup_test();

        let dir = TempDir::new("test_status_extensions").unwrap();
        let query_data = QueryData::<MockTypes, MockNodeImpl, u64>::create(dir.path(), 0).unwrap();

        let extensions = toml! {
            [route.post_ext]
            PATH = ["/ext/:val"]
            METHOD = "POST"
            ":val" = "Integer"

            [route.get_ext]
            PATH = ["/ext"]
            METHOD = "GET"
        };

        let mut api = define_api::<RwLock<QueryData<MockTypes, MockNodeImpl, u64>>>(&Options {
            extensions: vec![extensions],
            ..Default::default()
        })
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

        let mut app = App::<_, Error>::with_state(RwLock::new(query_data));
        app.register_module("status", api).unwrap();

        let port = pick_unused_port().unwrap();
        spawn(app.serve(format!("0.0.0.0:{}", port)));

        let client =
            Client::<Error>::new(format!("http://localhost:{}/status", port).parse().unwrap());
        assert!(client.connect(Some(Duration::from_secs(60))).await);

        assert_eq!(client.get::<u64>("ext").send().await.unwrap(), 0);
        client.post::<()>("ext/42").send().await.unwrap();
        assert_eq!(client.get::<u64>("ext").send().await.unwrap(), 42);

        // Ensure we can still access the built-in functionality.
        assert_eq!(
            client
                .get::<u64>("latest_block_height")
                .send()
                .await
                .unwrap(),
            0
        );
    }
}
