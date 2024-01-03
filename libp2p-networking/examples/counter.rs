//! This is dead code, do not use
// pub mod common;
//
// use async_compatibility_layer::art::async_main;
// use clap::Parser;
// use color_eyre::eyre::Result;
// #[cfg(all(feature = "lossy_network", target_os = "linux"))]
// use common::{
//     lossy_network::{IsolationConfig, LossyNetworkBuilder},
//     ExecutionEnvironment,
// };
// use common::{start_main, CliOpt};
// use tracing::instrument;
//
// #[async_main]
// #[instrument]
// async fn main() -> Result<()> {
// fn main() -> Result<(), ()> {
    //     let args = CliOpt::parse();
    //
    //     #[cfg(all(feature = "lossy_network", target_os = "linux"))]
    //     let network = {
    //         use crate::common::lossy_network::LOSSY_QDISC;
    //         let mut builder = LossyNetworkBuilder::default();
    //         builder
    //             .env_type(args.env_type_delegate.env_type)
    //             .netem_config(LOSSY_QDISC);
    //         match args.env_type_delegate.env_type {
    //             ExecutionEnvironment::Docker => {
    //                 builder.eth_name("eth0".to_string()).isolation_config(None)
    //             }
    //             ExecutionEnvironment::Metal => builder
    //                 .eth_name("ens5".to_string())
    //                 .isolation_config(Some(IsolationConfig::default())),
    //         };
    //         builder.build()
    //     }?;
    //
    //     #[cfg(all(feature = "lossy_network", target_os = "linux"))]
    //     {
    //         network.isolate().await?;
    //         network.create_qdisc().await?;
    //     }
    //
    //     start_main(args).await?;
    //
    //     #[cfg(all(feature = "lossy_network", target_os = "linux"))]
    //     {
    //         // implicitly deletes qdisc in the case of metal run
    //         // leaves qdisc alive in docker run with expectation docker does cleanup
    //         network.undo_isolate().await?;
    //     }
    //
//     Ok(())
// }
//
/// dead code
fn main() {}
