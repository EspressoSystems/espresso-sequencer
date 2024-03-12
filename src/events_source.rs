use std::{hash::Hash, marker::PhantomData};

use hotshot_types::event::Event;
use serde::{Deserialize, Serialize};

use crate::{events_info::{EventInfo, EventError}};

#[async_trait]
pub trait EventsSource<I>
where
    I: NodeType,
    <<I as NodeType>::SignatureKey as SignatureKey>::PureAssembledSignatureType:
        for<'a> TryFrom<&'a TaggedBase64> + Into<TaggedBase64>,
{
    async fn get_available_hotshot_events(
        &self,
        view_number: I::Time,
    ) -> Result<Vec<EventInfo<I>>, EventError>;
}