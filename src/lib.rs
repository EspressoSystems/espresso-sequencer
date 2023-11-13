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
//!     data_source::{UpdateDataSource, QueryData},
//!     status, Error
//! };
//!
//! use async_std::{sync::{Arc, RwLock}, task::spawn};
//! use futures::StreamExt;
//! use hotshot::SystemContext;
//! use tide_disco::App;
//!
//! // Create or open query data.
//! let query_data = QueryData::<AppTypes, AppNodeImpl, ()>::create(storage_path, ())
//!     .map_err(Error::internal)?;
//!
//! // Create hotshot, giving it a handle to the status metrics.
//! let (mut hotshot, _) = SystemContext::<AppTypes, AppNodeImpl>::init(
//! #   panic!(), panic!(), panic!(), panic!(), panic!(), panic!(), panic!(),
//!     query_data.metrics(),
//!     // Other fields omitted
//! ).await.map_err(Error::internal)?;
//!
//! // Create API modules.
//! let availability_api = availability::define_api(&Default::default())
//!     .map_err(Error::internal)?;
//! let status_api = status::define_api(&Default::default())
//!     .map_err(Error::internal)?;
//!
//! // Create app. We wrap `query_data` into an `RwLock` so we can share it with the web server.
//! let query_data = Arc::new(RwLock::new(query_data));
//! let mut app = App::<_, Error>::with_state(query_data.clone());
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
//!     let mut query_data = query_data.write().await;
//!
//!     // Update the query data based on this event.
//!     query_data.update(&event);
//!     query_data.commit_version().await.map_err(Error::internal)?;
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
//! # use hotshot_query_service::{data_source::QueryData, Error, Options};
//! # use hotshot_query_service::testing::mocks::{MockTypes, MockNodeImpl};
//! # use std::path::Path;
//! # fn doc(storage_path: &Path, options: &Options, hotshot: SystemContextHandle<MockTypes, MockNodeImpl>) -> Result<(), Error> {
//! use hotshot_query_service::run_standalone_service;
//!
//! let query_data = QueryData::create(storage_path, ()).map_err(Error::internal)?;
//! spawn(run_standalone_service(options, query_data, hotshot));
//! # Ok(())
//! # }
//! ```
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
//! required additional data structures to the module state, and creating a new API endpoint to
//! expose the functionality. For the former, you can take advantage of the `UserData` type
//! parameter of the [QueryData] state to inject whatever additional data you need. In the case of
//! adding a UTXO index, it might look like this:
//!
//! ```
//! # use hotshot_query_service::availability::{
//! #     self, TransactionIndex,
//! # };
//! # use hotshot_query_service::data_source::QueryData;
//! # use hotshot_query_service::testing::mocks::{
//! #   MockNodeImpl as AppNodeImpl, MockTypes as AppTypes,
//! # };
//! # use std::collections::HashMap;
//! #[derive(Default)]
//! struct AppQueryData {
//!     // Index mapping UTXO index to (block index, transaction index, output index)
//!     utxo_index: HashMap<u64, (u64, TransactionIndex<AppTypes>, u64)>,
//! }
//!
//! type AvailabilityState = QueryData<AppTypes, AppNodeImpl, AppQueryData>;
//! ```
//!
//! `QueryData<AppTypes, AppNodeImpl, AppQueryData>` implements `AsRef<AppQueryData>` and
//! `AsMut<AppQueryData>`, so you can now modify the default availablity API with the addition of a
//! new endpoint that accesses `AppQueryData` like so:
//!
//! ```
//! # use async_std::sync::RwLock;
//! # use futures::FutureExt;
//! # use hotshot_query_service::availability::{
//! #   self, AvailabilityDataSource, QueryBlockSnafu, TransactionIndex,
//! # };
//! # use hotshot_query_service::data_source::QueryData;
//! # use hotshot_query_service::testing::mocks::{
//! #   MockNodeImpl as AppNodeImpl, MockTypes as AppTypes,
//! # };
//! # use hotshot_query_service::Error;
//! # use snafu::ResultExt;
//! # use std::collections::HashMap;
//! # use std::path::Path;
//! # use tide_disco::{api::ApiError, method::ReadState, Api, App, StatusCode};
//! #[derive(Default)]
//! # struct AppQueryData {
//! #     utxo_index: HashMap<u64, (usize, TransactionIndex<AppTypes>, usize)>,
//! # }
//! # type AvailabilityState = QueryData<AppTypes, AppNodeImpl, AppQueryData>;
//! fn define_app_specific_availability_api<State>(
//!     options: &availability::Options,
//! ) -> Result<Api<State, availability::Error>, ApiError>
//! where
//!     State: 'static + Send + Sync + ReadState,
//!     <State as ReadState>::State:
//!         Send +
//!         Sync +
//!         AvailabilityDataSource<AppTypes, AppNodeImpl> +
//!         AsRef<AppQueryData>,
//! {
//!     let mut api = availability::define_api(options)?;
//!     api.get("get_utxo", |req, state: &<State as ReadState>::State| async move {
//!         let utxo_index = req.integer_param("index")?;
//!         let app_query_data = state.as_ref();
//!         let (block_index, txn_index, output_index) = *app_query_data
//!             .utxo_index
//!             .get(&utxo_index)
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
//! fn init_server(
//!     options: &availability::Options,
//!     storage_path: &Path,
//! ) -> Result<App<RwLock<AvailabilityState>, Error>, availability::Error> {
//!     let api = define_app_specific_availability_api(options)
//!         .map_err(availability::Error::internal)?;
//!     let state = AvailabilityState::create(storage_path, AppQueryData::default())
//!         .map_err(availability::Error::internal)?;
//!     let mut app = App::with_state(RwLock::new(state));
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
//! The one wrinkle is that all modules within a [tide_disco] app must share the state type. It is
//! for this reason that the modules provided by this crate are generic on the state type -- both
//! [availability::define_api] and [status::define_api] can work with any state type, provided that
//! type implements the corresponding data source traits. The application state provided by this
//! crate ([data_source::QueryData]) implements both of these traits, but if you want to use a
//! custom state type that includes state for other modules, you will need to implement these traits
//! for your custom type. The basic pattern looks like:
//!
//! ```
//! # use async_trait::async_trait;
//! # use hotshot_types::traits::signature_key::EncodedPublicKey;
//! # use hotshot_query_service::availability::{
//! #   AvailabilityDataSource, BlockId, BlockQueryData, LeafId, LeafQueryData, QueryResult,
//! #   TransactionHash, TransactionIndex,
//! # };
//! # use hotshot_query_service::data_source::QueryData;
//! # use hotshot_query_service::status::{MempoolQueryData, StatusDataSource};
//! # use hotshot_query_service::testing::mocks::{
//! #   MockNodeImpl as AppNodeImpl, MockTypes as AppTypes,
//! # };
//! # use std::ops::RangeBounds;
//! # type AppQueryData = ();
//! struct AppState {
//!     hotshot_qs: QueryData<AppTypes, AppNodeImpl, AppQueryData>,
//!     // additional state for other modules
//! }
//!
//! // Implement data source trait for availability API.
//! #[async_trait]
//! impl AvailabilityDataSource<AppTypes, AppNodeImpl> for AppState {
//!     type LeafRange<'a, R> =
//!         <QueryData<AppTypes, AppNodeImpl, AppQueryData> as
//!             AvailabilityDataSource<AppTypes, AppNodeImpl>>::LeafRange<'a, R>
//!     where
//!         Self: 'a,
//!         R: RangeBounds<usize> + Send;
//!     type BlockRange<'a, R> =
//!         <QueryData<AppTypes, AppNodeImpl, AppQueryData> as
//!             AvailabilityDataSource<AppTypes, AppNodeImpl>>::BlockRange<'a, R>
//!     where
//!         Self: 'a,
//!         R: RangeBounds<usize> + Send;
//!
//!     type LeafStream =
//!         <QueryData<AppTypes, AppNodeImpl, AppQueryData> as
//!             AvailabilityDataSource<AppTypes, AppNodeImpl>>::LeafStream;
//!     type BlockStream =
//!         <QueryData<AppTypes, AppNodeImpl, AppQueryData> as
//!             AvailabilityDataSource<AppTypes, AppNodeImpl>>::BlockStream;
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
//! // Implement data source trait for status API.
//! #[async_trait]
//! impl StatusDataSource for AppState {
//!     type Error = <QueryData<AppTypes, AppNodeImpl, AppQueryData> as StatusDataSource>::Error;
//!
//!     async fn block_height(&self) -> Result<usize, Self::Error> {
//!         self.hotshot_qs.block_height().await
//!     }
//!
//!     // etc
//! #   async fn mempool_info(&self) -> Result<MempoolQueryData, <Self as StatusDataSource>::Error> { todo!() }
//! #   async fn success_rate(&self) -> Result<f64, <Self as StatusDataSource>::Error> { todo!() }
//! #   async fn export_metrics(&self) -> Result<String, <Self as StatusDataSource>::Error> { todo!() }
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
//! instantiate the state as normal, using [create](QueryData::create) or [open](QueryData::open) to
//! initialize the [QueryData] part of the `AppState`. _However_, this only works if you want the
//! persistent storage for the availability and status modules (managed by [QueryData]) to be
//! independent of the persistent storage for other modules. You may well want to synchronize the
//! storage for all modules together, so that updates to the entire application state can be done
//! atomically. This is particularly relevant if one of your application-specific modules updates
//! its storage based on a stream of HotShot leaves. Since the availability module also updates with
//! each new leaf, you probably want these two modules to stay in sync.
//!
//! To achieve this, add a top level [AtomicStore](atomic_store::AtomicStore) to `AppState` to
//! synchronize all persistent storage, and use [create_with_store](QueryData::create_with_store) or
//! [open_with_store](QueryData::open_with_store) when initializeing the [QueryData] to associate
//! its persistent storage with the top-level atomic store. In this case, [QueryData] will not
//! manage the [AtomicStore](atomic_store::AtomicStore) for you, so you must be sure to call
//! [AtomicStore::commit_version](atomic_store::AtomicStore::commit_version) after each call to
//! [QueryData::commit_version], and you must call either [QueryData::commit_version] or
//! [QueryData::skip_version] before each call to
//! [AtomicStore::commit_version](atomic_store::AtomicStore::commit_version).
//!
//! ```
//! # use async_std::{sync::{Arc, RwLock}, task::spawn};
//! # use atomic_store::{AtomicStore, AtomicStoreLoader};
//! # use futures::StreamExt;
//! # use hotshot::types::SystemContextHandle;
//! # use hotshot_query_service::Error;
//! # use hotshot_query_service::data_source::{UpdateDataSource, QueryData};
//! # use hotshot_query_service::testing::mocks::{
//! #   MockNodeImpl as AppNodeImpl, MockTypes as AppTypes,
//! # };
//! # use std::path::Path;
//! # use tide_disco::App;
//! # type AppQueryData = ();
//! struct AppState {
//!     // Top-level storage coordinator
//!     store: AtomicStore,
//!     hotshot_qs: QueryData<AppTypes, AppNodeImpl, AppQueryData>,
//!     // additional state for other modules
//! }
//!
//! fn init_server(
//!     storage_path: &Path,
//!     mut hotshot: SystemContextHandle<AppTypes, AppNodeImpl>,
//! ) -> Result<App<Arc<RwLock<AppState>>, Error>, Error> {
//!     let mut loader = AtomicStoreLoader::create(storage_path, "my_app") // or `open`
//!         .map_err(Error::internal)?;
//!     let hotshot_qs = QueryData::create_with_store(&mut loader, AppQueryData::default())
//!         .map_err(Error::internal)?;
//!     // Initialize storage for other modules using the same loader.
//!
//!     let store = AtomicStore::open(loader).map_err(Error::internal)?;
//!     let state = Arc::new(RwLock::new(AppState {
//!         store,
//!         hotshot_qs,
//!         // additional state for other modules
//!     }));
//!     let mut app = App::with_state(state.clone());
//!     // Register API modules.
//!
//!     spawn(async move {
//!         let mut events = hotshot.get_event_stream(Default::default()).await.0;
//!         while let Some(event) = events.next().await {
//!             let mut state = state.write().await;
//!             state.hotshot_qs.update(&event).await.unwrap();
//!             // Update other modules' states based on `event`.
//!
//!             state.hotshot_qs.commit_version().await.unwrap();
//!             // Commit or skip versions for other modules' storage.
//!             state.store.commit_version().unwrap();
//!         }
//!     });
//!
//!     Ok(app)
//! }
//! ```
//!

mod api;
pub mod availability;
pub mod data_source;
mod error;
mod ledger_log;
mod metrics;
mod resolvable;
pub mod status;
pub mod testing;
mod update;

pub use availability::QueryableBlock;
pub use error::Error;
pub use resolvable::Resolvable;

use data_source::QueryData;
use futures::Future;
use hotshot::{certificate, types::SystemContextHandle};
use hotshot_types::{
    data::LeafType,
    traits::{
        block_contents,
        node_implementation::{NodeImplementation, NodeType},
    },
};

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

#[derive(clap::Args, Default)]
pub struct Options {
    #[clap(flatten)]
    pub availability: availability::Options,
    #[clap(flatten)]
    pub status: status::Options,
}

/// Run an instance of the HotShot Query service with no customization.
pub fn run_standalone_service<Types: NodeType, I: NodeImplementation<Types>>(
    _options: &Options,
    _data_source: QueryData<Types, I, ()>,
    _hotshot: SystemContextHandle<Types, I>,
) -> impl Future<Output = ()> + Send + Sync + 'static
where
    Block<Types>: QueryableBlock,
{
    async move { unimplemented!() }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        availability::{
            AvailabilityDataSource, BlockId, BlockQueryData, LeafId, LeafQueryData, QueryResult,
            TransactionHash, TransactionIndex,
        },
        status::{MempoolQueryData, StatusDataSource},
        testing::mocks::{MockNodeImpl, MockTypes},
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
        hotshot_qs: QueryData<MockTypes, MockNodeImpl, ()>,
        module_state: RollingLog<BincodeLoadStore<u64>>,
    }

    #[async_trait]
    impl AvailabilityDataSource<MockTypes, MockNodeImpl> for CompositeState {
        type LeafStream = <QueryData<MockTypes, MockNodeImpl, ()> as AvailabilityDataSource<
            MockTypes,
            MockNodeImpl,
        >>::LeafStream;
        type BlockStream = <QueryData<MockTypes, MockNodeImpl, ()> as AvailabilityDataSource<
            MockTypes,
            MockNodeImpl,
        >>::BlockStream;

        type LeafRange<'a, R> =
            <QueryData<MockTypes, MockNodeImpl, ()> as AvailabilityDataSource<
                MockTypes,
                MockNodeImpl,
            >>::LeafRange<'a, R>
        where
            Self: 'a,
            R: RangeBounds<usize> + Send;
        type BlockRange<'a, R> =
            <QueryData<MockTypes, MockNodeImpl, ()> as AvailabilityDataSource<
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
        type Error = <QueryData<MockTypes, MockNodeImpl, ()> as StatusDataSource>::Error;

        async fn block_height(&self) -> Result<usize, Self::Error> {
            self.hotshot_qs.block_height().await
        }
        async fn mempool_info(&self) -> Result<MempoolQueryData, Self::Error> {
            self.hotshot_qs.mempool_info().await
        }
        async fn success_rate(&self) -> Result<f64, Self::Error> {
            self.hotshot_qs.success_rate().await
        }
        async fn export_metrics(&self) -> Result<String, Self::Error> {
            self.hotshot_qs.export_metrics().await
        }
    }

    #[async_std::test]
    async fn test_composition() {
        let dir = TempDir::new("test_composition").unwrap();
        let mut loader = AtomicStoreLoader::create(dir.path(), "test_composition").unwrap();
        let hotshot_qs = QueryData::create_with_store(&mut loader, ()).unwrap();
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
