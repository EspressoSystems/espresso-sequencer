use std::sync::{Arc, RwLock};

use node_metrics::api::node_validator;
use tide_disco::App;
use vbs::version::{StaticVersion, Version};

/// CONSTANT for protocol major version
pub const VERSION_MAJ: u16 = 0;

/// CONSTANT for protocol major version
pub const VERSION_MIN: u16 = 1;

pub const VERSION_0_1: Version = Version {
    major: VERSION_MAJ,
    minor: VERSION_MIN,
};

/// Constant for the base protocol version in this instance of HotShot.
pub const BASE_VERSION: Version = VERSION_0_1;

/// Type for protocol static version 0.1.
pub type Version01 = StaticVersion<VERSION_MAJ, VERSION_MIN>;

/// This represents the latest version of this service. This will likely
/// always be whatever the max API version that's being served is.
pub const SERVICE_VER_0_1: Version01 = StaticVersion {};

/// The client definition for the Push CDN. Uses the Quic
/// protocol and no middleware. Differs from the user
/// definition in that is on the client-side.
#[derive(Clone)]
pub struct ClientDef;

struct State {}

/// ClientConnectionMessage is a message that indicates when a client is
/// connecting or disconnecting from the service. This message is used
/// to signify when the client arrives or leaves.
pub enum ClientConnectionMessage {
    Connected,
    Disconnected,
}

#[async_std::main]
async fn main() {
    // We have two separate states we want to maintain as much as possible.
    // The first is the Data State, which contains all of the recorded state
    // we want to keep track of and to be able to relay at a moment's notice.
    // The second is a state of the connected clients.  This state should be
    // able to be read from and written to indirectly by the clients.

    let state = Arc::new(RwLock::new(State {}));

    let mut app = App::<_, node_validator::v0::Error>::with_state(state);
    let node_validator_api_v0 = node_validator::v0::define_api().expect("api to be defined");
    app.register_module("node-validator", node_validator_api_v0)
        .expect("register module");

    // Serve the app

    let url = format!("0.0.0.0:9000");
    app.serve(&url, SERVICE_VER_0_1)
        .await
        .expect("app to be served");
}
