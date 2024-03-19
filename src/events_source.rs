use crate::events_info::EventInfo;
use crate::fetch::Fetch;
use async_trait::async_trait;
use futures::stream::{BoxStream, Stream, StreamExt};
//use hotshot_query_service::availability::Fetch;
use hotshot_types::traits::{node_implementation::NodeType, signature_key::SignatureKey};
use std::ops::RangeBounds;
use tagged_base64::TaggedBase64;
#[async_trait]
pub trait EventsSource<I>
where
    I: NodeType,
    <<I as NodeType>::SignatureKey as SignatureKey>::PureAssembledSignatureType:
        for<'a> TryFrom<&'a TaggedBase64> + Into<TaggedBase64>,
{
    type EventRange<R>: Stream<Item = Fetch<EventInfo<I>>> + Unpin + Send + 'static
    where
        R: RangeBounds<usize> + Send;

    async fn get_events_range<R>(&self, range: R) -> Self::EventRange<R>
    where
        R: RangeBounds<usize> + Send + 'static;

    async fn subscribe_events(&self, view_number: usize) -> BoxStream<'static, EventInfo<I>> {
        self.get_events_range(view_number..)
            .await
            .then(Fetch::resolve)
            .boxed()
    }
}
