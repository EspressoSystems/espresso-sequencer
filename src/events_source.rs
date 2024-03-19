use crate::events_info::EventInfo;
use crate::fetch::Fetch;
use crate::notifier::Notifier;
use async_lock::{RwLock, RwLockReadGuard};
use async_trait::async_trait;
use futures::stream::{BoxStream, Stream, StreamExt};
//use hotshot_query_service::availability::Fetch;
use futures::future::BoxFuture;
use futures::stream::{BoxStream, Stream, StreamExt};
use hotshot_types::traits::{node_implementation::NodeType, signature_key::SignatureKey};
use std::ops::RangeBounds;
use std::sync::Arc;
use tagged_base64::TaggedBase64;

#[async_trait]
pub trait EventsSource<Types>
where
    Types: NodeType,
    <<Types as NodeType>::SignatureKey as SignatureKey>::PureAssembledSignatureType:
        for<'a> TryFrom<&'a TaggedBase64> + Into<TaggedBase64>,
{
    type EventRange<R>: Stream<Item = Fetch<EventInfo<Types>>> + Unpin + Send + 'static
    where
        R: RangeBounds<usize> + Send;

    async fn get_events_range<R>(&self, range: R) -> Self::EventRange<R>
    where
        R: RangeBounds<usize> + Send + 'static;

    async fn subscribe_events(&self, view_number: usize) -> BoxStream<'static, EventInfo<Types>> {
        self.get_events_range(view_number..)
            .await
            .then(Fetch::resolve)
            .boxed()
    }
}

/// Asynchronous retrieval and storage of [`Fetchable`] resources.
#[derive(Debug)]
struct Fetcher<Types: NodeType> {
    notifier: RwLock<Notifier<EventInfo<Types>>>,
}

// From fetcher.rs
impl<Types> Fetcher<Types>
where
    Types: NodeType,
    EventInfo<Types>: EventsSource<Types>,
    <<Types as NodeType>::SignatureKey as SignatureKey>::PureAssembledSignatureType:
        for<'a> TryFrom<&'a TaggedBase64> + Into<TaggedBase64>,
{
    async fn passive_fetch(notifier: &Notifier<Types>) -> BoxFuture<'static, Option<Self>> {
        notifier
            .wait_for(move |event| true)
            .await
            .into_future()
            .boxed()
    }

    async fn fetch<R, T>(
        self: &Arc<Self>,
        notifier: &RwLockReadGuard<'_, Notifier<Types>>,
    ) -> Fetch<T> {
        // Subscribe to notifications so we are alerted when we get the resource.
        let fut = Self::passive_fetch(&**notifier)
            .await
            .then(move |opt| async move {
                match opt {
                    Some(t) => t,
                    None => {
                        // If `passive_fetch` returns `None`, it means the notifier was dropped
                        // without ever sending a notification. In this case, the correct behavior
                        // is actually to block forever (unless the `Fetch` itself is dropped),
                        // since the semantics of `Fetch` are to never fail. This is analogous to
                        // fetching an object which doesn't actually exist: the `Fetch` will never
                        // return.
                        //
                        // However, for ease of debugging, and since this is never expected to
                        // happen in normal usage, we panic instead. This should only happen in two
                        // cases:
                        // * The server was shut down (dropping the notifier) without cleaning up
                        //   some background tasks. This will not affect runtime behavior, but
                        //   should be fixed if it happens.
                        // * There is a very unexpected runtime bug resulting in the notifier being
                        //   dropped. If this happens, things are very broken in any case, and it is
                        //   better to panic loudly than simply block forever.
                        panic!("notifier dropped without satisfying request {req:?}");
                    }
                }
            });
        // Wait for the object to arrive.
        Fetch::Pending(fut.boxed())
    }
}
