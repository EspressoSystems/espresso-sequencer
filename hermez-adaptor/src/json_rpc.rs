use std::sync::Arc;

use crate::Options;
use ethers::{
    types::{Bytes, H256},
    utils::keccak256,
};
use http_types::{headers::HeaderValue, Url};
use jsonrpc_v2::{Data, Error as RpcError, MapRouter, Params, RequestObject, Server};
use sequencer::{Transaction, VmId};
use surf_disco::error::ClientError;
use tide::security::{CorsMiddleware, Origin};

pub type RpcApiService = Arc<Server<MapRouter>>;
pub type RpcServer = tide::Server<RpcApiService>;
pub type RpcServerRequest = tide::Request<RpcApiService>;

pub type RpcData = (Url, VmId);

/// Handle incoming HTTP JSON RPC requests.
pub async fn handle_http_request(mut request: RpcServerRequest) -> tide::Result {
    // Parse RPC request
    let rpc_request: RequestObject = match request.body_json().await {
        Ok(result) => result,
        Err(err) => {
            println!("Error: {err}");
            return Err(err);
        }
    };
    tracing::trace!("Request: {}", serde_json::to_string(&rpc_request)?);

    // Handle RPC request
    let rpc_server = request.state();
    let rpc_result = rpc_server.handle(rpc_request).await;

    // Serialize response to JSON
    let rpc_result_json = serde_json::to_string(&rpc_result)?;

    tracing::trace!("Response: {}", rpc_result_json);

    // Respond with RPC result
    let response = tide::Response::builder(http_types::StatusCode::Ok)
        .body(rpc_result_json)
        .content_type("application/json-rpc;charset=utf-8")
        .build();

    Ok(response)
}

/// Build HTTP and WebSocket server both exposing a JSON RPC API.
pub fn build_rpc_server(api: RpcApiService) -> RpcServer {
    // Configure CORS middleware
    let cors = CorsMiddleware::new()
        .allow_methods("GET, POST, OPTIONS".parse::<HeaderValue>().unwrap())
        .allow_origin(Origin::from("*"))
        .allow_credentials(false);

    // Prepare HTTP server with RPC route
    let mut app = tide::with_state(api);
    app.with(cors);
    app.at("/").post(handle_http_request);
    app
}

pub async fn eth_send_raw_transaction(
    data: Data<RpcData>,
    Params((raw_tx,)): Params<(Bytes,)>,
) -> Result<H256, RpcError> {
    let url = (*data).0.clone();
    let vmid = data.1;

    let client = surf_disco::Client::<ClientError>::new(url.join("submit").unwrap());

    client.connect(None).await;

    let txn = Transaction::new(vmid, raw_tx.to_vec());

    client
        .post::<()>("submit")
        .body_json(&txn)
        .unwrap()
        .send()
        .await
        .unwrap();

    Ok(keccak256(raw_tx).into())
}

pub async fn serve(opt: &Options) {
    let rpc_data: RpcData = (opt.sequencer_url.clone(), opt.l2_chain_id.into());

    let rpc = Server::new()
        .with_data(Data::new(rpc_data))
        .with_method("eth_sendRawTransaction", eth_send_raw_transaction)
        .finish();

    let server = build_rpc_server(rpc);
    server
        .listen(&format!("http://0.0.0.0:{}", opt.rpc_port))
        .await
        .unwrap();
}
