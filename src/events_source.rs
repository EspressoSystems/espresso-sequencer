use crate::{events::EventError, events_info::EventInfo};
use async_trait::async_trait;
use hotshot_types::{
    data::ViewNumber,
    traits::{node_implementation::NodeType, signature_key::SignatureKey},
};
use tagged_base64::TaggedBase64;

#[async_trait]
pub trait EventsSource<I>
where
    I: NodeType,
    <<I as NodeType>::SignatureKey as SignatureKey>::PureAssembledSignatureType:
        for<'a> TryFrom<&'a TaggedBase64> + Into<TaggedBase64>,
{
    async fn get_available_hotshot_events(
        &self,
        view_number: ViewNumber,
    ) -> Result<Vec<EventInfo<I>>, EventError>;
}
