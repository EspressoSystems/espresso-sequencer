use std::sync::Arc;

use crate::Options;
use ethers::types::{Bytes, H256};
use http_types::headers::HeaderValue;
use jsonrpc_v2::{Data, Error as RpcError, MapRouter, Params, RequestObject, Server};
use surf_disco::{error::ClientError, Client};
use tide::security::{CorsMiddleware, Origin};

pub type RpcApiService = Arc<Server<MapRouter>>;
pub type RpcServer = tide::Server<RpcApiService>;
pub type RpcServerRequest = tide::Request<RpcApiService>;

pub type ClientType = Client<ClientError>;

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
    app.at("/")
        // .with(WebSocket::new(handle_ws_request))
        .get(|_| async { Ok("Used HTTP Method is not allowed. POST or OPTIONS is required") })
        .post(handle_http_request);
    app
}

pub async fn serve(opt: &Options) {
    // Use client as RPC server data?
    let _client = surf_disco::Client::<ClientError>::new(opt.sequencer_url.join("submit").unwrap());

    let rpc = Server::new()
        .with_method("eth_send_raw_transaction", eth_send_raw_transaction)
        .finish();

    let server = build_rpc_server(rpc);
    server
        .listen(&format!("http://127.0.0.1:{}", opt.port))
        .await
        .unwrap();
}

pub async fn eth_send_raw_transaction(
    _data: Data<ClientType>,
    Params((_raw_tx,)): Params<(Bytes,)>,
) -> Result<H256, RpcError> {
    // TODO
    Ok(H256([0u8; 32]))
}
