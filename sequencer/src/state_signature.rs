use crate::{api::Context, network, Leaf, SeqTypes};
use futures::stream::{Stream, StreamExt};
use hotshot::types::Event;
use hotshot_types::{light_client::LightClientState, traits::state::ConsensusTime};

pub mod signature_pool;

pub type StateSignatureScheme =
    jf_primitives::signatures::schnorr::SchnorrSignatureScheme<ark_ed_on_bn254::EdwardsConfig>;
pub use hotshot_stake_table::vec_based::config::FieldType as BaseField;

pub(super) async fn state_signature_loop<N>(
    context: Context<N>,
    mut events: impl Stream<Item = Event<SeqTypes>> + Unpin,
) where
    N: network::Type,
{
    tracing::debug!("waiting for event");
    while let Some(event) = events.next().await {
        tracing::info!("got event {:?}", event);

        // Trigger the light client signature hook when a new leaf is decided
        if let Event {
            event: hotshot_types::event::EventType::Decide { leaf_chain, .. },
            ..
        } = event
        {
            if let Some(leaf) = leaf_chain.first() {
                let new_state = form_light_client_state(leaf);
                context.sign_new_state(&new_state);
            }
        }
    }
    tracing::warn!("end of HotShot event stream, updater task will exit");
}

fn form_light_client_state(leaf: &Leaf) -> LightClientState<BaseField> {
    // TODO(Chengyu): fill these `default()` with actual value
    LightClientState::<BaseField> {
        view_number: leaf.get_view_number().get_u64() as usize,
        block_height: leaf.get_height() as usize,
        block_comm_root: BaseField::default(),
        fee_ledger_comm: BaseField::default(),
        stake_table_comm: (
            BaseField::default(),
            BaseField::default(),
            BaseField::default(),
        ),
    }
}
