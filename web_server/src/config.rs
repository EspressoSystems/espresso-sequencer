/// the default port on which to run the web server
pub const DEFAULT_WEB_SERVER_PORT: u16 = 9000;
/// the default port on which to serve Data availability functionality
pub const DEFAULT_WEB_SERVER_DA_PORT: u16 = 9001;
/// the default port on which to serve View Sync functionality
pub const DEFAULT_WEB_SERVER_VIEW_SYNC_PORT: u16 = 9002;

/// How many views to keep in memory
pub const MAX_VIEWS: usize = 100;
/// How many transactions to keep in memory
pub const MAX_TXNS: usize = 500;
/// How many transactions to return at once
pub const TX_BATCH_SIZE: u64 = 1;

/// get proposal
#[must_use]
pub fn get_proposal_route(view_number: u64) -> String {
    format!("api/proposal/{view_number}")
}

/// post proposal
#[must_use]
pub fn post_proposal_route(view_number: u64) -> String {
    format!("api/proposal/{view_number}")
}

/// get latest qc
#[must_use]
pub fn get_latest_proposal_route() -> String {
    "api/proposal/latest".to_string()
}

/// get latest view sync proposal
#[must_use]
pub fn get_latest_view_sync_certificate_route() -> String {
    "api/view_sync_certificate/latest".to_string()
}

/// get latest certificate
#[must_use]
pub fn get_da_certificate_route(view_number: u64) -> String {
    format!("api/certificate/{view_number}")
}

/// post data availability certificate
#[must_use]
pub fn post_da_certificate_route(view_number: u64) -> String {
    format!("api/certificate/{view_number}")
}

/// get vote
#[must_use]
pub fn get_vote_route(view_number: u64, index: u64) -> String {
    format!("api/votes/{view_number}/{index}")
}

/// post vote
#[must_use]
pub fn post_vote_route(view_number: u64) -> String {
    format!("api/votes/{view_number}")
}

/// get upgrade votes
#[must_use]
pub fn get_upgrade_vote_route(view_number: u64, index: u64) -> String {
    format!("api/upgrade_votes/{view_number}/{index}")
}

/// post vote
#[must_use]
pub fn post_upgrade_vote_route(view_number: u64) -> String {
    format!("api/upgrade_votes/{view_number}")
}

/// get vid dispersal
#[must_use]
pub fn get_vid_disperse_route(view_number: u64) -> String {
    format!("api/vid_disperse/{view_number}")
}

/// post vid dispersal
#[must_use]
pub fn post_vid_disperse_route(view_number: u64) -> String {
    format!("api/vid_disperse/{view_number}")
}

/// get upgrade proposal
#[must_use]
pub fn get_upgrade_proposal_route(view_number: u64) -> String {
    format!("api/upgrade_proposal/{view_number}")
}

/// post upgrade proposal
#[must_use]
pub fn post_upgrade_proposal_route(view_number: u64) -> String {
    format!("api/upgrade_proposal/{view_number}")
}

/// get vid vote route
#[must_use]
pub fn get_vid_vote_route(view_number: u64, index: u64) -> String {
    format!("api/vid_votes/{view_number}/{index}")
}

/// post vid vote route
#[must_use]
pub fn post_vid_vote_route(view_number: u64) -> String {
    format!("api/vid_votes/{view_number}")
}

/// get vid certificate
#[must_use]
pub fn get_vid_certificate_route(view_number: u64) -> String {
    format!("api/vid_certificate/{view_number}")
}

/// post vid certificate
#[must_use]
pub fn post_vid_certificate_route(view_number: u64) -> String {
    format!("api/vid_certificate/{view_number}")
}

/// get transactions
#[must_use]
pub fn get_transactions_route(index: u64) -> String {
    format!("api/transactions/{index}")
}

/// post transactions
#[must_use]
pub fn post_transactions_route() -> String {
    "api/transactions".to_string()
}

/// post stake table
#[must_use]
pub fn post_staketable_route() -> String {
    "api/staketable".to_string()
}

/// post view sync proposal
#[must_use]
pub fn post_view_sync_certificate_route(view_number: u64) -> String {
    format!("api/view_sync_certificate/{view_number}")
}

/// get view sync proposal
#[must_use]
pub fn get_view_sync_certificate_route(view_number: u64, index: u64) -> String {
    format!("api/view_sync_certificate/{view_number}/{index}")
}

/// post view sync vote
#[must_use]
pub fn post_view_sync_vote_route(view_number: u64) -> String {
    format!("api/view_sync_vote/{view_number}")
}

/// get view sync vote
#[must_use]
pub fn get_view_sync_vote_route(view_number: u64, index: u64) -> String {
    format!("api/view_sync_vote/{view_number}/{index}")
}
