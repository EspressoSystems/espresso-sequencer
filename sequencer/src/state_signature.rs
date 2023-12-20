use crate::{context::SequencerContext, network, Leaf, Node, SeqTypes};
use hotshot_types::{light_client::LightClientState, traits::state::ConsensusTime};

pub mod signature_pool;

pub type StateSignatureScheme =
    jf_primitives::signatures::schnorr::SchnorrSignatureScheme<ark_ed_on_bn254::EdwardsConfig>;
pub use hotshot_stake_table::vec_based::config::FieldType as BaseField;

pub async fn state_signature_hook<N: network::Type>(
    context: &mut SequencerContext<SeqTypes, Node<N>>,
    leaf: &Leaf,
) {
    let new_light_client_state = LightClientState::<BaseField> {
        view_number: leaf.get_view_number().get_u64() as usize,
        block_height: leaf.get_height() as usize,
        block_comm_root: BaseField::default(),
        fee_ledger_comm: BaseField::default(),
        stake_table_comm: (
            // TODO(Chengyu): change here
            BaseField::default(),
            BaseField::default(),
            BaseField::default(),
        ),
    };
    context.sign_new_state(&new_light_client_state);
}
