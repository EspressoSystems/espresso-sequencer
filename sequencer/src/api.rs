use crate::{network, Node, SeqTypes};
use async_std::task::JoinHandle;
use data_source::SubmitDataSource;
use hotshot::types::SystemContextHandle;
use hotshot_query_service::data_source::ExtensibleDataSource;

mod data_source;
mod endpoints;
pub mod fs;
pub mod options;
mod update;

use hotshot_types::light_client::StateKeyPair;
pub use options::Options;

type NodeIndex = u64;

pub type Consensus<N> = SystemContextHandle<SeqTypes, Node<N>>;

pub struct SequencerNode<N: network::Type> {
    pub handle: Consensus<N>,
    pub update_task: JoinHandle<anyhow::Result<()>>,
    pub node_index: NodeIndex,
    pub state_key_pair: StateKeyPair,
}

type AppState<N, D> = ExtensibleDataSource<D, Consensus<N>>;

impl<N: network::Type, D> SubmitDataSource<N> for AppState<N, D> {
    fn handle(&self) -> &Consensus<N> {
        self.as_ref()
    }
}

impl<N: network::Type> SubmitDataSource<N> for Consensus<N> {
    fn handle(&self) -> &Consensus<N> {
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        testing::{
            init_hotshot_handles, init_hotshot_handles_with_metrics, wait_for_decide_on_handle,
        },
        Header, Transaction, VmId,
    };
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use async_std::task::sleep;
    use commit::Committable;
    use endpoints::TimeWindowQueryData;
    use futures::FutureExt;
    use hotshot_query_service::availability::BlockQueryData;
    use portpicker::pick_unused_port;
    use std::time::Duration;
    use surf_disco::Client;
    use tempfile::TempDir;
    use tide_disco::{app::AppHealth, error::ServerError, healthcheck::HealthStatus};

    #[async_std::test]
    async fn test_healthcheck() {
        setup_logging();
        setup_backtrace();

        let port = pick_unused_port().expect("No ports free");
        let url = format!("http://localhost:{port}").parse().unwrap();
        let client: Client<ServerError> = Client::new(url);

        let handles = init_hotshot_handles().await;
        for handle in handles.iter() {
            handle.hotshot.start_consensus().await;
        }

        let options = Options::from(options::Http { port });
        options
            .serve(|_| async move { (handles[0].clone(), 0, Default::default()) }.boxed())
            .await
            .unwrap();

        client.connect(None).await;
        let health = client.get::<AppHealth>("healthcheck").send().await.unwrap();
        assert_eq!(health.status, HealthStatus::Available);
    }

    #[async_std::test]
    async fn submit_test_with_query_module() {
        let tmp_dir = TempDir::new().unwrap();
        let storage_path = tmp_dir.path().join("tmp_storage");
        submit_test_helper(Some(options::Fs {
            storage_path,
            reset_store: true,
        }))
        .await
    }

    #[async_std::test]
    async fn submit_test_without_query_module() {
        submit_test_helper(None).await
    }

    async fn submit_test_helper(query_opt: Option<options::Fs>) {
        setup_logging();
        setup_backtrace();

        let txn = Transaction::new(VmId(0), vec![1, 2, 3, 4]);

        let port = pick_unused_port().expect("No ports free");

        let url = format!("http://localhost:{port}").parse().unwrap();
        let client: Client<ServerError> = Client::new(url);

        // Get list of HotShot handles, take the first one, and submit a transaction to it
        let handles = init_hotshot_handles().await;
        for handle in handles.iter() {
            handle.hotshot.start_consensus().await;
        }

        let mut options = Options::from(options::Http { port }).submit(Default::default());
        if let Some(query) = query_opt {
            options = options.query_fs(query);
        }
        let SequencerNode { mut handle, .. } = options
            .serve(|_| async move { (handles[0].clone(), 0, Default::default()) }.boxed())
            .await
            .unwrap();
        let mut events = handle.get_event_stream(Default::default()).await.0;

        client.connect(None).await;

        client
            .post::<()>("submit/submit")
            .body_json(&txn)
            .unwrap()
            .send()
            .await
            .unwrap();

        // Wait for a Decide event containing transaction matching the one we sent
        wait_for_decide_on_handle(&mut events, &txn).await.unwrap()
    }

    #[async_std::test]
    async fn status_test_with_query_module() {
        let tmp_dir = TempDir::new().unwrap();
        let storage_path = tmp_dir.path().join("tmp_storage");
        status_test_helper(Some(options::Fs {
            storage_path,
            reset_store: true,
        }))
        .await
    }

    #[async_std::test]
    async fn status_test_without_query_module() {
        status_test_helper(None).await
    }

    async fn status_test_helper(query_opt: Option<options::Fs>) {
        setup_logging();
        setup_backtrace();

        let port = pick_unused_port().expect("No ports free");
        let url = format!("http://localhost:{port}").parse().unwrap();
        let client: Client<ServerError> = Client::new(url);

        let init_handle = |metrics: Box<dyn crate::Metrics>| {
            async move {
                let handles = init_hotshot_handles_with_metrics(&*metrics).await;
                for handle in &handles {
                    handle.hotshot.start_consensus().await;
                }
                (handles[0].clone(), 0, Default::default())
            }
            .boxed()
        };

        let mut options = Options::from(options::Http { port }).status(Default::default());
        if let Some(query) = query_opt {
            options = options.query_fs(query);
        }
        options.serve(init_handle).await.unwrap();
        client.connect(None).await;

        // The status API is well tested in the query service repo. Here we are just smoke testing
        // that we set it up correctly. Wait for a (non-genesis) block to be sequenced and then
        // check the success rate metrics.
        tracing::info!(
            "metrics {}",
            client.get::<String>("status/metrics").send().await.unwrap()
        );
        while client
            .get::<u64>("status/latest_block_height")
            .send()
            .await
            .unwrap()
            <= 1
        {
            sleep(Duration::from_secs(1)).await;
        }
        let success_rate = client
            .get::<f64>("status/success_rate")
            .send()
            .await
            .unwrap();
        // If metrics are populating correctly, we should get a finite number. If not, we might get
        // NaN or infinity due to division by 0.
        assert!(success_rate.is_finite(), "{success_rate}");
        // We know at least some views have been successful, since we finalized a block.
        assert!(success_rate > 0.0, "{success_rate}");
    }

    #[async_std::test]
    async fn test_timestamp_window() {
        setup_logging();
        setup_backtrace();

        // Start sequencer.
        let handles = init_hotshot_handles().await;
        for handle in handles.iter() {
            handle.hotshot.start_consensus().await;
        }

        // Start query service.
        let port = pick_unused_port().expect("No ports free");
        let tmp_dir = TempDir::new().unwrap();
        let storage_path = tmp_dir.path().join("tmp_storage");
        Options::from(options::Http { port })
            .query_fs(options::Fs {
                storage_path,
                reset_store: true,
            })
            .status(Default::default())
            .serve(|_| async move { (handles[0].clone(), 0, Default::default()) }.boxed())
            .await
            .unwrap();

        // Connect client.
        let client: Client<ServerError> =
            Client::new(format!("http://localhost:{port}").parse().unwrap());
        client.connect(None).await;

        // Wait for blocks with at least three different timestamps to be sequenced. This lets us
        // test all the edge cases.
        let mut test_blocks: Vec<Vec<Header>> = vec![];
        while test_blocks.len() < 3 {
            let num_blocks = test_blocks.iter().flatten().count();

            // Wait for the next block to be sequenced.
            while client
                .get::<usize>("status/latest_block_height")
                .send()
                .await
                .unwrap()
                < num_blocks + 1
            {
                sleep(Duration::from_secs(1)).await;
            }

            let block: BlockQueryData<SeqTypes> = client
                .get(&format!("availability/block/{num_blocks}"))
                .send()
                .await
                .unwrap();
            let header = block.header().clone();
            if let Some(last_timestamp) = test_blocks.last_mut() {
                if last_timestamp[0].timestamp == header.timestamp {
                    last_timestamp.push(header);
                } else {
                    test_blocks.push(vec![header]);
                }
            } else {
                test_blocks.push(vec![header]);
            }
        }
        tracing::info!("blocks for testing: {test_blocks:#?}");

        // Define invariants that every response should satisfy.
        let check_invariants = |res: &TimeWindowQueryData, start, end, check_prev| {
            let mut prev = res.prev.as_ref();
            if let Some(prev) = prev {
                if check_prev {
                    assert!(prev.timestamp < start);
                }
            } else {
                // `prev` can only be `None` if the first block in the window is the genesis block.
                assert_eq!(res.from, 0);
            };
            for header in &res.window {
                assert!(start <= header.timestamp);
                assert!(header.timestamp < end);
                if let Some(prev) = prev {
                    assert!(prev.timestamp <= header.timestamp);
                }
                prev = Some(header);
            }
            if let Some(next) = &res.next {
                assert!(next.timestamp >= end);
                // If there is a `next`, there must be at least one previous block (either `prev`
                // itself or the last block if the window is nonempty), so we can `unwrap` here.
                assert!(next.timestamp >= prev.unwrap().timestamp);
            }
        };

        let get_window = |start, end| {
            let client = client.clone();
            async move {
                let res = client
                    .get(&format!("availability/headers/window/{start}/{end}"))
                    .send()
                    .await
                    .unwrap();
                tracing::info!("window for timestamp range {start}-{end}: {res:#?}");
                check_invariants(&res, start, end, true);
                res
            }
        };

        // Case 0: happy path. All blocks are available, including prev and next.
        let start = test_blocks[1][0].timestamp;
        let end = start + 1;
        let res = get_window(start, end).await;
        assert_eq!(res.prev.unwrap(), *test_blocks[0].last().unwrap());
        assert_eq!(res.window, test_blocks[1]);
        assert_eq!(res.next.unwrap(), test_blocks[2][0]);

        // Case 1: no `prev`, start of window is before genesis.
        let start = 0;
        let end = test_blocks[0][0].timestamp + 1;
        let res = get_window(start, end).await;
        assert_eq!(res.prev, None);
        assert_eq!(res.window, test_blocks[0]);
        assert_eq!(res.next.unwrap(), test_blocks[1][0]);

        // Case 2: no `next`, end of window is after the most recently sequenced block.
        let start = test_blocks[2][0].timestamp;
        let end = u64::MAX;
        let res = get_window(start, end).await;
        assert_eq!(res.prev.unwrap(), *test_blocks[1].last().unwrap());
        // There may have been more blocks sequenced since we grabbed `test_blocks`, so just check
        // that the prefix of the window is correct.
        assert_eq!(res.window[..test_blocks[2].len()], test_blocks[2]);
        assert_eq!(res.next, None);
        // Fetch more blocks using the `from` form of the endpoint. Start from the last block we had
        // previously (ie fetch a slightly overlapping window) to ensure there is at least one block
        // in the new window.
        let from = test_blocks.iter().flatten().count() - 1;
        let more: TimeWindowQueryData = client
            .get(&format!("availability/headers/window/from/{from}/{end}",))
            .send()
            .await
            .unwrap();
        check_invariants(&more, start, end, false);
        assert_eq!(
            more.prev.as_ref().unwrap(),
            test_blocks.iter().flatten().nth(from - 1).unwrap()
        );
        assert_eq!(
            more.window[..res.window.len() - test_blocks[2].len() + 1],
            res.window[test_blocks[2].len() - 1..]
        );
        assert_eq!(res.next, None);
        // We should get the same result whether we query by block height or hash.
        let more2: TimeWindowQueryData = client
            .get(&format!(
                "availability/headers/window/from/hash/{}/{}",
                test_blocks[2].last().unwrap().commit(),
                end
            ))
            .send()
            .await
            .unwrap();
        check_invariants(&more2, start, end, false);
        assert_eq!(more2.from, more.from);
        assert_eq!(more2.prev, more.prev);
        assert_eq!(more2.next, more.next);
        assert_eq!(more2.window[..more.window.len()], more.window);

        // Case 3: the window is empty.
        let start = test_blocks[1][0].timestamp;
        let end = start;
        let res = get_window(start, end).await;
        assert_eq!(res.prev.unwrap(), *test_blocks[0].last().unwrap());
        assert_eq!(res.next.unwrap(), test_blocks[1][0]);
        assert_eq!(res.window, vec![]);

        // Case 5: no relevant blocks are available yet.
        client
            .get::<TimeWindowQueryData>(&format!(
                "availability/headers/window/{}/{}",
                u64::MAX - 1,
                u64::MAX
            ))
            .send()
            .await
            .unwrap_err();
    }
}
