use crate::{enriched_handle::EnrichedSystemContextHandle, network, Leaf, Node, SeqTypes};
use hotshot_types::{
    light_client::{LightClientState, StateSignKey},
    traits::state::ConsensusTime,
};
use jf_primitives::signatures::SignatureScheme;

type StateSignatureScheme =
    jf_primitives::signatures::schnorr::SchnorrSignatureScheme<ark_ed_on_bn254::EdwardsConfig>;
use hotshot_stake_table::vec_based::config::FieldType as BaseField;

pub async fn light_client_signature_hook<N: network::Type>(
    handle: &mut EnrichedSystemContextHandle<SeqTypes, Node<N>>,
    leaf: &Leaf,
    sign_key: &StateSignKey,
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
    let state_msg: [BaseField; 7] = new_light_client_state.into();
    let state_signature =
        StateSignatureScheme::sign(&(), sign_key, state_msg, &mut rand::thread_rng()).unwrap();
    handle.add_new_state_signature(leaf.get_height(), state_signature);
}
