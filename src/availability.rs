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
use hotshot_types::traits::node_implementation::NodeTypes;
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
    #[arg(long = "availability-api-path", env = "HOTSHOT_AVAILABILITY_API_PATH")]
    pub api_path: Option<PathBuf>,

    /// Additional API specification files to merge with `availability-api-path`.
    ///
    /// These optional files may contain route definitions for application-specific routes that have
    /// been added as extensions to the basic availability API.
    #[arg(
        long = "availability-extension",
        env = "HOTSHOT_AVAILABILITY_EXTENSIONS",
        value_delimiter = ','
    )]
    pub extensions: Vec<PathBuf>,
}

#[derive(Clone, Debug, From, Snafu, Deserialize, Serialize)]
pub enum Error {
    Request { source: RequestError },
    Custom { message: String, status: StatusCode },
}

impl Error {
    pub fn internal<M: Display>(message: M) -> Self {
        Self::Custom {
            message: message.to_string(),
            status: StatusCode::InternalServerError,
        }
    }

    pub fn status(&self) -> StatusCode {
        match self {
            Self::Request { .. } => StatusCode::BadRequest,
            Self::Custom { status, .. } => *status,
        }
    }
}

pub fn define_api<State, Types: NodeTypes>(options: &Options) -> Result<Api<State, Error>, ApiError>
where
    State: 'static + Send + Sync + ReadState,
    <State as ReadState>::State: Send + Sync + AvailabilityDataSource<Types>,
{
    let mut api = load_api(
        options.api_path.as_ref(),
        include_str!("../api/availability.toml"),
        &options.extensions,
    )?;
    api.with_version("0.0.1".parse().unwrap());
    Ok(api)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{data_source::QueryData, testing::mocks::MockTypes, Error};
    use async_std::{sync::RwLock, task::spawn};
    use futures::FutureExt;
    use portpicker::pick_unused_port;
    use std::fs;
    use std::time::Duration;
    use surf_disco::Client;
    use tempdir::TempDir;
    use tide_disco::App;
    use toml::toml;

    #[test]
    fn instantiate_api() {
        define_api::<RwLock<QueryData<MockTypes, ()>>, MockTypes>(&Default::default()).unwrap();
    }

    #[async_std::test]
    async fn test_extensions() {
        let dir = TempDir::new("test_availability_extensions").unwrap();
        let query_data = QueryData::<MockTypes, u64>::create(dir.path(), 0).unwrap();

        // Create the API extensions specification.
        let extensions_path = dir.path().join("extensions.toml");
        let extensions = toml! {
            [route.post_ext]
            PATH = ["/ext/:val"]
            METHOD = "POST"
            ":val" = "Integer"

            [route.get_ext]
            PATH = ["/ext"]
            METHOD = "GET"
        };
        fs::write(&extensions_path, extensions.to_string().as_bytes()).unwrap();

        let mut api = define_api::<RwLock<QueryData<MockTypes, u64>>, MockTypes>(&Options {
            extensions: vec![extensions_path],
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
        app.register_module("availability", api).unwrap();

        let port = pick_unused_port().unwrap();
        spawn(app.serve(format!("0.0.0.0:{}", port)));

        let client = Client::<Error>::new(
            format!("http://localhost:{}/availability", port)
                .parse()
                .unwrap(),
        );
        assert!(client.connect(Some(Duration::from_secs(60))).await);

        assert_eq!(client.get::<u64>("ext").send().await.unwrap(), 0);
        client.post::<()>("ext/42").send().await.unwrap();
        assert_eq!(client.get::<u64>("ext").send().await.unwrap(), 42);
    }
}
