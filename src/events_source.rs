use async_trait::async_trait;
use std::{hash::Hash, marker::PhantomData};
use hotshot_types::event::Event;
use serde::{Deserialize, Serialize};
use hotshot_types::{
    traits::{node_implementation::NodeType, signature_key::SignatureKey},
    utils::BuilderCommitment,
    vid::VidCommitment,
    data::ViewNumber
};
use tagged_base64::TaggedBase64;
use crate::{events_info::{EventInfo}, events::EventError};

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