pub mod config;

use crate::config::{MAX_TXNS, MAX_VIEWS, TX_BATCH_SIZE};
use async_compatibility_layer::channel::OneShotReceiver;
use async_lock::RwLock;
use clap::Args;
use futures::FutureExt;

use hotshot_types::traits::signature_key::{EncodedPublicKey, SignatureKey};
use rand::{distributions::Alphanumeric, rngs::StdRng, thread_rng, Rng, SeedableRng};
use std::{collections::HashMap, io, path::PathBuf};
use tide_disco::{
    api::ApiError,
    error::ServerError,
    method::{ReadState, WriteState},
    Api, App, StatusCode,
};
use tracing::{debug, info};

type State<KEY> = RwLock<WebServerState<KEY>>;
type Error = ServerError;

/// State that tracks proposals and votes the server receives
/// Data is stored as a `Vec<u8>` to not incur overhead from deserializing
struct WebServerState<KEY> {
    /// view number -> (secret, proposal)
    proposals: HashMap<u64, (String, Vec<u8>)>,

    view_sync_proposals: HashMap<u64, Vec<(u64, Vec<u8>)>>,

    view_sync_proposal_index: HashMap<u64, u64>,
    /// view number -> (secret, da_certificates)
    da_certificates: HashMap<u64, (String, Vec<u8>)>,
    /// view for oldest proposals in memory
    oldest_proposal: u64,
    /// view for teh oldest DA certificate
    oldest_certificate: u64,

    oldest_view_sync_proposal: u64,
    /// view number -> Vec(index, vote)
    votes: HashMap<u64, Vec<(u64, Vec<u8>)>>,

    view_sync_votes: HashMap<u64, Vec<(u64, Vec<u8>)>>,
    /// view number -> highest vote index for that view number
    vote_index: HashMap<u64, u64>,

    view_sync_vote_index: HashMap<u64, u64>,
    /// view number of oldest votes in memory
    oldest_vote: u64,

    oldest_view_sync_vote: u64,

    /// index -> transaction
    // TODO ED Make indexable by hash of tx
    transactions: HashMap<u64, Vec<u8>>,
    txn_lookup: HashMap<Vec<u8>, u64>,
    /// highest transaction index
    num_txns: u64,

    /// shutdown signal
    shutdown: Option<OneShotReceiver<()>>,
    /// stake table with leader keys
    stake_table: Vec<KEY>,
    /// prng for generating endpoint
    _prng: StdRng,
}

impl<KEY: SignatureKey + 'static> WebServerState<KEY> {
    fn new() -> Self {
        Self {
            proposals: HashMap::new(),
            da_certificates: HashMap::new(),
            votes: HashMap::new(),
            num_txns: 0,
            oldest_vote: 0,
            oldest_proposal: 0,
            oldest_certificate: 0,
            shutdown: None,
            stake_table: Vec::new(),
            vote_index: HashMap::new(),
            transactions: HashMap::new(),
            txn_lookup: HashMap::new(),
            _prng: StdRng::from_entropy(),
            view_sync_proposals: HashMap::new(),
            view_sync_votes: HashMap::new(),
            view_sync_vote_index: HashMap::new(),
            oldest_view_sync_vote: 0,
            oldest_view_sync_proposal: 0,
            view_sync_proposal_index: HashMap::new(),
        }
    }
    pub fn with_shutdown_signal(mut self, shutdown_listener: Option<OneShotReceiver<()>>) -> Self {
        if self.shutdown.is_some() {
            panic!("A shutdown signal is already registered and can not be registered twice");
        }
        self.shutdown = shutdown_listener;
        self
    }
}

/// Trait defining methods needed for the `WebServerState`
pub trait WebServerDataSource<KEY> {
    fn get_proposal(&self, view_number: u64) -> Result<Option<Vec<Vec<u8>>>, Error>;
    fn get_view_sync_proposal(
        &self,
        view_number: u64,
        index: u64,
    ) -> Result<Option<Vec<Vec<u8>>>, Error>;

    fn get_votes(&self, view_number: u64, index: u64) -> Result<Option<Vec<Vec<u8>>>, Error>;
    fn get_view_sync_votes(
        &self,
        view_number: u64,
        index: u64,
    ) -> Result<Option<Vec<Vec<u8>>>, Error>;

    #[allow(clippy::type_complexity)]
    fn get_transactions(&self, index: u64) -> Result<Option<(u64, Vec<Vec<u8>>)>, Error>;
    fn get_da_certificate(&self, index: u64) -> Result<Option<Vec<Vec<u8>>>, Error>;
    fn post_vote(&mut self, view_number: u64, vote: Vec<u8>) -> Result<(), Error>;
    fn post_view_sync_vote(&mut self, view_number: u64, vote: Vec<u8>) -> Result<(), Error>;

    fn post_proposal(&mut self, view_number: u64, proposal: Vec<u8>) -> Result<(), Error>;
    fn post_view_sync_proposal(&mut self, view_number: u64, proposal: Vec<u8>)
        -> Result<(), Error>;

    fn post_da_certificate(&mut self, view_number: u64, cert: Vec<u8>) -> Result<(), Error>;
    fn post_transaction(&mut self, txn: Vec<u8>) -> Result<(), Error>;
    fn post_staketable(&mut self, key: Vec<u8>) -> Result<(), Error>;
    fn post_completed_transaction(&mut self, block: Vec<u8>) -> Result<(), Error>;
    fn post_secret_proposal(&mut self, _view_number: u64, _proposal: Vec<u8>) -> Result<(), Error>;
    fn proposal(&self, view_number: u64) -> Option<(String, Vec<u8>)>;
}

impl<KEY: SignatureKey> WebServerDataSource<KEY> for WebServerState<KEY> {
    fn proposal(&self, view_number: u64) -> Option<(String, Vec<u8>)> {
        self.proposals.get(&view_number).cloned()
    }
    /// Return the proposal the server has received for a particular view
    fn get_proposal(&self, view_number: u64) -> Result<Option<Vec<Vec<u8>>>, Error> {
        match self.proposals.get(&view_number) {
            Some(proposal) => {
                if proposal.1.is_empty() {
                    Err(ServerError {
                        status: StatusCode::NotImplemented,
                        message: format!("Proposal not found for view {view_number}"),
                    })
                } else {
                    Ok(Some(vec![proposal.1.clone()]))
                }
            }
            None => Err(ServerError {
                status: StatusCode::NotImplemented,
                message: format!("Proposal not found for view {view_number}"),
            }),
        }
    }

    fn get_view_sync_proposal(
        &self,
        view_number: u64,
        index: u64,
    ) -> Result<Option<Vec<Vec<u8>>>, Error> {
        let proposals = self.view_sync_proposals.get(&view_number);
        let mut ret_proposals = vec![];
        if let Some(cert) = proposals {
            for i in index..*self.view_sync_proposal_index.get(&view_number).unwrap() {
                ret_proposals.push(cert[i as usize].1.clone());
            }
        }
        if !ret_proposals.is_empty() {
            Ok(Some(ret_proposals))
        } else {
            Ok(None)
        }
    }

    /// Return all votes the server has received for a particular view from provided index to most recent
    fn get_votes(&self, view_number: u64, index: u64) -> Result<Option<Vec<Vec<u8>>>, Error> {
        let votes = self.votes.get(&view_number);
        let mut ret_votes = vec![];
        if let Some(votes) = votes {
            for i in index..*self.vote_index.get(&view_number).unwrap() {
                ret_votes.push(votes[i as usize].1.clone());
            }
        }
        if !ret_votes.is_empty() {
            Ok(Some(ret_votes))
        } else {
            Ok(None)
        }
    }

    fn get_view_sync_votes(
        &self,
        view_number: u64,
        index: u64,
    ) -> Result<Option<Vec<Vec<u8>>>, Error> {
        let votes = self.view_sync_votes.get(&view_number);
        let mut ret_votes = vec![];
        if let Some(votes) = votes {
            // error!("Passed in index is: {} self index is: {}", index, *self.vote_index.get(&view_number).unwrap());
            for i in index..*self.view_sync_vote_index.get(&view_number).unwrap() {
                ret_votes.push(votes[i as usize].1.clone());
            }
        }
        if !ret_votes.is_empty() {
            Ok(Some(ret_votes))
        } else {
            Ok(None)
        }
    }

    #[allow(clippy::type_complexity)]
    /// Return the transaction at the specified index (which will help with Nginx caching, but reduce performance otherwise)
    /// In the future we will return batches of transactions
    fn get_transactions(&self, index: u64) -> Result<Option<(u64, Vec<Vec<u8>>)>, Error> {
        let mut txns_to_return = vec![];

        let lowest_in_memory_txs = if self.num_txns < MAX_TXNS.try_into().unwrap() {
            0
        } else {
            self.num_txns as usize - MAX_TXNS
        };

        let starting_index = if (index as usize) < lowest_in_memory_txs {
            lowest_in_memory_txs
        } else {
            index as usize
        };

        for idx in starting_index..=self.num_txns.try_into().unwrap() {
            if let Some(txn) = self.transactions.get(&(idx as u64)) {
                txns_to_return.push(txn.clone())
            }
            if txns_to_return.len() >= TX_BATCH_SIZE as usize {
                break;
            }
        }

        if !txns_to_return.is_empty() {
            debug!("Returning this many txs {}", txns_to_return.len());
            //starting_index is the oldest index of the returned txns
            Ok(Some((starting_index as u64, txns_to_return)))
        } else {
            Err(ServerError {
                // TODO ED: Why does NoContent status code cause errors?
                status: StatusCode::NotImplemented,
                message: format!("Transaction not found for index {index}"),
            })
        }
    }

    /// Return the da certificate the server has received for a particular view
    fn get_da_certificate(&self, index: u64) -> Result<Option<Vec<Vec<u8>>>, Error> {
        match self.da_certificates.get(&index) {
            Some(cert) => {
                if cert.1.is_empty() {
                    Err(ServerError {
                        status: StatusCode::NotImplemented,
                        message: format!("DA Certificate not found for view {index}"),
                    })
                } else {
                    Ok(Some(vec![cert.1.clone()]))
                }
            }
            None => Err(ServerError {
                status: StatusCode::NotImplemented,
                message: format!("Proposal not found for view {index}"),
            }),
        }
    }

    /// Stores a received vote in the `WebServerState`
    fn post_vote(&mut self, view_number: u64, vote: Vec<u8>) -> Result<(), Error> {
        // Only keep vote history for MAX_VIEWS number of views
        if self.votes.len() >= MAX_VIEWS {
            self.votes.remove(&self.oldest_vote);
            while !self.votes.contains_key(&self.oldest_vote) {
                self.oldest_vote += 1;
            }
        }
        let highest_index = self.vote_index.entry(view_number).or_insert(0);
        self.votes
            .entry(view_number)
            .and_modify(|current_votes| current_votes.push((*highest_index, vote.clone())))
            .or_insert_with(|| vec![(*highest_index, vote)]);
        self.vote_index
            .entry(view_number)
            .and_modify(|index| *index += 1);
        Ok(())
    }

    fn post_view_sync_vote(&mut self, view_number: u64, vote: Vec<u8>) -> Result<(), Error> {
        // Only keep vote history for MAX_VIEWS number of views
        if self.view_sync_votes.len() >= MAX_VIEWS {
            self.view_sync_votes.remove(&self.oldest_view_sync_vote);
            while !self
                .view_sync_votes
                .contains_key(&self.oldest_view_sync_vote)
            {
                self.oldest_view_sync_vote += 1;
            }
        }
        let highest_index = self.view_sync_vote_index.entry(view_number).or_insert(0);
        self.view_sync_votes
            .entry(view_number)
            .and_modify(|current_votes| current_votes.push((*highest_index, vote.clone())))
            .or_insert_with(|| vec![(*highest_index, vote)]);
        self.view_sync_vote_index
            .entry(view_number)
            .and_modify(|index| {
                // Update the index if it's not just added.
                if *index > 0 {
                    *index += 1
                }
            });
        Ok(())
    }
    /// Stores a received proposal in the `WebServerState`
    fn post_proposal(&mut self, view_number: u64, mut proposal: Vec<u8>) -> Result<(), Error> {
        debug!("Received proposal for view {}", view_number);

        // Only keep proposal history for MAX_VIEWS number of view
        if self.proposals.len() >= MAX_VIEWS {
            self.proposals.remove(&self.oldest_proposal);
            while !self.proposals.contains_key(&self.oldest_proposal) {
                self.oldest_proposal += 1;
            }
        }
        self.proposals
            .entry(view_number)
            .and_modify(|(_, empty_proposal)| empty_proposal.append(&mut proposal))
            .or_insert_with(|| (String::new(), proposal));
        Ok(())
    }

    fn post_view_sync_proposal(
        &mut self,
        view_number: u64,
        proposal: Vec<u8>,
    ) -> Result<(), Error> {
        // Only keep proposal history for MAX_VIEWS number of view
        if self.view_sync_proposals.len() >= MAX_VIEWS {
            self.view_sync_proposals
                .remove(&self.oldest_view_sync_proposal);
            while !self
                .view_sync_proposals
                .contains_key(&self.oldest_view_sync_proposal)
            {
                self.oldest_view_sync_proposal += 1;
            }
        }
        let highest_index = self
            .view_sync_proposal_index
            .entry(view_number)
            .or_insert(0);
        self.view_sync_proposals
            .entry(view_number)
            .and_modify(|current_props| current_props.push((*highest_index, proposal.clone())))
            .or_insert_with(|| vec![(*highest_index, proposal)]);
        self.view_sync_proposal_index
            .entry(view_number)
            .and_modify(|index| {
                // Update the index if it's not just added.
                if *index > 0 {
                    *index += 1
                }
            });
        Ok(())
    }

    /// Stores a received DA certificate in the `WebServerState`
    fn post_da_certificate(&mut self, view_number: u64, mut cert: Vec<u8>) -> Result<(), Error> {
        debug!("Received DA Certificate for view {}", view_number);

        // Only keep proposal history for MAX_VIEWS number of view
        if self.da_certificates.len() >= MAX_VIEWS {
            self.da_certificates.remove(&self.oldest_certificate);
            while !self.da_certificates.contains_key(&self.oldest_certificate) {
                self.oldest_certificate += 1;
            }
        }
        self.da_certificates
            .entry(view_number)
            .and_modify(|(_, empty_cert)| empty_cert.append(&mut cert))
            .or_insert_with(|| (String::new(), cert));
        Ok(())
    }
    /// Stores a received group of transactions in the `WebServerState`
    fn post_transaction(&mut self, txn: Vec<u8>) -> Result<(), Error> {
        if self.transactions.len() >= MAX_TXNS {
            let old_txn = self.transactions.remove(&(self.num_txns - MAX_TXNS as u64));
            if let Some(old_txn) = old_txn {
                self.txn_lookup.remove(&old_txn);
            }
        }
        self.txn_lookup.insert(txn.clone(), self.num_txns);
        self.transactions.insert(self.num_txns, txn);
        self.num_txns += 1;

        debug!(
            "Received transaction!  Number of transactions received is: {}",
            self.num_txns
        );

        Ok(())
    }

    fn post_staketable(&mut self, key: Vec<u8>) -> Result<(), Error> {
        // KALEY TODO: need security checks here
        let new_key = KEY::from_bytes(&(EncodedPublicKey(key)));
        if let Some(new_key) = new_key {
            let node_index = self.stake_table.len() as u64;
            //generate secret for leader's first submission endpoint when key is added
            let secret = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(30)
                .map(char::from)
                .collect();
            self.proposals.insert(node_index, (secret, Vec::new()));
            self.stake_table.push(new_key);
            Ok(())
        } else {
            Err(ServerError {
                status: StatusCode::BadRequest,
                message: "Only signature keys can be added to stake table".to_string(),
            })
        }
    }

    fn post_completed_transaction(&mut self, txn: Vec<u8>) -> Result<(), Error> {
        if let Some(idx) = self.txn_lookup.remove(&txn) {
            self.transactions.remove(&idx);
            Ok(())
        } else {
            Err(ServerError {
                status: StatusCode::BadRequest,
                message: "Transaction Not Found".to_string(),
            })
        }
    }

    //KALEY TODO: this will be merged with post_proposal once it is fully working,
    //but keeping it separate to not break things in the meantime
    fn post_secret_proposal(
        &mut self,
        view_number: u64,
        mut proposal: Vec<u8>,
    ) -> Result<(), Error> {
        debug!("Received proposal for view {}", view_number);

        // Only keep proposal history for MAX_VIEWS number of views
        if self.proposals.len() >= MAX_VIEWS {
            self.proposals.remove(&self.oldest_proposal);
            while !self.proposals.contains_key(&self.oldest_proposal) {
                self.oldest_proposal += 1;
            }
        }
        self.proposals
            .entry(view_number)
            .and_modify(|(_, empty_proposal)| empty_proposal.append(&mut proposal));

        //generate new secret for the next time this node is leader
        let secret = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();
        let next_view_for_leader = view_number + self.stake_table.len() as u64;
        self.proposals
            .insert(next_view_for_leader, (secret, Vec::new()));
        Ok(())
    }
}

#[derive(Args, Default)]
pub struct Options {
    #[arg(long = "web-server-api-path", env = "WEB_SERVER_API_PATH")]
    pub api_path: Option<PathBuf>,
}

/// Sets up all API routes
fn define_api<State, KEY>(options: &Options) -> Result<Api<State, Error>, ApiError>
where
    State: 'static + Send + Sync + ReadState + WriteState,
    <State as ReadState>::State: Send + Sync + WebServerDataSource<KEY>,
    KEY: SignatureKey,
{
    let mut api = match &options.api_path {
        Some(path) => Api::<State, Error>::from_file(path)?,
        None => {
            let toml = toml::from_str(include_str!("../api.toml")).map_err(|err| {
                ApiError::CannotReadToml {
                    reason: err.to_string(),
                }
            })?;
            Api::<State, Error>::new(toml)?
        }
    };
    api.get("getproposal", |req, state| {
        async move {
            let view_number: u64 = req.integer_param("view_number")?;
            state.get_proposal(view_number)
        }
        .boxed()
    })?
    .get("getviewsyncproposal", |req, state| {
        async move {
            let view_number: u64 = req.integer_param("view_number")?;
            let index: u64 = req.integer_param("index")?;
            state.get_view_sync_proposal(view_number, index)
        }
        .boxed()
    })?
    .get("getcertificate", |req, state| {
        async move {
            let view_number: u64 = req.integer_param("view_number")?;
            state.get_da_certificate(view_number)
        }
        .boxed()
    })?
    .get("getvotes", |req, state| {
        async move {
            let view_number: u64 = req.integer_param("view_number")?;
            let index: u64 = req.integer_param("index")?;
            state.get_votes(view_number, index)
        }
        .boxed()
    })?
    .get("getviewsyncvotes", |req, state| {
        async move {
            let view_number: u64 = req.integer_param("view_number")?;
            let index: u64 = req.integer_param("index")?;
            state.get_view_sync_votes(view_number, index)
        }
        .boxed()
    })?
    .get("gettransactions", |req, state| {
        async move {
            let index: u64 = req.integer_param("index")?;
            state.get_transactions(index)
        }
        .boxed()
    })?
    .post("postvote", |req, state| {
        async move {
            let view_number: u64 = req.integer_param("view_number")?;
            // Using body_bytes because we don't want to deserialize; body_auto or body_json deserializes automatically
            let vote = req.body_bytes();
            state.post_vote(view_number, vote)
        }
        .boxed()
    })?
    .post("postviewsyncvote", |req, state| {
        async move {
            let view_number: u64 = req.integer_param("view_number")?;
            // Using body_bytes because we don't want to deserialize; body_auto or body_json deserializes automatically
            let vote = req.body_bytes();
            state.post_view_sync_vote(view_number, vote)
        }
        .boxed()
    })?
    .post("postproposal", |req, state| {
        async move {
            let view_number: u64 = req.integer_param("view_number")?;
            let proposal = req.body_bytes();
            state.post_proposal(view_number, proposal)
        }
        .boxed()
    })?
    .post("postviewsyncproposal", |req, state| {
        async move {
            let view_number: u64 = req.integer_param("view_number")?;
            let proposal = req.body_bytes();
            state.post_view_sync_proposal(view_number, proposal)
        }
        .boxed()
    })?
    .post("postcertificate", |req, state| {
        async move {
            let view_number: u64 = req.integer_param("view_number")?;
            let cert = req.body_bytes();
            state.post_da_certificate(view_number, cert)
        }
        .boxed()
    })?
    .post("posttransaction", |req, state| {
        async move {
            let txns = req.body_bytes();
            state.post_transaction(txns)
        }
        .boxed()
    })?
    .post("poststaketable", |req, state| {
        async move {
            //works one key at a time for now
            let key = req.body_bytes();
            state.post_staketable(key)
        }
        .boxed()
    })?
    .post("postcompletedtransaction", |req, state| {
        async move {
            //works one txn at a time for now
            let txn = req.body_bytes();
            state.post_completed_transaction(txn)
        }
        .boxed()
    })?
    .post("secret", |req, state| {
        async move {
            let view_number: u64 = req.integer_param("view_number")?;
            let secret: &str = req.string_param("secret")?;
            //if secret is correct and view_number->proposal is empty, proposal is valid
            if let Some(prop) = state.proposal(view_number) {
                if prop.1.is_empty() {
                    if prop.0 == secret {
                        let proposal = req.body_bytes();
                        state.post_secret_proposal(view_number, proposal)
                    } else {
                        Err(ServerError {
                            status: StatusCode::BadRequest,
                            message: format!(
                                "Wrong secret value for proposal for view {:?}",
                                view_number
                            ),
                        })
                    }
                } else {
                    Err(ServerError {
                        status: StatusCode::BadRequest,
                        message: format!("Proposal already submitted for view {:?}", view_number),
                    })
                }
            } else {
                Err(ServerError {
                    status: StatusCode::BadRequest,
                    message: format!("No endpoint for view number {} yet", view_number),
                })
            }
        }
        .boxed()
    })?;
    Ok(api)
}

pub async fn run_web_server<KEY: SignatureKey + 'static>(
    shutdown_listener: Option<OneShotReceiver<()>>,
    port: u16,
) -> io::Result<()> {
    let options = Options::default();

    let api = define_api(&options).unwrap();
    let state = State::new(WebServerState::new().with_shutdown_signal(shutdown_listener));
    let mut app = App::<State<KEY>, Error>::with_state(state);

    app.register_module("api", api).unwrap();

    let app_future = app.serve(format!("http://0.0.0.0:{port}"));

    info!("Web server started on port {port}");

    app_future.await
}
