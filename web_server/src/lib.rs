//! Web server for `HotShot`

/// Configuration for the webserver
pub mod config;

use crate::config::{MAX_TXNS, MAX_VIEWS, TX_BATCH_SIZE};
use async_compatibility_layer::channel::OneShotReceiver;
use async_lock::RwLock;
use clap::Args;
use futures::FutureExt;

use hotshot_types::traits::signature_key::SignatureKey;
use rand::{distributions::Alphanumeric, rngs::StdRng, thread_rng, Rng, SeedableRng};
use std::{
    collections::{BTreeMap, HashMap},
    io,
    path::PathBuf,
};
use tide_disco::{
    api::ApiError,
    error::ServerError,
    method::{ReadState, WriteState},
    Api, App, StatusCode, Url,
};
use tracing::{debug, info};
use vbs::version::StaticVersionType;

/// Convience alias for a lock over the state of the app
/// TODO this is used in two places. It might be clearer to just inline
type State<KEY> = RwLock<WebServerState<KEY>>;
/// Convience alias for errors in this crate
type Error = ServerError;

/// State that tracks proposals and votes the server receives
/// Data is stored as a `Vec<u8>` to not incur overhead from deserializing
// TODO should the view numbers be generic over time?
struct WebServerState<KEY> {
    /// view number -> (secret, proposal)
    proposals: BTreeMap<u64, (String, Vec<u8>)>,
    /// view number -> (secret, proposal)
    upgrade_proposals: BTreeMap<u64, (String, Vec<u8>)>,
    /// for view sync: view number -> (relay, certificate)
    view_sync_certificates: BTreeMap<u64, Vec<(u64, Vec<u8>)>>,
    /// view number -> relay
    view_sync_certificate_index: HashMap<u64, u64>,
    /// view number -> (secret, da_certificates)
    da_certificates: HashMap<u64, (String, Vec<u8>)>,
    /// view for the most recent proposal to help nodes catchup
    latest_proposal: u64,
    /// view for the most recent view sync proposal
    latest_view_sync_certificate: u64,
    /// view for the oldest DA certificate
    oldest_certificate: u64,
    /// view number -> Vec(index, vote)
    votes: HashMap<u64, Vec<(u64, Vec<u8>)>>,
    /// view number -> Vec(index, vote)
    upgrade_votes: HashMap<u64, Vec<(u64, Vec<u8>)>>,
    /// view sync: view number -> Vec(relay, vote)
    view_sync_votes: HashMap<u64, Vec<(u64, Vec<u8>)>>,
    /// view number -> highest vote index for that view number
    vote_index: HashMap<u64, u64>,
    /// view number -> highest vote index for that view number
    upgrade_vote_index: HashMap<u64, u64>,
    /// view_sync: view number -> highest vote index for that view number
    view_sync_vote_index: HashMap<u64, u64>,
    /// view number of oldest votes in memory
    oldest_vote: u64,
    /// view number of oldest votes in memory
    oldest_upgrade_vote: u64,
    /// view sync: view number of oldest votes in memory
    oldest_view_sync_vote: u64,
    /// view number -> (secret, string)
    vid_disperses: HashMap<u64, (String, Vec<u8>)>,
    /// view for the oldest vid disperal
    oldest_vid_disperse: u64,
    /// view of most recent vid dispersal
    recent_vid_disperse: u64,
    /// votes that a node got, that is, their VID share
    vid_votes: HashMap<u64, Vec<(u64, Vec<u8>)>>,
    /// oldest vid vote view number
    oldest_vid_vote: u64,
    /// recent_vid_vote view number
    vid_certificates: HashMap<u64, (String, Vec<u8>)>,
    /// oldest vid certificate view number
    oldest_vid_certificate: u64,
    /// recent_vid_certificate: u64,
    vid_vote_index: HashMap<u64, u64>,
    /// index -> transaction
    // TODO ED Make indexable by hash of tx
    transactions: HashMap<u64, Vec<u8>>,
    /// tx hash -> tx index, is currently unused
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
    /// Create new web server state
    fn new() -> Self {
        Self {
            proposals: BTreeMap::new(),
            upgrade_proposals: BTreeMap::new(),
            da_certificates: HashMap::new(),
            votes: HashMap::new(),
            num_txns: 0,
            oldest_vote: 0,
            latest_proposal: 0,
            latest_view_sync_certificate: 0,
            oldest_certificate: 0,
            shutdown: None,
            stake_table: Vec::new(),
            vote_index: HashMap::new(),
            transactions: HashMap::new(),
            txn_lookup: HashMap::new(),
            _prng: StdRng::from_entropy(),
            view_sync_certificates: BTreeMap::new(),
            view_sync_votes: HashMap::new(),
            view_sync_vote_index: HashMap::new(),
            upgrade_votes: HashMap::new(),
            oldest_upgrade_vote: 0,
            upgrade_vote_index: HashMap::new(),

            vid_disperses: HashMap::new(),
            oldest_vid_disperse: 0,
            recent_vid_disperse: 0,

            vid_votes: HashMap::new(),
            oldest_vid_vote: 0,
            // recent_vid_vote: 0,
            vid_certificates: HashMap::new(),
            oldest_vid_certificate: 0,
            // recent_vid_certificate: 0,
            vid_vote_index: HashMap::new(),

            oldest_view_sync_vote: 0,
            view_sync_certificate_index: HashMap::new(),
        }
    }
    /// Provide a shutdown signal to the server
    /// # Panics
    /// Panics if already shut down
    #[allow(clippy::panic)]
    pub fn with_shutdown_signal(mut self, shutdown_listener: Option<OneShotReceiver<()>>) -> Self {
        assert!(
            self.shutdown.is_none(),
            "A shutdown signal is already registered and can not be registered twice"
        );
        self.shutdown = shutdown_listener;
        self
    }
}

/// Trait defining methods needed for the `WebServerState`
pub trait WebServerDataSource<KEY> {
    /// Get proposal
    /// # Errors
    /// Error if unable to serve.
    fn get_proposal(&self, view_number: u64) -> Result<Option<Vec<Vec<u8>>>, Error>;
    /// Get upgrade proposal
    /// # Errors
    /// Error if unable to serve.
    fn get_upgrade_proposal(&self, view_number: u64) -> Result<Option<Vec<Vec<u8>>>, Error>;
    /// Get latest quanrum proposal
    /// # Errors
    /// Error if unable to serve.
    fn get_latest_proposal(&self) -> Result<Option<Vec<Vec<u8>>>, Error>;
    /// Get latest view sync proposal
    /// # Errors
    /// Error if unable to serve.
    fn get_latest_view_sync_certificate(&self) -> Result<Option<Vec<Vec<u8>>>, Error>;
    /// Get view sync proposal
    /// # Errors
    /// Error if unable to serve.
    fn get_view_sync_certificate(
        &self,
        view_number: u64,
        index: u64,
    ) -> Result<Option<Vec<Vec<u8>>>, Error>;

    /// Get vote
    /// # Errors
    /// Error if unable to serve.
    fn get_votes(&self, view_number: u64, index: u64) -> Result<Option<Vec<Vec<u8>>>, Error>;
    /// Get upgrade votes
    /// # Errors
    /// Error if unable to serve.
    fn get_upgrade_votes(
        &self,
        view_number: u64,
        index: u64,
    ) -> Result<Option<Vec<Vec<u8>>>, Error>;
    /// Get view sync votes
    /// # Errors
    /// Error if unable to serve.
    fn get_view_sync_votes(
        &self,
        view_number: u64,
        index: u64,
    ) -> Result<Option<Vec<Vec<u8>>>, Error>;

    #[allow(clippy::type_complexity)]
    /// Get transactions
    /// # Errors
    /// Error if unable to serve.
    fn get_transactions(&self, index: u64) -> Result<Option<(u64, Vec<Vec<u8>>)>, Error>;
    /// Get da certificate
    /// # Errors
    /// Error if unable to serve.
    fn get_da_certificate(&self, index: u64) -> Result<Option<Vec<Vec<u8>>>, Error>;
    /// Post vote
    /// # Errors
    /// Error if unable to serve.
    fn post_vote(&mut self, view_number: u64, vote: Vec<u8>) -> Result<(), Error>;
    /// Post upgrade vote
    /// # Errors
    /// Error if unable to serve.
    fn post_upgrade_vote(&mut self, view_number: u64, vote: Vec<u8>) -> Result<(), Error>;
    /// Post view sync vote
    /// # Errors
    /// Error if unable to serve.
    fn post_view_sync_vote(&mut self, view_number: u64, vote: Vec<u8>) -> Result<(), Error>;

    /// Post proposal
    /// # Errors
    /// Error if unable to serve.
    fn post_proposal(&mut self, view_number: u64, proposal: Vec<u8>) -> Result<(), Error>;
    /// Post upgrade proposal
    /// # Errors
    /// Error if unable to serve.
    fn post_upgrade_proposal(&mut self, view_number: u64, proposal: Vec<u8>) -> Result<(), Error>;
    /// Post view sync certificate
    /// # Errors
    /// Error if unable to serve.
    fn post_view_sync_certificate(
        &mut self,
        view_number: u64,
        certificate: Vec<u8>,
    ) -> Result<(), Error>;

    /// Post data avaiability certificate
    /// # Errors
    /// Error if unable to serve.
    fn post_da_certificate(&mut self, view_number: u64, cert: Vec<u8>) -> Result<(), Error>;
    /// Post transaction
    /// # Errors
    /// Error if unable to serve.
    fn post_transaction(&mut self, txn: Vec<u8>) -> Result<(), Error>;
    /// Post staketable
    /// # Errors
    /// Error if unable to serve.
    fn post_staketable(&mut self, key: Vec<u8>) -> Result<(), Error>;
    /// Post completed transaction
    /// # Errors
    /// Error if unable to serve.
    fn post_completed_transaction(&mut self, block: Vec<u8>) -> Result<(), Error>;
    /// Post secret proposal
    /// # Errors
    /// Error if unable to serve.
    fn post_secret_proposal(&mut self, _view_number: u64, _proposal: Vec<u8>) -> Result<(), Error>;
    /// Post proposal
    /// # Errors
    /// Error if unable to serve.
    fn proposal(&self, view_number: u64) -> Option<(String, Vec<u8>)>;
    /// Post vid disperal
    /// # Errors
    /// Error if unable to serve.
    fn post_vid_disperse(&mut self, view_number: u64, disperse: Vec<u8>) -> Result<(), Error>;
    /// Post vid vote
    /// # Errors
    /// Error if unable to serve.
    fn post_vid_vote(&mut self, view_number: u64, vote: Vec<u8>) -> Result<(), Error>;
    /// post vid certificate
    /// # Errors
    /// Error if unable to serve.
    fn post_vid_certificate(&mut self, view_number: u64, certificate: Vec<u8>)
        -> Result<(), Error>;
    /// Get vid dispersal
    /// # Errors
    /// Error if unable to serve.
    fn get_vid_disperse(&self, view_number: u64) -> Result<Option<Vec<Vec<u8>>>, Error>;
    /// Get vid votes
    /// # Errors
    /// Error if unable to serve.
    fn get_vid_votes(&self, view_number: u64, index: u64) -> Result<Option<Vec<Vec<u8>>>, Error>;
    /// Get vid certificates
    /// # Errors
    /// Error if unable to serve.
    fn get_vid_certificate(&self, index: u64) -> Result<Option<Vec<Vec<u8>>>, Error>;
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
                        message: format!("Proposal empty for view {view_number}"),
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
    /// Return the proposal the server has received for a particular view
    fn get_upgrade_proposal(&self, view_number: u64) -> Result<Option<Vec<Vec<u8>>>, Error> {
        match self.upgrade_proposals.get(&view_number) {
            Some(proposal) => {
                if proposal.1.is_empty() {
                    Err(ServerError {
                        status: StatusCode::NotImplemented,
                        message: format!("Proposal empty for view {view_number}"),
                    })
                } else {
                    tracing::error!("found proposal");
                    Ok(Some(vec![proposal.1.clone()]))
                }
            }
            None => Err(ServerError {
                status: StatusCode::NotImplemented,
                message: format!("Proposal not found for view {view_number}"),
            }),
        }
    }

    /// Return the VID disperse data that the server has received for a particular view
    fn get_vid_disperse(&self, view_number: u64) -> Result<Option<Vec<Vec<u8>>>, Error> {
        match self.vid_disperses.get(&view_number) {
            Some(disperse) => {
                if disperse.1.is_empty() {
                    Err(ServerError {
                        status: StatusCode::NotImplemented,
                        message: format!("VID disperse not found for view {view_number}"),
                    })
                } else {
                    Ok(Some(vec![disperse.1.clone()]))
                }
            }
            None => Err(ServerError {
                status: StatusCode::NotImplemented,
                message: format!("VID disperse not found for view {view_number}"),
            }),
        }
    }

    fn get_latest_proposal(&self) -> Result<Option<Vec<Vec<u8>>>, Error> {
        self.get_proposal(self.latest_proposal)
    }

    fn get_latest_view_sync_certificate(&self) -> Result<Option<Vec<Vec<u8>>>, Error> {
        self.get_view_sync_certificate(self.latest_view_sync_certificate, 0)
    }

    fn get_view_sync_certificate(
        &self,
        view_number: u64,
        index: u64,
    ) -> Result<Option<Vec<Vec<u8>>>, Error> {
        let proposals = self.view_sync_certificates.get(&view_number);
        let mut ret_proposals = vec![];
        if let Some(cert) = proposals {
            for i in index..*self.view_sync_certificate_index.get(&view_number).unwrap() {
                ret_proposals.push(cert[usize::try_from(i).unwrap()].1.clone());
            }
        }
        if ret_proposals.is_empty() {
            Ok(None)
        } else {
            Ok(Some(ret_proposals))
        }
    }

    /// Return all votes the server has received for a particular view from provided index to most recent
    fn get_votes(&self, view_number: u64, index: u64) -> Result<Option<Vec<Vec<u8>>>, Error> {
        let votes = self.votes.get(&view_number);
        let mut ret_votes = vec![];
        if let Some(votes) = votes {
            for i in index..*self.vote_index.get(&view_number).unwrap() {
                ret_votes.push(votes[usize::try_from(i).unwrap()].1.clone());
            }
        }
        if ret_votes.is_empty() {
            Ok(None)
        } else {
            Ok(Some(ret_votes))
        }
    }

    /// Return all votes the server has received for a particular view from provided index to most recent
    fn get_upgrade_votes(
        &self,
        view_number: u64,
        index: u64,
    ) -> Result<Option<Vec<Vec<u8>>>, Error> {
        let votes = self.upgrade_votes.get(&view_number);
        let mut ret_votes = vec![];
        if let Some(votes) = votes {
            for i in index..*self.upgrade_vote_index.get(&view_number).unwrap() {
                ret_votes.push(votes[usize::try_from(i).unwrap()].1.clone());
            }
        }
        if ret_votes.is_empty() {
            Ok(None)
        } else {
            Ok(Some(ret_votes))
        }
    }

    /// Return all VID votes the server has received for a particular view from provided index to most recent
    fn get_vid_votes(&self, view_number: u64, index: u64) -> Result<Option<Vec<Vec<u8>>>, Error> {
        let vid_votes = self.vid_votes.get(&view_number);
        let mut ret_votes = vec![];
        if let Some(vid_votes) = vid_votes {
            for i in index..*self.vid_vote_index.get(&view_number).unwrap() {
                ret_votes.push(vid_votes[usize::try_from(i).unwrap()].1.clone());
            }
        }
        if ret_votes.is_empty() {
            Ok(None)
        } else {
            Ok(Some(ret_votes))
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
            for i in index..*self.view_sync_vote_index.get(&view_number).unwrap() {
                ret_votes.push(votes[usize::try_from(i).unwrap()].1.clone());
            }
        }
        if ret_votes.is_empty() {
            Ok(None)
        } else {
            Ok(Some(ret_votes))
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
            usize::try_from(self.num_txns).unwrap() - MAX_TXNS
        };

        let starting_index = if (usize::try_from(index).unwrap()) < lowest_in_memory_txs {
            lowest_in_memory_txs
        } else {
            usize::try_from(index).unwrap()
        };

        for idx in starting_index..=self.num_txns.try_into().unwrap() {
            if let Some(txn) = self.transactions.get(&(idx as u64)) {
                txns_to_return.push(txn.clone());
            }
            if txns_to_return.len() >= usize::try_from(TX_BATCH_SIZE).unwrap() {
                break;
            }
        }

        if txns_to_return.is_empty() {
            Err(ServerError {
                // TODO ED: Why does NoContent status code cause errors?
                status: StatusCode::NotImplemented,
                message: format!("Transaction not found for index {index}"),
            })
        } else {
            debug!("Returning this many txs {}", txns_to_return.len());
            //starting_index is the oldest index of the returned txns
            Ok(Some((starting_index as u64, txns_to_return)))
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

    /// Return the VID certificate the server has received for a particular view
    fn get_vid_certificate(&self, index: u64) -> Result<Option<Vec<Vec<u8>>>, Error> {
        match self.vid_certificates.get(&index) {
            Some(vid_cert) => {
                if vid_cert.1.is_empty() {
                    Err(ServerError {
                        status: StatusCode::NotImplemented,
                        message: format!("VID Certificate not found for view {index}"),
                    })
                } else {
                    Ok(Some(vec![vid_cert.1.clone()]))
                }
            }
            None => Err(ServerError {
                status: StatusCode::NotImplemented,
                message: format!("VID certificate not found for view {index}"),
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

        // don't accept the vote if it is too old
        if self.oldest_vote > view_number {
            return Err(ServerError {
                status: StatusCode::Gone,
                message: "Posted vote is too old".to_string(),
            });
        }

        let next_index = self.vote_index.entry(view_number).or_insert(0);
        self.votes
            .entry(view_number)
            .and_modify(|current_votes| current_votes.push((*next_index, vote.clone())))
            .or_insert_with(|| vec![(*next_index, vote)]);
        self.vote_index
            .entry(view_number)
            .and_modify(|index| *index += 1);
        Ok(())
    }

    /// Stores a received vote in the `WebServerState`
    fn post_upgrade_vote(&mut self, view_number: u64, vote: Vec<u8>) -> Result<(), Error> {
        // Only keep vote history for MAX_VIEWS number of views
        if self.upgrade_votes.len() >= MAX_VIEWS {
            self.upgrade_votes.remove(&self.oldest_upgrade_vote);
            while !self.upgrade_votes.contains_key(&self.oldest_upgrade_vote) {
                self.oldest_upgrade_vote += 1;
            }
        }

        // don't accept the vote if it is too old
        if self.oldest_upgrade_vote > view_number {
            return Err(ServerError {
                status: StatusCode::Gone,
                message: "Posted vote is too old".to_string(),
            });
        }

        let next_index = self.upgrade_vote_index.entry(view_number).or_insert(0);
        self.upgrade_votes
            .entry(view_number)
            .and_modify(|current_votes| current_votes.push((*next_index, vote.clone())))
            .or_insert_with(|| vec![(*next_index, vote)]);
        self.upgrade_vote_index
            .entry(view_number)
            .and_modify(|index| *index += 1);
        Ok(())
    }

    /// Stores a received VID vote in the `WebServerState`
    fn post_vid_vote(&mut self, view_number: u64, vote: Vec<u8>) -> Result<(), Error> {
        // Only keep vote history for MAX_VIEWS number of views
        if self.vid_votes.len() >= MAX_VIEWS {
            self.vid_votes.remove(&self.oldest_vote);
            while !self.vid_votes.contains_key(&self.oldest_vid_vote) {
                self.oldest_vid_vote += 1;
            }
        }

        // don't accept the vote if it is too old
        if self.oldest_vid_vote > view_number {
            return Err(ServerError {
                status: StatusCode::Gone,
                message: "Posted vid vote is too old".to_string(),
            });
        }

        let next_index = self.vid_vote_index.entry(view_number).or_insert(0);
        self.vid_votes
            .entry(view_number)
            .and_modify(|current_votes| current_votes.push((*next_index, vote.clone())))
            .or_insert_with(|| vec![(*next_index, vote)]);
        self.vid_vote_index
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

        // don't accept the vote if it is too old
        if self.oldest_view_sync_vote > view_number {
            return Err(ServerError {
                status: StatusCode::Gone,
                message: "Posted view sync vote is too old".to_string(),
            });
        }

        let next_index = self.view_sync_vote_index.entry(view_number).or_insert(0);
        self.view_sync_votes
            .entry(view_number)
            .and_modify(|current_votes| current_votes.push((*next_index, vote.clone())))
            .or_insert_with(|| vec![(*next_index, vote)]);
        self.view_sync_vote_index
            .entry(view_number)
            .and_modify(|index| *index += 1);
        Ok(())
    }
    /// Stores a received proposal in the `WebServerState`
    fn post_proposal(&mut self, view_number: u64, mut proposal: Vec<u8>) -> Result<(), Error> {
        info!("Received proposal for view {}", view_number);

        if view_number > self.latest_proposal {
            self.latest_proposal = view_number;
        }

        // Only keep proposal history for MAX_VIEWS number of view
        if self.proposals.len() >= MAX_VIEWS {
            self.proposals.pop_first();
        }
        self.proposals
            .entry(view_number)
            .and_modify(|(_, empty_proposal)| empty_proposal.append(&mut proposal))
            .or_insert_with(|| (String::new(), proposal));
        Ok(())
    }

    fn post_upgrade_proposal(
        &mut self,
        view_number: u64,
        mut proposal: Vec<u8>,
    ) -> Result<(), Error> {
        tracing::error!("Received upgrade proposal for view {}", view_number);

        if self.upgrade_proposals.len() >= MAX_VIEWS {
            self.upgrade_proposals.pop_first();
        }

        self.upgrade_proposals
            .entry(view_number)
            .and_modify(|(_, empty_proposal)| empty_proposal.append(&mut proposal))
            .or_insert_with(|| (String::new(), proposal));
        Ok(())
    }

    fn post_vid_disperse(&mut self, view_number: u64, mut disperse: Vec<u8>) -> Result<(), Error> {
        info!("Received VID disperse for view {}", view_number);
        if view_number > self.recent_vid_disperse {
            self.recent_vid_disperse = view_number;
        }

        // Only keep proposal history for MAX_VIEWS number of view
        if self.vid_disperses.len() >= MAX_VIEWS {
            self.vid_disperses.remove(&self.oldest_vid_disperse);
            while !self.vid_disperses.contains_key(&self.oldest_vid_disperse) {
                self.oldest_vid_disperse += 1;
            }
        }
        self.vid_disperses
            .entry(view_number)
            .and_modify(|(_, empty_proposal)| empty_proposal.append(&mut disperse))
            .or_insert_with(|| (String::new(), disperse));
        Ok(())
    }

    fn post_view_sync_certificate(
        &mut self,
        view_number: u64,
        proposal: Vec<u8>,
    ) -> Result<(), Error> {
        if view_number > self.latest_view_sync_certificate {
            self.latest_view_sync_certificate = view_number;
        }

        // Only keep proposal history for MAX_VIEWS number of view
        if self.view_sync_certificates.len() >= MAX_VIEWS {
            self.view_sync_certificates.pop_first();
        }
        let next_index = self
            .view_sync_certificate_index
            .entry(view_number)
            .or_insert(0);
        self.view_sync_certificates
            .entry(view_number)
            .and_modify(|current_props| current_props.push((*next_index, proposal.clone())))
            .or_insert_with(|| vec![(*next_index, proposal)]);
        self.view_sync_certificate_index
            .entry(view_number)
            .and_modify(|index| *index += 1);
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

    fn post_vid_certificate(
        &mut self,
        view_number: u64,
        mut certificate: Vec<u8>,
    ) -> Result<(), Error> {
        info!("Received VID Certificate for view {}", view_number);

        // Only keep proposal history for MAX_VIEWS number of view
        if self.vid_certificates.len() >= MAX_VIEWS {
            self.vid_certificates.remove(&self.oldest_vid_certificate);
            while !self
                .vid_certificates
                .contains_key(&self.oldest_vid_certificate)
            {
                self.oldest_vid_certificate += 1;
            }
        }
        self.vid_certificates
            .entry(view_number)
            .and_modify(|(_, empty_cert)| empty_cert.append(&mut certificate))
            .or_insert_with(|| (String::new(), certificate));
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
        let new_key = KEY::from_bytes(&key).map_err(|_| ServerError {
            status: StatusCode::BadRequest,
            message: "Only signature keys can be added to stake table".to_string(),
        })?;
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
            self.proposals.pop_first();
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

/// configurability options for the web server
#[derive(Args, Default)]
pub struct Options {
    #[arg(long = "web-server-api-path", env = "WEB_SERVER_API_PATH")]
    /// path to API
    pub api_path: Option<PathBuf>,
}

/// Sets up all API routes
/// This web server incorporates the protocol version within each message.
/// Transport versioning (generic params here) only changes when the web-CDN itself changes.
/// When transport versioning changes, the application itself must update its version.
#[allow(clippy::too_many_lines)]
fn define_api<State, KEY, NetworkVersion: StaticVersionType>(
    options: &Options,
) -> Result<Api<State, Error, NetworkVersion>, ApiError>
where
    State: 'static + Send + Sync + ReadState + WriteState,
    <State as ReadState>::State: Send + Sync + WebServerDataSource<KEY>,
    KEY: SignatureKey,
    NetworkVersion: 'static,
{
    let mut api = match &options.api_path {
        Some(path) => Api::<State, Error, NetworkVersion>::from_file(path)?,
        None => {
            let toml: toml::Value = toml::from_str(include_str!("../api.toml")).map_err(|err| {
                ApiError::CannotReadToml {
                    reason: err.to_string(),
                }
            })?;
            Api::<State, Error, NetworkVersion>::new(toml)?
        }
    };
    api.get("getproposal", |req, state| {
        async move {
            let view_number: u64 = req.integer_param("view_number")?;
            state.get_proposal(view_number)
        }
        .boxed()
    })?
    .get("get_upgrade_proposal", |req, state| {
        async move {
            let view_number: u64 = req.integer_param("view_number")?;
            state.get_upgrade_proposal(view_number)
        }
        .boxed()
    })?
    .get("getviddisperse", |req, state| {
        async move {
            let view_number: u64 = req.integer_param("view_number")?;
            state.get_vid_disperse(view_number)
        }
        .boxed()
    })?
    .get("get_latest_proposal", |_req, state| {
        async move { state.get_latest_proposal() }.boxed()
    })?
    .get("get_latest_view_sync_certificate", |_req, state| {
        async move { state.get_latest_view_sync_certificate() }.boxed()
    })?
    .get("getviewsynccertificate", |req, state| {
        async move {
            let view_number: u64 = req.integer_param("view_number")?;
            let index: u64 = req.integer_param("index")?;
            state.get_view_sync_certificate(view_number, index)
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
    .get("get_upgrade_votes", |req, state| {
        async move {
            let view_number: u64 = req.integer_param("view_number")?;
            let index: u64 = req.integer_param("index")?;
            state.get_upgrade_votes(view_number, index)
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
    .post("post_upgrade_vote", |req, state| {
        async move {
            let view_number: u64 = req.integer_param("view_number")?;
            // Using body_bytes because we don't want to deserialize; body_auto or body_json deserializes automatically
            let vote = req.body_bytes();
            state.post_upgrade_vote(view_number, vote)
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
    .post("post_upgrade_proposal", |req, state| {
        async move {
            let view_number: u64 = req.integer_param("view_number")?;
            let proposal = req.body_bytes();
            state.post_upgrade_proposal(view_number, proposal)
        }
        .boxed()
    })?
    .post("postviddisperse", |req, state| {
        async move {
            let view_number: u64 = req.integer_param("view_number")?;
            let disperse = req.body_bytes();
            state.post_vid_disperse(view_number, disperse)
        }
        .boxed()
    })?
    .post("postviewsynccertificate", |req, state| {
        async move {
            let view_number: u64 = req.integer_param("view_number")?;
            let proposal = req.body_bytes();
            state.post_view_sync_certificate(view_number, proposal)
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
                                "Wrong secret value for proposal for view {view_number:?}"
                            ),
                        })
                    }
                } else {
                    Err(ServerError {
                        status: StatusCode::BadRequest,
                        message: format!("Proposal already submitted for view {view_number:?}"),
                    })
                }
            } else {
                Err(ServerError {
                    status: StatusCode::BadRequest,
                    message: format!("No endpoint for view number {view_number:?} yet"),
                })
            }
        }
        .boxed()
    })?;
    Ok(api)
}

/// run the web server
/// # Errors
/// TODO
/// this looks like it will panic not error
/// # Panics
/// on errors creating or registering the tide disco api
pub async fn run_web_server<
    KEY: SignatureKey + 'static,
    NetworkVersion: StaticVersionType + 'static,
>(
    shutdown_listener: Option<OneShotReceiver<()>>,
    url: Url,
    bind_version: NetworkVersion,
) -> io::Result<()> {
    let options = Options::default();

    let web_api = define_api(&options).unwrap();
    let state = State::new(WebServerState::new().with_shutdown_signal(shutdown_listener));
    let mut app = App::<State<KEY>, Error>::with_state(state);

    app.register_module::<Error, NetworkVersion>("api", web_api)
        .unwrap();

    let app_future = app.serve(url, bind_version);

    app_future.await
}
