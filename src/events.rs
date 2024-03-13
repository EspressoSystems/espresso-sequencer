use clap::Args;
use derive_more::From;
use futures::{FutureExt, StreamExt, TryFutureExt};
use hotshot_types::traits::{node_implementation::NodeType, signature_key::SignatureKey};
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::{fmt::Display, path::PathBuf};
use tagged_base64::TaggedBase64;
use tide_disco::{api::ApiError, method::ReadState, Api, RequestError, StatusCode};

use crate::{api::load_api, events_source::EventsSource};

#[derive(Args, Default)]
pub struct Options {
    #[arg(
        long = "hotshot-events-service-api-path",
        env = "HOTSHOT_EVENTS_SERVICE_API_PATH"
    )]
    pub api_path: Option<PathBuf>,

    /// Additional API specification files to merge with `hotshot-events-service-api-path`.
    ///
    /// These optional files may contain route definitions for application-specific routes that have
    /// been added as extensions to the basic hotshot-events-service API.
    #[arg(
        long = "hotshot-events-extension",
        env = "HOTSHOT_EVENTS_SERVICE_EXTENSIONS",
        value_delimiter = ','
    )]
    pub extensions: Vec<toml::Value>,
}

#[derive(Clone, Debug, Snafu, Deserialize, Serialize)]
#[snafu(visibility(pub))]
pub enum EventError {
    /// The requested resource does not exist or is not known to this hotshot node.
    NotFound,
    /// The requested resource exists but is not currently available.
    Missing,
    /// There was an error while trying to fetch the requested resource.
    #[snafu(display("Failed to fetch requested resource: {message}"))]
    Error { message: String },
}

#[derive(Clone, Debug, From, Snafu, Deserialize, Serialize)]
#[snafu(visibility(pub))]
pub enum Error {
    Request {
        source: RequestError,
    },
    #[snafu(display("error receiving events {resource}: {source}"))]
    #[from(ignore)]
    EventAvailable {
        source: EventError,
        resource: String,
    },
    Custom {
        message: String,
        status: StatusCode,
    },
}

impl tide_disco::error::Error for Error {
    fn catch_all(status: StatusCode, msg: String) -> Self {
        Error::Custom {
            message: msg,
            status,
        }
    }
    fn status(&self) -> StatusCode {
        match self {
            Error::Request { .. } => StatusCode::BadRequest,
            Error::EventAvailable { source, .. } => match source {
                EventError::NotFound => StatusCode::NotFound,
                EventError::Missing => StatusCode::NotFound,
                EventError::Error { .. } => StatusCode::InternalServerError,
            },
            Error::Custom { .. } => StatusCode::InternalServerError,
        }
    }
}

pub fn define_api<State, Types: NodeType>(options: &Options) -> Result<Api<State, Error>, ApiError>
where
    State: 'static + Send + Sync + ReadState,
    <State as ReadState>::State: Send + Sync + EventsSource<Types>,
    Types: NodeType,
    <<Types as NodeType>::SignatureKey as SignatureKey>::PureAssembledSignatureType:
        for<'a> TryFrom<&'a TaggedBase64> + Into<TaggedBase64> + Display,
    for<'a> <<<Types as NodeType>::SignatureKey as SignatureKey>::PureAssembledSignatureType as TryFrom<
        &'a TaggedBase64,
    >>::Error: Display,
{
    let mut api = load_api::<State, Error>(
        options.api_path.as_ref(),
        include_str!("../api/hotshot_events.toml"),
        options.extensions.clone(),
    )?;
    api.with_version("0.1.0".parse().unwrap())
        .stream("hotshot_events", move |req, state| {
            async move {
                let view_number = req.integer_param("view_number")?;
                Ok(state
                    .read(|state| {
                        async move {
                            state
                                .get_available_hotshot_events(view_number)
                                .await
                                .map(Ok)
                        }
                        .boxed()
                    })
                    .await)
            }
            .try_flatten_stream()
            .boxed()
        })?;

    Ok(api)
}
