// TODO these should be config options for lossy network
// #![allow(clippy::type_complexity)]
// use hotshot_testing::{
//     network_reliability::{AsynchronousNetwork, PartiallySynchronousNetwork, SynchronousNetwork},
//     test_builder::{TestBuilder, TestMetadata},
//     test_types::{StaticCommitteeTestTypes, StaticNodeImplType},
// };
// use std::sync::Arc;
// use tracing::instrument;
//
// // tests base level of working synchronous network
// #[cfg_attr(
//     feature = "tokio-executor",
//     tokio::test(flavor = "multi_thread", worker_threads = 2)
// )]
// #[cfg_attr(feature = "async-std-executor", async_std::test)]
// #[instrument]
// async fn test_no_loss_network() {
//     let builder = TestBuilder {
//         metadata: TestMetadata {
//             total_nodes: 10,
//             start_nodes: 10,
//             network_reliability: Some(Arc::new(SynchronousNetwork::default())),
//             ..TestMetadata::default()
//         },
//         ..Default::default()
//     };
//     builder
//         .build::<StaticCommitteeTestTypes, StaticNodeImplType>()
//         .launch()
//         .run_test()
//         .await
//         .unwrap();
// }
//
// // // tests network with forced packet delay
// #[cfg_attr(
//     feature = "tokio-executor",
//     tokio::test(flavor = "multi_thread", worker_threads = 2)
// )]
// #[cfg_attr(feature = "async-std-executor", async_std::test)]
// #[instrument]
// async fn test_synchronous_network() {
//     let builder = TestBuilder {
//         metadata: TestMetadata {
//             total_nodes: 5,
//             start_nodes: 5,
//             num_succeeds: 2,
//             ..TestMetadata::default()
//         },
//         ..Default::default()
//     };
//     builder
//         .build::<StaticCommitteeTestTypes, StaticNodeImplType>()
//         .launch()
//         .run_test()
//         .await
//         .unwrap();
// }
//
// // tests network with small packet delay and dropped packets
// #[cfg_attr(
//     feature = "tokio-executor",
//     tokio::test(flavor = "multi_thread", worker_threads = 2)
// )]
// #[cfg_attr(feature = "async-std-executor", async_std::test)]
// #[instrument]
// #[ignore]
// async fn test_asynchronous_network() {
//     let builder = TestBuilder {
//         metadata: TestMetadata {
//             total_nodes: 5,
//             start_nodes: 5,
//             num_succeeds: 2,
//             failure_threshold: 5,
//             network_reliability: Some(Arc::new(AsynchronousNetwork::new(97, 100, 0, 5))),
//             ..TestMetadata::default()
//         },
//         ..Default::default()
//     };
//     builder
//         .build::<StaticCommitteeTestTypes, StaticNodeImplType>()
//         .launch()
//         .run_test()
//         .await
//         .unwrap();
// }
//
// /// tests network with asynchronous patch that eventually becomes synchronous
// #[cfg_attr(
//     feature = "tokio-executor",
//     tokio::test(flavor = "multi_thread", worker_threads = 2)
// )]
// #[cfg_attr(feature = "async-std-executor", async_std::test)]
// #[instrument]
// #[ignore]
// async fn test_partially_synchronous_network() {
//     let asn = AsynchronousNetwork::new(90, 100, 0, 0);
//     let sn = SynchronousNetwork::new(10, 0);
//     let gst = std::time::Duration::new(10, 0);
//
//     let builder = TestBuilder {
//         metadata: TestMetadata {
//             total_nodes: 5,
//             start_nodes: 5,
//             num_succeeds: 2,
//             network_reliability: Some(Arc::new(PartiallySynchronousNetwork::new(asn, sn, gst))),
//             ..TestMetadata::default()
//         },
//         ..Default::default()
//     };
//     builder
//         .build::<StaticCommitteeTestTypes, StaticNodeImplType>()
//         .launch()
//         .run_test()
//         .await
//         .unwrap();
// }
