use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use clap::Parser;
use futures::join;
use hermez_adaptor::{json_rpc, query_service, Options};

#[async_std::main]
async fn main() {
    setup_logging();
    setup_backtrace();

    let opt = Options::parse();
    join!(json_rpc::serve(&opt), query_service::serve(&opt),);
}
