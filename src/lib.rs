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

//! The HotShot Query Service is a minimal, generic query service that can be integrated into any
//! decentralized application running on the [hotshot] consensus layer. It provides all the features
//! that HotShot itself expects of a query service (such as providing consensus-related data for
//! catchup and synchronization) as well as some application-level features that deal only with
//! consensus-related or application-agnostic data. In addition, the query service is provided as an
//! extensible library, which makes it easy to add additional, application-specific features.
//!
//! # Basic usage
//!
//! ```
//! # use hotshot::types::SystemContextHandle;
//! # use hotshot_query_service::testing::mocks::{
//! #   MockNodeImpl as AppNodeImpl, MockTypes as AppTypes,
//! # };
//! # use std::path::Path;
//! # async fn doc(storage_path: &std::path::Path) -> Result<(), hotshot_query_service::Error> {
//! use hotshot_query_service::{
//!     availability,
//!     data_source::{FileSystemDataSource, UpdateDataSource, VersionedDataSource},
//!     status::UpdateStatusData,
//!     status, Error
//! };
//!
//! use async_std::{sync::{Arc, RwLock}, task::spawn};
//! use futures::StreamExt;
//! use hotshot::SystemContext;
//! use tide_disco::App;
//!
//! // Create or open a data source.
//! let data_source = FileSystemDataSource::<AppTypes, AppNodeImpl>::create(storage_path)
//!     .map_err(Error::internal)?;
//!
//! // Create hotshot, giving it a handle to the status metrics.
//! let (mut hotshot, _) = SystemContext::<AppTypes, AppNodeImpl>::init(
//! #   panic!(), panic!(), panic!(), panic!(), panic!(), panic!(), panic!(),
//!     data_source.populate_metrics(),
//!     // Other fields omitted
//! ).await.map_err(Error::internal)?;
//!
//! // Create API modules.
//! let availability_api = availability::define_api(&Default::default())
//!     .map_err(Error::internal)?;
//! let status_api = status::define_api(&Default::default())
//!     .map_err(Error::internal)?;
//!
//! // Create app. We wrap `data_source` into an `RwLock` so we can share it with the web server.
//! let data_source = Arc::new(RwLock::new(data_source));
//! let mut app = App::<_, Error>::with_state(data_source.clone());
//! app
//!     .register_module("availability", availability_api)
//!     .map_err(Error::internal)?
//!     .register_module("status", status_api)
//!     .map_err(Error::internal)?;
//!
//! // Serve app.
//! spawn(app.serve("0.0.0.0:8080"));
//!
//! // Update query data using HotShot events.
//! let mut events = hotshot.get_event_stream(Default::default()).await.0;
//! while let Some(event) = events.next().await {
//!     // Re-lock the mutex each time we get a new event.
//!     let mut data_source = data_source.write().await;
//!
//!     // Update the query data based on this event.
//!     data_source.update(&event);
//!     data_source.commit().await.map_err(Error::internal)?;
//! }
//! # Ok(())
//! # }
//! ```
//!
//! Shortcut for starting an out-of-the-box service with no extensions (does exactly the above and
//! nothing more):
//!
//! ```
//! # use async_std::task::spawn;
//! # use hotshot::types::SystemContextHandle;
//! # use hotshot_query_service::{data_source::FileSystemDataSource, Error, Options};
//! # use hotshot_query_service::testing::mocks::{MockTypes, MockNodeImpl};
//! # use std::path::Path;
//! # fn doc(storage_path: &Path, options: &Options, hotshot: SystemContextHandle<MockTypes, MockNodeImpl>) -> Result<(), Error> {
//! use hotshot_query_service::run_standalone_service;
//!
//! let data_source = FileSystemDataSource::create(storage_path).map_err(Error::internal)?;
//! spawn(run_standalone_service(options, data_source, hotshot));
//! # Ok(())
//! # }
//! ```
//!
//! # Persistence
//!
//! Naturally, an archival query service such as this is heavily dependent on a persistent storage
//! implementation. The APIs provided by this query service are generic over the specific type of
//! the persistence layer, which we call a _data source_. This crate provides several data source
//! implementations in the [`data_source`] module.
//!
//! # Interaction with other components
//!
//! While the HotShot Query Service [can be used as a standalone service](run_standalone_service),
//! it is designed to be used as a single component of a larger service consisting of several other
//! interacting components. This interaction has two dimensions:
//! * _extension_, adding new functionality to the API modules provided by this crate
//! * _composition_, combining the API modules from this crate with other, application-specific API
//!   modules to create a single [tide_disco] API
//!
//! ## Extension
//!
//! It is possible to add new functionality directly to the modules provided by this create. This
//! allows you to keep semantically related functionality grouped together in a single API module,
//! for interface purposes, even while some of the functionality of that module is provided by this
//! crate and some of it is an application-specific extension.
//!
//! For example, consider an application which is a UTXO-based blockchain. Each transaction consists
//! of a handful of new _output records_, and you want your query service to provide an API for
//! looking up a specific output by its index. Semantically, this functionality belongs in the
//! _data availability_ API, however it is application-specific -- HotShot itself makes no
//! assumptions and provides no guarantees about the internal structure of a transaction. In order
//! to expose this UTXO-specific functionality as well as the generic data availability
//! functionality provided by this crate as part of the same public API, you can extend the
//! [availability] module of this crate with additional data structures and endpoints that know
//! about the internal structure of your transactions.
//!
//! There are two parts to adding additional functionality to a module in this crate: adding the
//! required additional data structures to the data source, and creating a new API endpoint to
//! expose the functionality. The mechanism for the former will depend on the specific data source
//! you are using. Check the documentation for your data source implementation to see how it can be
//! extended.
//!
//! For the latter, you can modify the default availablity API with the addition of a new endpoint
//! that accesses the custom state you have added to the data source. It is good practice to define
//! a trait for accessing this custom state, so that if you want to switch data sources in the
//! future, you can easily extend the new data source, implement the trait, and then transparently
//! replace the data source that you use to set up your API. In the case of
//! adding a UTXO index, this trait might look like this:
//!
//! ```
//! # use hotshot_query_service::{
//! #   availability::{AvailabilityDataSource, TransactionIndex},
//! #   testing::mocks::{MockNodeImpl as AppNodeImpl, MockTypes as AppTypes},
//! # };
//! use async_trait::async_trait;
//!
//! #[async_trait]
//! trait UtxoDataSource: AvailabilityDataSource<AppTypes, AppNodeImpl> {
//!     // Index mapping UTXO index to (block index, transaction index, output index)
//!     async fn find_utxo(&self, utxo: u64) -> Option<(usize, TransactionIndex<AppTypes>, usize)>;
//! }
//! ```
//!
//! Implement this trait for the extended data source you're using, and then add a new endpoint to
//! the availability API like so:
//!
//! ```
//! # use async_std::sync::RwLock;
//! # use async_trait::async_trait;
//! # use futures::FutureExt;
//! # use hotshot_query_service::availability::{
//! #   self, AvailabilityDataSource, QueryBlockSnafu, TransactionIndex,
//! # };
//! # use hotshot_query_service::testing::mocks::{
//! #   MockNodeImpl as AppNodeImpl, MockTypes as AppTypes,
//! # };
//! # use hotshot_query_service::Error;
//! # use snafu::ResultExt;
//! # use tide_disco::{api::ApiError, method::ReadState, Api, App, StatusCode};
//! # #[async_trait]
//! # trait UtxoDataSource: AvailabilityDataSource<AppTypes, AppNodeImpl> {
//! #   async fn find_utxo(&self, utxo: u64) -> Option<(usize, TransactionIndex<AppTypes>, usize)>;
//! # }
//!
//! fn define_app_specific_availability_api<State>(
//!     options: &availability::Options,
//! ) -> Result<Api<State, availability::Error>, ApiError>
//! where
//!     State: 'static + Send + Sync + ReadState,
//!     <State as ReadState>::State: UtxoDataSource + Send + Sync,
//! {
//!     let mut api = availability::define_api(options)?;
//!     api.get("get_utxo", |req, state: &<State as ReadState>::State| async move {
//!         let utxo_index = req.integer_param("index")?;
//!         let (block_index, txn_index, output_index) = state
//!             .find_utxo(utxo_index)
//!             .await
//!             .ok_or_else(|| availability::Error::Custom {
//!                 message: format!("no such UTXO {}", utxo_index),
//!                 status: StatusCode::NotFound,
//!             })?;
//!         let block = state
//!             .get_block(block_index)
//!             .await
//!             .context(QueryBlockSnafu { resource: block_index.to_string() })?;
//!         let txn = block.transaction(&txn_index).unwrap();
//!         let utxo = // Application-specific logic to extract a UTXO from a transaction.
//! #           todo!();
//!         Ok(utxo)
//!     }.boxed())?;
//!     Ok(api)
//! }
//!
//! fn init_server<D: UtxoDataSource + Send + Sync + 'static>(
//!     options: &availability::Options,
//!     data_source: D,
//! ) -> Result<App<RwLock<D>, Error>, availability::Error> {
//!     let api = define_app_specific_availability_api(options)
//!         .map_err(availability::Error::internal)?;
//!     let mut app = App::with_state(RwLock::new(data_source));
//!     app.register_module("availability", api).map_err(availability::Error::internal)?;
//!     Ok(app)
//! }
//! ```
//!
//! Now you need to define the new route, `get_utxo`, in your API specification. Create a file
//! `app_specific_availability.toml`:
//!
//! ```toml
//! [route.get_utxo]
//! PATH = ["utxo/:index"]
//! ":index" = "Integer"
//! DOC = "Get a UTXO by its index"
//! ```
//!
//! and make sure `options.extensions` includes `"app_specific_availability.toml"`.
//!
//! ## Composition
//!
//! Composing the modules provided by this crate with other, unrelated modules to create a unified
//! service is fairly simple, as most of the complexity is handled by [tide_disco], which already
//! provides a mechanism for composing several modules into a single application. In principle, all
//! you need to do is register the [availability] and [status] APIs provided by this crate with a
//! [tide_disco::App], and then register your own API modules with the same app.
//!
//! The one wrinkle is that all modules within a [tide_disco] app must share the same state type. It
//! is for this reason that the modules provided by this crate are generic on the state type -- both
//! [availability::define_api] and [status::define_api] can work with any state type, provided that
//! type implements the corresponding data source traits. The data sources provided by this crate
//! implement both of these traits, but if you want to use a custom state type that includes state
//! for other modules, you will need to implement these traits for your custom type. The basic
//! pattern looks like this:
//!
//! ```
//! # use async_trait::async_trait;
//! # use hotshot_types::traits::signature_key::EncodedPublicKey;
//! # use hotshot_query_service::QueryResult;
//! # use hotshot_query_service::availability::{
//! #   AvailabilityDataSource, BlockId, BlockQueryData, LeafId, LeafQueryData, TransactionHash,
//! #   TransactionIndex,
//! # };
//! # use hotshot_query_service::metrics::PrometheusMetrics;
//! # use hotshot_query_service::status::StatusDataSource;
//! # use hotshot_query_service::testing::mocks::{
//! #   MockNodeImpl as AppNodeImpl, MockTypes as AppTypes,
//! # };
//! # use std::ops::RangeBounds;
//! # type AppQueryData = ();
//! // Our AppState takes an underlying data source `D` which already implements the relevant
//! // traits, and adds some state for use with other modules.
//! struct AppState<D> {
//!     hotshot_qs: D,
//!     // additional state for other modules
//! }
//!
//! // Implement data source trait for availability API by delegating to the underlying data source.
//! #[async_trait]
//! impl<D: AvailabilityDataSource<AppTypes, AppNodeImpl> + Send + Sync>
//!     AvailabilityDataSource<AppTypes, AppNodeImpl> for AppState<D>
//! {
//!     type LeafRange<'a, R> = D::LeafRange<'a, R>
//!     where
//!         Self: 'a,
//!         R: RangeBounds<usize> + Send;
//!     type BlockRange<'a, R> = D::BlockRange<'a, R>
//!     where
//!         Self: 'a,
//!         R: RangeBounds<usize> + Send;
//!
//!     type LeafStream = D::LeafStream;
//!     type BlockStream = D::BlockStream;
//!
//!     async fn get_leaf<ID>(&self, id: ID) -> QueryResult<LeafQueryData<AppTypes, AppNodeImpl>>
//!     where
//!         ID: Into<LeafId<AppTypes, AppNodeImpl>> + Send + Sync,
//!     {
//!         self.hotshot_qs.get_leaf(id).await
//!     }
//!
//!     // etc
//! #   async fn get_block<ID>(&self, id: ID) -> QueryResult<BlockQueryData<AppTypes>>
//! #   where
//! #       ID: Into<BlockId<AppTypes>> + Send + Sync { todo!() }
//! #   async fn get_block_with_transaction(&self, hash: TransactionHash<AppTypes>) -> QueryResult<(BlockQueryData<AppTypes>, TransactionIndex<AppTypes>)> { todo!() }
//! #   async fn get_leaf_range<R>(&self, range: R) -> QueryResult<Self::LeafRange<'_, R>>
//! #   where
//! #       R: RangeBounds<usize> + Send { todo!() }
//! #   async fn get_block_range<R>(&self, range: R) -> QueryResult<Self::BlockRange<'_, R>>
//! #   where
//! #       R: RangeBounds<usize> + Send { todo!() }
//! #   async fn get_proposals(&self, id: &EncodedPublicKey, limit: Option<usize>) -> QueryResult<Vec<LeafQueryData<AppTypes, AppNodeImpl>>> { todo!() }
//! #   async fn count_proposals(&self, id: &EncodedPublicKey) -> QueryResult<usize> { todo!() }
//! #   async fn subscribe_leaves(&self, height: usize) -> QueryResult<Self::LeafStream> { todo!() }
//! #   async fn subscribe_blocks(&self, height: usize) -> QueryResult<Self::BlockStream> { todo!() }
//! }
//!
//! // Implement data source trait for status API by delegating to the underlying data source.
//! #[async_trait]
//! impl<D: StatusDataSource + Send + Sync> StatusDataSource for AppState<D> {
//!     async fn block_height(&self) -> QueryResult<usize> {
//!         self.hotshot_qs.block_height().await
//!     }
//!
//!     fn metrics(&self) -> &PrometheusMetrics {
//!         self.hotshot_qs.metrics()
//!     }
//! }
//!
//! // Implement data source traits for other modules, using additional state from AppState.
//! ```
//!
//! In the future, we may provide derive macros for
//! [AvailabilityDataSource](availability::AvailabilityDataSource) and
//! [StatusDataSource](status::StatusDataSource) to eliminate the boilerplate of implementing them
//! for a custom type that has an existing implementation as one of its fields.
//!
//! Once you have created your `AppState` type aggregating the state for each API module, you can
//! initialize the state as normal, instantiating `D` with a concrete implementation of a data
//! source and initializing `hotshot_qs` as you normally would that data source.
//!
//! _However_, this only works if you want the persistent storage for the availability and status
//! modules (managed by `hotshot_qs`) to be independent of the persistent storage for other modules.
//! You may well want to synchronize the storage for all modules together, so that updates to the
//! entire application state can be done atomically. This is particularly relevant if one of your
//! application-specific modules updates its storage based on a stream of HotShot leaves. Since the
//! availability module also updates with each new leaf, you probably want these two modules to stay
//! in sync. The data source implementations provided by this crate provide means by which you can
//! add additional data to the same persistent store and synchronize the entire store together.
//! Refer to the documentation for you specific data source for information on how to achieve this.
//!

mod api;
pub mod availability;
pub mod data_source;
mod error;
pub mod metrics;
mod resolvable;
pub mod status;
pub mod testing;

pub use availability::QueryableBlock;
pub use error::Error;
pub use resolvable::Resolvable;

use futures::Future;
use hotshot::{certificate, types::SystemContextHandle};
use hotshot_types::{
    data::LeafType,
    traits::{
        block_contents,
        node_implementation::{NodeImplementation, NodeType},
    },
};
use serde::{Deserialize, Serialize};
use snafu::Snafu;

/// Leaf type appended to a chain by consensus.
pub type Leaf<Types, I> = <I as NodeImplementation<Types>>::Leaf;
/// Certificate justifying a [`Leaf`].
pub type QuorumCertificate<Types, I> = certificate::QuorumCertificate<Types, Leaf<Types, I>>;
/// State change indicated by a [`Leaf`].
pub type Deltas<Types, I> = <Leaf<Types, I> as LeafType>::DeltasType;
/// Block of data appened to a chain by consensus.
pub type Block<Types> = <Types as NodeType>::BlockType;
/// Item within a [`Block`].
pub type Transaction<Types> = <Block<Types> as block_contents::Block>::Transaction;

#[derive(Clone, Debug, Snafu, Deserialize, Serialize)]
#[snafu(visibility(pub))]
pub enum QueryError {
    /// The requested resource does not exist or is not known to this query service.
    NotFound,
    /// The requested resource exists but is not currently available.
    ///
    /// In most cases a missing resource can be recovered from DA.
    Missing,
    /// There was an error while trying to fetch the requested resource.
    #[snafu(display("Failed to fetch requested resource: {message}"))]
    Error { message: String },
}

pub type QueryResult<T> = Result<T, QueryError>;

#[derive(clap::Args, Default)]
pub struct Options {
    #[clap(flatten)]
    pub availability: availability::Options,
    #[clap(flatten)]
    pub status: status::Options,
}

/// Run an instance of the HotShot Query service with no customization.
pub fn run_standalone_service<Types: NodeType, I: NodeImplementation<Types>, D>(
    _options: &Options,
    _data_source: D,
    _hotshot: SystemContextHandle<Types, I>,
) -> impl Future<Output = ()> + Send + Sync + 'static
where
    Block<Types>: QueryableBlock,
    D: availability::AvailabilityDataSource<Types, I>
        + status::StatusDataSource
        + data_source::UpdateDataSource<Types, I>,
{
    async move { unimplemented!() }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        availability::{
            AvailabilityDataSource, BlockId, BlockQueryData, LeafId, LeafQueryData,
            TransactionHash, TransactionIndex,
        },
        metrics::PrometheusMetrics,
        status::StatusDataSource,
        testing::{
            consensus::MockDataSource,
            mocks::{MockNodeImpl, MockTypes},
        },
    };
    use async_std::{sync::RwLock, task::spawn};
    use async_trait::async_trait;
    use atomic_store::{load_store::BincodeLoadStore, AtomicStore, AtomicStoreLoader, RollingLog};
    use futures::FutureExt;
    use hotshot::types::SignatureKey;
    use hotshot_signature_key::bn254::BN254Pub;
    use hotshot_types::traits::signature_key::EncodedPublicKey;
    use portpicker::pick_unused_port;
    use std::ops::RangeBounds;
    use std::time::Duration;
    use surf_disco::Client;
    use tempdir::TempDir;
    use tide_disco::App;
    use toml::toml;

    struct CompositeState {
        store: AtomicStore,
        hotshot_qs: MockDataSource,
        module_state: RollingLog<BincodeLoadStore<u64>>,
    }

    #[async_trait]
    impl AvailabilityDataSource<MockTypes, MockNodeImpl> for CompositeState {
        type LeafStream =
            <MockDataSource as AvailabilityDataSource<MockTypes, MockNodeImpl>>::LeafStream;
        type BlockStream =
            <MockDataSource as AvailabilityDataSource<MockTypes, MockNodeImpl>>::BlockStream;

        type LeafRange<'a, R> =
            <MockDataSource as AvailabilityDataSource<
                MockTypes,
                MockNodeImpl,
            >>::LeafRange<'a, R>
        where
            Self: 'a,
            R: RangeBounds<usize> + Send;
        type BlockRange<'a, R> =
            <MockDataSource as AvailabilityDataSource<
                MockTypes,
                MockNodeImpl,
            >>::BlockRange<'a, R>
        where
            Self: 'a,
            R: RangeBounds<usize> + Send;

        async fn get_leaf<ID>(&self, id: ID) -> QueryResult<LeafQueryData<MockTypes, MockNodeImpl>>
        where
            ID: Into<LeafId<MockTypes, MockNodeImpl>> + Send + Sync,
        {
            self.hotshot_qs.get_leaf(id).await
        }
        async fn get_block<ID>(&self, id: ID) -> QueryResult<BlockQueryData<MockTypes>>
        where
            ID: Into<BlockId<MockTypes>> + Send + Sync,
        {
            self.hotshot_qs.get_block(id).await
        }
        async fn get_leaf_range<R>(&self, range: R) -> QueryResult<Self::LeafRange<'_, R>>
        where
            R: RangeBounds<usize> + Send,
        {
            self.hotshot_qs.get_leaf_range(range).await
        }
        async fn get_block_range<R>(&self, range: R) -> QueryResult<Self::BlockRange<'_, R>>
        where
            R: RangeBounds<usize> + Send,
        {
            self.hotshot_qs.get_block_range(range).await
        }
        async fn get_block_with_transaction(
            &self,
            hash: TransactionHash<MockTypes>,
        ) -> QueryResult<(BlockQueryData<MockTypes>, TransactionIndex<MockTypes>)> {
            self.hotshot_qs.get_block_with_transaction(hash).await
        }
        async fn get_proposals(
            &self,
            proposer: &EncodedPublicKey,
            limit: Option<usize>,
        ) -> QueryResult<Vec<LeafQueryData<MockTypes, MockNodeImpl>>> {
            self.hotshot_qs.get_proposals(proposer, limit).await
        }
        async fn count_proposals(&self, proposer: &EncodedPublicKey) -> QueryResult<usize> {
            self.hotshot_qs.count_proposals(proposer).await
        }
        async fn subscribe_leaves(&self, height: usize) -> QueryResult<Self::LeafStream> {
            self.hotshot_qs.subscribe_leaves(height).await
        }
        async fn subscribe_blocks(&self, height: usize) -> QueryResult<Self::BlockStream> {
            self.hotshot_qs.subscribe_blocks(height).await
        }
    }

    // Implement data source trait for status API.
    #[async_trait]
    impl StatusDataSource for CompositeState {
        async fn block_height(&self) -> QueryResult<usize> {
            self.hotshot_qs.block_height().await
        }

        fn metrics(&self) -> &PrometheusMetrics {
            self.hotshot_qs.metrics()
        }
    }

    #[async_std::test]
    async fn test_composition() {
        let dir = TempDir::new("test_composition").unwrap();
        let mut loader = AtomicStoreLoader::create(dir.path(), "test_composition").unwrap();
        let hotshot_qs = MockDataSource::create_with_store(&mut loader).unwrap();
        let module_state =
            RollingLog::create(&mut loader, Default::default(), "module_state", 1024).unwrap();
        let state = CompositeState {
            hotshot_qs,
            module_state,
            store: AtomicStore::open(loader).unwrap(),
        };

        let module_spec = toml! {
            [route.post_ext]
            PATH = ["/ext/:val"]
            METHOD = "POST"
            ":val" = "Integer"

            [route.get_ext]
            PATH = ["/ext"]
            METHOD = "GET"
        };

        let mut app = App::<_, Error>::with_state(RwLock::new(state));
        app.register_module(
            "availability",
            availability::define_api(&Default::default()).unwrap(),
        )
        .unwrap()
        .register_module("status", status::define_api(&Default::default()).unwrap())
        .unwrap()
        .module::<Error>("mod", module_spec)
        .unwrap()
        .get("get_ext", |_, state| {
            async move { state.module_state.load_latest().map_err(Error::internal) }.boxed()
        })
        .unwrap()
        .post("post_ext", |req, state| {
            async move {
                state
                    .module_state
                    .store_resource(&req.integer_param("val").map_err(Error::internal)?)
                    .map_err(Error::internal)?;
                state
                    .module_state
                    .commit_version()
                    .map_err(Error::internal)?;
                state.hotshot_qs.skip_version().map_err(Error::internal)?;
                state.store.commit_version().map_err(Error::internal)
            }
            .boxed()
        })
        .unwrap();

        let port = pick_unused_port().unwrap();
        spawn(app.serve(format!("0.0.0.0:{}", port)));

        let client = Client::<Error>::new(format!("http://localhost:{}", port).parse().unwrap());
        assert!(client.connect(Some(Duration::from_secs(60))).await);

        client.post::<()>("mod/ext/42").send().await.unwrap();
        assert_eq!(client.get::<u64>("mod/ext").send().await.unwrap(), 42);

        // Check that we can still access the built-in modules.
        assert_eq!(
            client
                .get::<u64>("status/latest_block_height")
                .send()
                .await
                .unwrap(),
            0
        );
        let (key, _) = BN254Pub::generated_from_seed_indexed([0; 32], 0);
        assert_eq!(
            client
                .get::<u64>(&format!("availability/proposals/{}/count", key.to_bytes()))
                .send()
                .await
                .unwrap(),
            0
        );
    }
}
