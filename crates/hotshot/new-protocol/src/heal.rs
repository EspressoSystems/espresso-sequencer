//! Re-creation of a node's lost VID shares from peers' shares.

use hotshot_types::{
    data::{
        EpochNumber, VidCommitment2, VidDisperseShare2, ViewNumber,
        vid_disperse::AvidmGf2DisperseParams,
    },
    traits::node_implementation::NodeType,
};

use crate::vid::{self, VidDisperseError};

#[derive(Debug, thiserror::Error)]
pub enum HealError {
    #[error("own key has no share in the committee")]
    NotRecipient,

    #[error("re-encoding failed: {0}")]
    Encode(#[from] VidDisperseError),

    #[error("re-encoded share is inconsistent with the commitment")]
    Inconsistent,
}

/// Re-encode the payload in `params` and return the share `own` received when
/// it was originally dispersed, byte-equal to what the live path stored.
pub fn rederive_own_share<T: NodeType>(
    params: &AvidmGf2DisperseParams<T>,
    own: &T::SignatureKey,
    view: ViewNumber,
    epoch: EpochNumber,
    commit: VidCommitment2,
) -> Result<VidDisperseShare2<T>, HealError> {
    let index = params
        .recipients
        .iter()
        .position(|key| key == own)
        .ok_or(HealError::NotRecipient)?;
    let all: Vec<usize> = (0..params.ns_table.len()).collect();
    let pieces = vid::encode_namespaces(params, &all)?.swap_remove(index);
    let share = vid::assemble_share(
        view,
        epoch,
        commit,
        own.clone(),
        params.param.clone(),
        pieces,
    );
    if !share.is_consistent() {
        return Err(HealError::Inconsistent);
    }
    Ok(share)
}

#[cfg(test)]
mod tests {
    use hotshot_example_types::node_types::TestTypes;
    use hotshot_types::{
        data::VidDisperse2,
        signature_key::BLSPubKey,
        traits::{BlockPayload, signature_key::SignatureKey},
    };

    use super::*;
    use crate::tests::common::utils::{TestData, mock_membership};

    #[tokio::test]
    async fn heal_rederive_matches_live_ok() {
        let data = TestData::new(3).await;
        let membership = mock_membership();
        for view in &data.views {
            let params = VidDisperse2::<TestTypes>::disperse_params(
                &view.payload,
                &membership,
                Some(view.epoch_number),
                &view.proposal.data.block_header.metadata,
            )
            .unwrap();
            let commit = view.vid_commitment();
            for i in 0..10u64 {
                let own = BLSPubKey::generated_from_seed_indexed([0u8; 32], i).0;
                let share =
                    rederive_own_share(&params, &own, view.view_number, view.epoch_number, commit)
                        .unwrap();
                assert_eq!(share, view.vid_share_for(&own));
            }
        }
    }

    #[tokio::test]
    async fn heal_rederive_not_recipient_err() {
        let data = TestData::new(1).await;
        let view = &data.views[0];
        let params = VidDisperse2::<TestTypes>::disperse_params(
            &view.payload,
            &mock_membership(),
            Some(view.epoch_number),
            &view.proposal.data.block_header.metadata,
        )
        .unwrap();
        let stranger = BLSPubKey::generated_from_seed_indexed([0u8; 32], 99).0;
        let err = rederive_own_share(
            &params,
            &stranger,
            view.view_number,
            view.epoch_number,
            view.vid_commitment(),
        )
        .unwrap_err();
        assert!(matches!(err, HealError::NotRecipient));
    }
}
