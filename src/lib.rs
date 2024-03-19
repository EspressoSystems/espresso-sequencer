mod api;
pub mod events;
pub mod events_info;
pub mod events_source;
pub mod fetch;
pub mod notifier;
#[cfg(test)]
mod tests {
    use crate::fetch::Fetch;
    use crate::{
        events::{define_api, Error, Options},
        events_info::EventInfo,
        events_source::EventsSource,
    };
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
    use hotshot_types::traits::{node_implementation::NodeType, signature_key::SignatureKey};
    use std::ops::RangeBounds;
    use std::time::Duration;
    use surf_disco::Client;
    use tagged_base64::TaggedBase64;
    use tide_disco::{method::ReadState, App, Url};

    pub fn setup_test() {
        setup_logging();
        setup_backtrace();

        #[cfg(all(feature = "backtrace-on-stack-overflow", not(windows)))]
        unsafe {
            backtrace_on_stack_overflow::enable();
        }
    }
    /// A mock implementation of the hotshot events source.
    #[derive(Clone, Debug)]
    pub struct TestableEventsSource {}

    #[async_trait]
    impl ReadState for TestableEventsSource {
        type State = Self;

        async fn read<T>(
            &self,
            op: impl Send + for<'a> FnOnce(&'a Self::State) -> BoxFuture<'a, T> + 'async_trait,
        ) -> T {
            op(self).await
        }
    }
    impl TestableEventsSource {
        fn get_event(
            hotshot_handle: SystemContextHandle<MockTypes, MockNodeImpl>,
        ) -> BoxStream<'static, Fetch<EventInfo<MockTypes>>> {
            let mut hotshot_handle = hotshot_handle;
            let mut events = Vec::new();
            for i in 0..10 {

                // TODO
            }
            futures::stream::iter(events.into_iter().map(Fetch::Resolve)).boxed()
        }
    }
    #[async_trait]
    impl<I> EventsSource<I> for TestableEventsSource
    where
        I: NodeType,
        <<I as NodeType>::SignatureKey as SignatureKey>::PureAssembledSignatureType:
            for<'a> TryFrom<&'a TaggedBase64> + Into<TaggedBase64>,
    {
        type EventRange<R> = BoxStream<'static, Fetch<EventInfo<I>>>
        where
            R: RangeBounds<usize> + Send;

        async fn get_events_range<R>(&self, range: R) -> Self::EventRange<R>
        where
            R: std::ops::RangeBounds<usize> + Send + 'static,
        {
            self.get_range(range)
        }

        // async fn subscribe_events(&self, view_number: usize) -> BoxStream<'static, EventInfo<I>> {
        //     self.get_events_range(view_number..)
        //         .await
        //         .then(Fetch::resolve)
        //         .boxed()
        // }
    }
    async fn test_helper<D: TestableDataSource>() {
        setup_test();
        let port = portpicker::pick_unused_port().expect("Could not find an open port");
        let api_url = Url::parse(format!("http://localhost:{port}").as_str()).unwrap();

        // Create the consensus network.
        let mut network = MockNetwork::<D>::init().await;
        network.start().await;

        let hotshot_context_handle = network.handle();

        // Start the web server.
        let mut app = App::<TestableEventsSource, Error>::with_state(TestableEventsSource {});
        let hotshot_events_api = define_api::<TestableEventsSource, MockTypes>(&Options::default())
            .expect("Failed to define hotshot eventsAPI");

        app.register_module("hotshot_events", hotshot_events_api)
            .expect("Failed to register hotshot events API");
        network.spawn("server", app.serve(format!("0.0.0.0:{}", port)));

        // Start a client.
        let client = Client::<Error>::new(
            format!("http://localhost:{}/hotshot_events", port)
                .parse()
                .unwrap(),
        );
        assert!(client.connect(Some(Duration::from_secs(60))).await);

        let events = client
            .socket("stream/events/0")
            .subscribe::<EventInfo<MockTypes>>()
            .await
            .unwrap();

        network.shut_down().await;
    }

    #[async_std::test]
    async fn test_event_stream() {
        print!("Starting test");
        test_helper().await;
    }
}
