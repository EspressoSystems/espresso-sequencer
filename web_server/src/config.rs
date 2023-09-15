pub const DEFAULT_WEB_SERVER_PORT: u16 = 9000;
pub const DEFAULT_WEB_SERVER_DA_PORT: u16 = 9001;
pub const DEFAULT_WEB_SERVER_VIEW_SYNC_PORT: u16 = 9002;

/// How many views to keep in memory
pub const MAX_VIEWS: usize = 25;
/// How many transactions to keep in memory
pub const MAX_TXNS: usize = 500;
/// How many transactions to return at once
pub const TX_BATCH_SIZE: u64 = 1;

pub fn get_proposal_route(view_number: u64) -> String {
    format!("api/proposal/{view_number}")
}

pub fn post_proposal_route(view_number: u64) -> String {
    format!("api/proposal/{view_number}")
}

pub fn get_recent_proposal_route() -> String {
    "api/proposal".to_string()
}

pub fn get_da_certificate_route(view_number: u64) -> String {
    format!("api/certificate/{view_number}")
}

pub fn post_da_certificate_route(view_number: u64) -> String {
    format!("api/certificate/{view_number}")
}

pub fn get_vote_route(view_number: u64, index: u64) -> String {
    format!("api/votes/{view_number}/{index}")
}

pub fn post_vote_route(view_number: u64) -> String {
    format!("api/votes/{view_number}")
}

pub fn get_vid_disperse_route(view_number: u64) -> String {
    format!("api/vid/disperse/{view_number}")
}

pub fn post_vid_disperse_route(view_number: u64) -> String {
    format!("api/vid/disperse/{view_number}")
}

pub fn get_vid_vote_route(view_number: u64, index: u64) -> String {
    format!("api/vid/votes/{view_number}/{index}")
}

pub fn post_vid_vote_route(view_number: u64) -> String {
    format!("api/vid/votes/{view_number}")
}

pub fn get_vid_cert_route(view_number: u64) -> String {
    format!("api/vid/cert/{view_number}")
}

pub fn post_vid_cert_route(view_number: u64) -> String {
    format!("api/vid/cert/{view_number}")
}

pub fn get_transactions_route(index: u64) -> String {
    format!("api/transactions/{index}")
}

pub fn post_transactions_route() -> String {
    "api/transactions".to_string()
}

pub fn post_staketable_route() -> String {
    "api/staketable".to_string()
}

pub fn post_view_sync_proposal_route(view_number: u64) -> String {
    format!("api/view_sync_proposal/{view_number}")
}

pub fn get_view_sync_proposal_route(view_number: u64, index: u64) -> String {
    format!("api/view_sync_proposal/{view_number}/{index}")
}

pub fn post_view_sync_vote_route(view_number: u64) -> String {
    format!("api/view_sync_vote/{view_number}")
}

pub fn get_view_sync_vote_route(view_number: u64, index: u64) -> String {
    format!("api/view_sync_vote/{view_number}/{index}")
}
