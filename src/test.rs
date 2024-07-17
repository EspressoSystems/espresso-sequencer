#[cfg(test)]
mod tests {
    use crate::events_source::{EventConsumer, EventsStreamer}; // EventsUpdater};
                                                               //use crate::fetch::Fetch;
    use crate::events::{define_api, Error, Options};
    use async_compatibility_layer::art::async_spawn;
    use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
    use async_std::sync::RwLock;
    use futures::stream::StreamExt;
    use hotshot_types::{
        data::ViewNumber,
        event::{Event, EventType},
        traits::node_implementation::{ConsensusTime, NodeType},
    };

    use hotshot_example_types::node_types::TestTypes;
    use std::sync::Arc;
    use surf_disco::Client;
    use tide_disco::{App, Url};
    use vbs::version::{StaticVersion, StaticVersionType};

    // return a empty transaction event
    fn generate_event<Types: NodeType<Time = ViewNumber>>(view_number: u64) -> Event<Types> {
        Event {
            view_number: ViewNumber::new(view_number),
            event: EventType::Transactions {
                transactions: vec![],
            },
        }
    }

    #[async_std::test]
    async fn test_no_active_receiver() {
        tracing::info!("Starting test_no_active_receiver");
        setup_logging();
        setup_backtrace();
        let port = portpicker::pick_unused_port().expect("Could not find an open port");
        let api_url = Url::parse(format!("http://localhost:{port}").as_str()).unwrap();

        let known_nodes_with_stake = vec![];
        let non_staked_node_count = 0;
        let events_streamer = Arc::new(RwLock::new(EventsStreamer::new(
            known_nodes_with_stake,
            non_staked_node_count,
            None,
        )));

        // Start the web server.
        let mut app = App::<_, Error>::with_state(events_streamer.clone());

        let hotshot_events_api =
            define_api::<Arc<RwLock<EventsStreamer<TestTypes>>>, TestTypes, StaticVersion<0, 1>>(
                &Options::default(),
            )
            .expect("Failed to define hotshot eventsAPI");

        app.register_module("hotshot_events", hotshot_events_api)
            .expect("Failed to register hotshot events API");

        async_spawn(app.serve(api_url, StaticVersion::<0, 1>::instance()));
        let total_count = 5;
        let send_handle = async_spawn(async move {
            let mut send_count = 0;
            loop {
                let tx_event = generate_event(send_count);
                tracing::debug!("Before writing to events_source");
                events_streamer
                    .write()
                    .await
                    .handle_event(tx_event.clone())
                    .await;
                send_count += 1;
                tracing::debug!("After writing to events_source");
                if send_count >= total_count {
                    break;
                }
            }
        });

        send_handle.await;
    }

    #[async_std::test]
    async fn test_event_stream() {
        tracing::info!("Starting test_event_stream");
        setup_logging();
        setup_backtrace();

        let port = portpicker::pick_unused_port().expect("Could not find an open port");
        let api_url = Url::parse(format!("http://localhost:{port}").as_str()).unwrap();

        let known_nodes_with_stake = vec![];
        let non_staked_node_count = 0;
        let events_streamer = Arc::new(RwLock::new(EventsStreamer::new(
            known_nodes_with_stake,
            non_staked_node_count,
            None,
        )));

        // Start the web server.
        let mut app = App::<_, Error>::with_state(events_streamer.clone());

        let hotshot_events_api =
            define_api::<Arc<RwLock<EventsStreamer<TestTypes>>>, TestTypes, StaticVersion<0, 1>>(
                &Options::default(),
            )
            .expect("Failed to define hotshot eventsAPI");

        app.register_module("hotshot_events", hotshot_events_api)
            .expect("Failed to register hotshot events API");

        async_spawn(app.serve(api_url, StaticVersion::<0, 1>::instance()));

        // Start Client 1
        let client_1 = Client::<Error, StaticVersion<0, 1>>::new(
            format!("http://localhost:{}/hotshot_events", port)
                .parse()
                .unwrap(),
        );
        client_1.connect(None).await;

        tracing::info!("Client 1 Connected to server");

        // client 1 subscribe to hotshot events
        let mut events_1 = client_1
            .socket("events")
            .subscribe::<Event<TestTypes>>()
            .await
            .unwrap();

        tracing::info!("Client 1 Subscribed to events");

        // Start Client 2
        let client_2 = Client::<Error, StaticVersion<0, 1>>::new(
            format!("http://localhost:{}/hotshot_events", port)
                .parse()
                .unwrap(),
        );
        client_2.connect(None).await;

        tracing::info!("Client 2 Connected to server");

        // client 2 subscrive to hotshot events
        let mut events_2 = client_2
            .socket("events")
            .subscribe::<Event<TestTypes>>()
            .await
            .unwrap();

        tracing::info!("Client 2 Subscribed to events");

        let total_count = 5;
        // wait for these events to receive on client 1
        let receive_handle_1 = async_spawn(async move {
            let mut receive_count = 0;
            while let Some(event) = events_1.next().await {
                let event = event.unwrap();
                tracing::info!("Received event in Client 1: {:?}", event);

                receive_count += 1;

                if receive_count == total_count {
                    tracing::info!("Client1 Received all sent events, exiting loop");
                    break;
                }
            }

            assert_eq!(receive_count, total_count);

            tracing::info!("stream ended");
        });

        // wait for these events to receive on client 2
        let receive_handle_2 = async_spawn(async move {
            let mut receive_count = 0;
            while let Some(event) = events_2.next().await {
                let event = event.unwrap();

                tracing::info!("Received event in Client 2: {:?}", event);
                receive_count += 1;

                if receive_count == total_count {
                    tracing::info!("Client 2 Received all sent events, exiting loop");
                    break;
                }
            }

            assert_eq!(receive_count, total_count);

            tracing::info!("stream ended");
        });

        let send_handle = async_spawn(async move {
            let mut send_count = 0;
            loop {
                let tx_event = generate_event(send_count);
                tracing::debug!("Before writing to events_source");
                events_streamer
                    .write()
                    .await
                    .handle_event(tx_event.clone())
                    .await;
                send_count += 1;
                tracing::debug!("After writing to events_source");
                tracing::info!("Event sent: {:?}", tx_event);
                if send_count >= total_count {
                    break;
                }
            }
        });

        send_handle.await;
        receive_handle_1.await;
        receive_handle_2.await;
    }
}
