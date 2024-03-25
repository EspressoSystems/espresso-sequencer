#![allow(unused_imports)]
#[cfg(test)]
mod tests {
    use crate::events_source::{EventConsumer, EventsStreamer};
    //use crate::fetch::Fetch;
    use crate::{
        events::{define_api, Error, Options},
        events_info::EventInfo,
        events_source::EventsSource,
    };
    use async_compatibility_layer::art::{async_sleep, async_spawn};
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use async_trait::async_trait;
    use futures::future::BoxFuture;
    use futures::stream::{BoxStream, Stream, StreamExt};
    //use hotshot::traits::NodeImplementation;
    use hotshot::types::SystemContextHandle;
    use hotshot_query_service::testing::{
        consensus::{MockNetwork, TestableDataSource},
        mocks::{MockNodeImpl, MockTypes},
    };
    use hotshot_types::constants::{Version01, STATIC_VER_0_1};
    use hotshot_types::data::ViewNumber;
    use hotshot_types::event::{Event, EventType};
    use hotshot_types::traits::{
        node_implementation::{ConsensusTime, NodeType},
        signature_key::SignatureKey,
    };
    //use std::arch::aarch64::ST;
    //use async_lock::RwLock;
    use async_std::sync::RwLock;

    use hotshot_example_types::node_types::TestTypes;
    use std::ops::RangeBounds;
    use std::sync::Arc;
    use std::time::Duration;
    use surf_disco::{get, Client};
    use tagged_base64::TaggedBase64;
    use tide_disco::{method::ReadState, App, Url};
    use tracing;

    fn generate_event<Types: NodeType<Time = ViewNumber>>() -> Event<Types> {
        // return a empty transaction event
        Event {
            view_number: ViewNumber::new(0 as u64),
            event: EventType::Transactions {
                transactions: vec![],
            },
        }
    }

    #[async_std::test]
    async fn test_event_stream() {
        tracing::info!("Starting hotshot test_event_stream");
        setup_logging();
        setup_backtrace();

        let port = portpicker::pick_unused_port().expect("Could not find an open port");
        let api_url = Url::parse(format!("http://localhost:{port}").as_str()).unwrap();

        let events_source = Arc::new(RwLock::new(EventsStreamer::new()));
        // Start the web server.
        let mut app = App::<_, Error, Version01>::with_state(events_source.clone());

        let hotshot_events_api =
            define_api::<Arc<RwLock<EventsStreamer<TestTypes>>>, TestTypes, Version01>(
                &Options::default(),
            )
            .expect("Failed to define hotshot eventsAPI");

        // failures:

        // ---- test::tests::test_event_stream stdout ----
        // The application panicked (crashed).
        // Message:  Failed to define hotshot eventsAPI: IncorrectMethod { expected: Socket, actual: Http(Get) }
        // Location: src/test.rs:69

        // Backtrace omitted. Run with RUST_BACKTRACE=1 environment variable to display it.
        // Run with RUST_BACKTRACE=full to include source snippets.

        // failures:
        //     test::tests::test_event_stream

        // test result: FAILED. 4 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.01s

        // error: test failed, to rerun pass `--lib`

        app.register_module("hotshot_events", hotshot_events_api)
            .expect("Failed to register hotshot events API");

        async_spawn(app.serve(api_url, STATIC_VER_0_1));

        //{
        //let events_source = events_source.clone();
        async_spawn(async move {
            loop {
                let tx_event = generate_event();
                events_source.write().await.handle_event(&tx_event).await;
            }
        });
        //}

        // Start a client.
        let client = Client::<Error, Version01>::new(
            format!("http://localhost:{}/hotshot_events", port)
                .parse()
                .unwrap(),
        );
        assert!(client.connect(Some(Duration::from_secs(60))).await);

        let mut events = client
            .socket("stream/events/0")
            .subscribe::<EventInfo<TestTypes>>()
            .await
            .unwrap();

        let event = events.next().await.unwrap();
        tracing::info!("Received event: {:?}", event);
    }
}
