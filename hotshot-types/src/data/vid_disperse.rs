// Copyright (c) 2021-2024 Espresso Systems (espressosys.com)
// This file is part of the HotShot repository.

// You should have received a copy of the MIT License
// along with the HotShot repository. If not, see <https://mit-license.org/>.

//! This module provides types for VID disperse related data structures.

use std::{collections::BTreeMap, fmt::Debug, hash::Hash, marker::PhantomData, sync::Arc};

use async_lock::RwLock;
use hotshot_utils::anytrace::*;
use jf_vid::{VidDisperse as JfVidDisperse, VidScheme};
use serde::{Deserialize, Serialize};
use tokio::task::spawn_blocking;

use crate::{
    impl_has_epoch,
    message::Proposal,
    simple_vote::HasEpoch,
    traits::{
        block_contents::EncodeBytes, election::Membership, node_implementation::NodeType,
        signature_key::SignatureKey, BlockPayload,
    },
    vid::{
        advz::{advz_scheme, ADVZCommitment, ADVZCommon, ADVZScheme, ADVZShare},
        avidm::{init_avidm_param, AvidMCommitment, AvidMScheme, AvidMShare},
    },
    vote::HasViewNumber,
};

use super::ns_table::parse_ns_table;

impl_has_epoch!(
    ADVZDisperse<TYPES>,
    AvidMDisperse<TYPES>,
    VidDisperseShare2<TYPES>
);

/// ADVZ dispersal data
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct ADVZDisperse<TYPES: NodeType> {
    /// The view number for which this VID data is intended
    pub view_number: TYPES::View,
    /// Epoch the data of this proposal belongs to
    pub epoch: Option<TYPES::Epoch>,
    /// Epoch to which the recipients of this VID belong to
    pub target_epoch: Option<TYPES::Epoch>,
    /// VidCommitment calculated based on the number of nodes in `target_epoch`.
    pub payload_commitment: ADVZCommitment,
    /// VidCommitment calculated based on the number of nodes in `epoch`. Needed during epoch transition.
    pub data_epoch_payload_commitment: Option<ADVZCommitment>,
    /// A storage node's key and its corresponding VID share
    pub shares: BTreeMap<TYPES::SignatureKey, ADVZShare>,
    /// VID common data sent to all storage nodes
    pub common: ADVZCommon,
}

impl<TYPES: NodeType> HasViewNumber<TYPES> for ADVZDisperse<TYPES> {
    fn view_number(&self) -> TYPES::View {
        self.view_number
    }
}

impl<TYPES: NodeType> ADVZDisperse<TYPES> {
    /// Create VID dispersal from a specified membership for the target epoch.
    /// Uses the specified function to calculate share dispersal
    /// Allows for more complex stake table functionality
    async fn from_membership(
        view_number: TYPES::View,
        mut vid_disperse: JfVidDisperse<ADVZScheme>,
        membership: &Arc<RwLock<TYPES::Membership>>,
        target_epoch: Option<TYPES::Epoch>,
        data_epoch: Option<TYPES::Epoch>,
        data_epoch_payload_commitment: Option<ADVZCommitment>,
    ) -> Self {
        let shares = membership
            .read()
            .await
            .committee_members(view_number, target_epoch)
            .iter()
            .map(|node| (node.clone(), vid_disperse.shares.remove(0)))
            .collect();

        Self {
            view_number,
            shares,
            common: vid_disperse.common,
            payload_commitment: vid_disperse.commit,
            data_epoch_payload_commitment,
            epoch: data_epoch,
            target_epoch,
        }
    }

    /// Calculate the vid disperse information from the payload given a view, epoch and membership,
    /// If the sender epoch is missing, it means it's the same as the target epoch.
    ///
    /// # Errors
    /// Returns an error if the disperse or commitment calculation fails
    #[allow(clippy::panic)]
    pub async fn calculate_vid_disperse(
        payload: &TYPES::BlockPayload,
        membership: &Arc<RwLock<TYPES::Membership>>,
        view: TYPES::View,
        target_epoch: Option<TYPES::Epoch>,
        data_epoch: Option<TYPES::Epoch>,
    ) -> Result<Self> {
        let num_nodes = membership.read().await.total_nodes(target_epoch);

        let txns = payload.encode();
        let txns_clone = Arc::clone(&txns);
        let num_txns = txns.len();

        let vid_disperse = spawn_blocking(move || advz_scheme(num_nodes).disperse(&txns_clone))
            .await
            .wrap()
            .context(error!("Join error"))?
            .wrap()
            .context(|err| error!("Failed to calculate VID disperse. Error: {}", err))?;

        let payload_commitment = if target_epoch == data_epoch {
            None
        } else {
            let num_nodes = membership.read().await.total_nodes(data_epoch);

            Some(
              spawn_blocking(move || advz_scheme(num_nodes).commit_only(&txns))
                .await
                .wrap()
                .context(error!("Join error"))?
                .wrap()
                .context(|err| error!("Failed to calculate VID commitment with (num_storage_nodes, payload_byte_len) = ({}, {}). Error: {}", num_nodes, num_txns, err))?
            )
        };

        Ok(Self::from_membership(
            view,
            vid_disperse,
            membership,
            target_epoch,
            data_epoch,
            payload_commitment,
        )
        .await)
    }

    /// Returns the payload length in bytes.
    pub fn payload_byte_len(&self) -> u32 {
        ADVZScheme::get_payload_byte_len(&self.common)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
/// ADVZ share and associated metadata for a single node
pub struct ADVZDisperseShare<TYPES: NodeType> {
    /// The view number for which this VID data is intended
    pub view_number: TYPES::View,
    /// Block payload commitment
    pub payload_commitment: ADVZCommitment,
    /// A storage node's key and its corresponding VID share
    pub share: ADVZShare,
    /// VID common data sent to all storage nodes
    pub common: ADVZCommon,
    /// a public key of the share recipient
    pub recipient_key: TYPES::SignatureKey,
}

impl<TYPES: NodeType> HasViewNumber<TYPES> for ADVZDisperseShare<TYPES> {
    fn view_number(&self) -> TYPES::View {
        self.view_number
    }
}

impl<TYPES: NodeType> ADVZDisperseShare<TYPES> {
    /// Create a vector of `VidDisperseShare` from `VidDisperse`
    pub fn from_advz_disperse(vid_disperse: ADVZDisperse<TYPES>) -> Vec<Self> {
        vid_disperse
            .shares
            .into_iter()
            .map(|(recipient_key, share)| Self {
                share,
                recipient_key,
                view_number: vid_disperse.view_number,
                common: vid_disperse.common.clone(),
                payload_commitment: vid_disperse.payload_commitment,
            })
            .collect()
    }

    /// Consume `self` and return a `Proposal`
    pub fn to_proposal(
        self,
        private_key: &<TYPES::SignatureKey as SignatureKey>::PrivateKey,
    ) -> Option<Proposal<TYPES, Self>> {
        let Ok(signature) =
            TYPES::SignatureKey::sign(private_key, self.payload_commitment.as_ref())
        else {
            tracing::error!("VID: failed to sign dispersal share payload");
            return None;
        };
        Some(Proposal {
            signature,
            _pd: PhantomData,
            data: self,
        })
    }

    /// Create `VidDisperse` out of an iterator to `VidDisperseShare`s
    pub fn to_advz_disperse<'a, I>(mut it: I) -> Option<ADVZDisperse<TYPES>>
    where
        I: Iterator<Item = &'a Self>,
    {
        let first_vid_disperse_share = it.next()?.clone();
        let mut share_map = BTreeMap::new();
        share_map.insert(
            first_vid_disperse_share.recipient_key,
            first_vid_disperse_share.share,
        );
        let mut vid_disperse = ADVZDisperse {
            view_number: first_vid_disperse_share.view_number,
            epoch: None,
            target_epoch: None,
            payload_commitment: first_vid_disperse_share.payload_commitment,
            data_epoch_payload_commitment: None,
            common: first_vid_disperse_share.common,
            shares: share_map,
        };
        let _ = it.map(|vid_disperse_share| {
            vid_disperse.shares.insert(
                vid_disperse_share.recipient_key.clone(),
                vid_disperse_share.share.clone(),
            )
        });
        Some(vid_disperse)
    }

    /// Split a VID share proposal into a proposal for each recipient.
    pub fn to_vid_share_proposals(
        vid_disperse: ADVZDisperse<TYPES>,
        signature: &<TYPES::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    ) -> Vec<Proposal<TYPES, Self>> {
        vid_disperse
            .shares
            .into_iter()
            .map(|(recipient_key, share)| Proposal {
                data: Self {
                    share,
                    recipient_key,
                    view_number: vid_disperse.view_number,
                    common: vid_disperse.common.clone(),
                    payload_commitment: vid_disperse.payload_commitment,
                },
                signature: signature.clone(),
                _pd: PhantomData,
            })
            .collect()
    }

    /// Internally verify the share given necessary information
    ///
    /// # Errors
    /// Verification fail
    #[allow(clippy::result_unit_err)]
    pub fn verify_share(&self, total_nodes: usize) -> std::result::Result<(), ()> {
        advz_scheme(total_nodes)
            .verify_share(&self.share, &self.common, &self.payload_commitment)
            .unwrap_or(Err(()))
    }

    /// Returns the payload length in bytes.
    pub fn payload_byte_len(&self) -> u32 {
        ADVZScheme::get_payload_byte_len(&self.common)
    }
}

/// ADVZ dispersal data
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct AvidMDisperse<TYPES: NodeType> {
    /// The view number for which this VID data is intended
    pub view_number: TYPES::View,
    /// Epoch the data of this proposal belongs to
    pub epoch: Option<TYPES::Epoch>,
    /// Epoch to which the recipients of this VID belong to
    pub target_epoch: Option<TYPES::Epoch>,
    /// VidCommitment calculated based on the number of nodes in `target_epoch`.
    pub payload_commitment: AvidMCommitment,
    /// VidCommitment calculated based on the number of nodes in `epoch`. Needed during epoch transition.
    pub data_epoch_payload_commitment: Option<AvidMCommitment>,
    /// A storage node's key and its corresponding VID share
    pub shares: BTreeMap<TYPES::SignatureKey, AvidMShare>,
    /// Length of payload in bytes
    pub payload_byte_len: usize,
}

impl<TYPES: NodeType> HasViewNumber<TYPES> for AvidMDisperse<TYPES> {
    fn view_number(&self) -> TYPES::View {
        self.view_number
    }
}

impl<TYPES: NodeType> AvidMDisperse<TYPES> {
    /// Create VID dispersal from a specified membership for the target epoch.
    /// Uses the specified function to calculate share dispersal
    /// Allows for more complex stake table functionality
    async fn from_membership(
        view_number: TYPES::View,
        commit: AvidMCommitment,
        shares: &[AvidMShare],
        membership: &Arc<RwLock<TYPES::Membership>>,
        target_epoch: Option<TYPES::Epoch>,
        data_epoch: Option<TYPES::Epoch>,
        data_epoch_payload_commitment: Option<AvidMCommitment>,
    ) -> Self {
        let payload_byte_len = shares[0].payload_byte_len();
        let shares = membership
            .read()
            .await
            .committee_members(view_number, target_epoch)
            .iter()
            .zip(shares)
            .map(|(node, share)| (node.clone(), share.clone()))
            .collect();

        Self {
            view_number,
            shares,
            payload_commitment: commit,
            data_epoch_payload_commitment,
            epoch: data_epoch,
            target_epoch,
            payload_byte_len,
        }
    }

    /// Calculate the vid disperse information from the payload given a view, epoch and membership,
    /// If the sender epoch is missing, it means it's the same as the target epoch.
    ///
    /// # Errors
    /// Returns an error if the disperse or commitment calculation fails
    #[allow(clippy::panic)]
    #[allow(clippy::single_range_in_vec_init)]
    pub async fn calculate_vid_disperse(
        payload: &TYPES::BlockPayload,
        membership: &Arc<RwLock<TYPES::Membership>>,
        view: TYPES::View,
        target_epoch: Option<TYPES::Epoch>,
        data_epoch: Option<TYPES::Epoch>,
        metadata: &<TYPES::BlockPayload as BlockPayload<TYPES>>::Metadata,
    ) -> Result<Self> {
        let num_nodes = membership.read().await.total_nodes(target_epoch);

        let txns = payload.encode();
        let txns_clone = Arc::clone(&txns);
        let num_txns = txns.len();

        let avidm_param = init_avidm_param(num_nodes)?;
        // TODO: get weight distribution
        let weights = vec![1u32; num_nodes];
        let ns_table = parse_ns_table(num_txns, &metadata.encode());
        let ns_table_clone = ns_table.clone();
        let (commit, shares) = spawn_blocking(move || {
            AvidMScheme::ns_disperse(&avidm_param, &weights, &txns_clone, ns_table_clone)
        })
        .await
        .wrap()
        .context(error!("Join error"))?
        .wrap()
        .context(|err| error!("Failed to calculate VID disperse. Error: {}", err))?;

        let payload_commitment = if target_epoch == data_epoch {
            None
        } else {
            let num_nodes = membership.read().await.total_nodes(data_epoch);
            let avidm_param = init_avidm_param(num_nodes)?;

            Some(
              spawn_blocking(move ||
                AvidMScheme::commit(
                    &avidm_param,
                    &txns,
                    ns_table,
                ))
                .await
                .wrap()
                .context(error!("Join error"))?
                .wrap()
                .context(|err| error!("Failed to calculate VID commitment with (num_storage_nodes, payload_byte_len) = ({}, {}). Error: {}", num_nodes, num_txns, err))?
            )
        };

        Ok(Self::from_membership(
            view,
            commit,
            &shares,
            membership,
            target_epoch,
            data_epoch,
            payload_commitment,
        )
        .await)
    }

    /// Returns the payload length in bytes.
    pub fn payload_byte_len(&self) -> u32 {
        self.payload_byte_len as u32
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
/// VID share and associated metadata for a single node
pub struct VidDisperseShare2<TYPES: NodeType> {
    /// The view number for which this VID data is intended
    pub view_number: TYPES::View,
    /// The epoch number for which this VID data belongs to
    pub epoch: Option<TYPES::Epoch>,
    /// The epoch number to which the recipient of this VID belongs to
    pub target_epoch: Option<TYPES::Epoch>,
    /// Block payload commitment
    pub payload_commitment: AvidMCommitment,
    /// VidCommitment calculated based on the number of nodes in `epoch`. Needed during epoch transition.
    pub data_epoch_payload_commitment: Option<AvidMCommitment>,
    /// A storage node's key and its corresponding VID share
    pub share: AvidMShare,
    /// a public key of the share recipient
    pub recipient_key: TYPES::SignatureKey,
}

impl<TYPES: NodeType> HasViewNumber<TYPES> for VidDisperseShare2<TYPES> {
    fn view_number(&self) -> TYPES::View {
        self.view_number
    }
}

impl<TYPES: NodeType> VidDisperseShare2<TYPES> {
    /// Create a vector of `VidDisperseShare` from `VidDisperse`
    pub fn from_vid_disperse(vid_disperse: AvidMDisperse<TYPES>) -> Vec<Self> {
        vid_disperse
            .shares
            .into_iter()
            .map(|(recipient_key, share)| Self {
                share,
                recipient_key,
                view_number: vid_disperse.view_number,
                payload_commitment: vid_disperse.payload_commitment,
                data_epoch_payload_commitment: vid_disperse.data_epoch_payload_commitment,
                epoch: vid_disperse.epoch,
                target_epoch: vid_disperse.target_epoch,
            })
            .collect()
    }

    /// Consume `self` and return a `Proposal`
    pub fn to_proposal(
        self,
        private_key: &<TYPES::SignatureKey as SignatureKey>::PrivateKey,
    ) -> Option<Proposal<TYPES, Self>> {
        let Ok(signature) =
            TYPES::SignatureKey::sign(private_key, self.payload_commitment.as_ref())
        else {
            tracing::error!("VID: failed to sign dispersal share payload");
            return None;
        };
        Some(Proposal {
            signature,
            _pd: PhantomData,
            data: self,
        })
    }

    /// Create `VidDisperse` out of an iterator to `VidDisperseShare`s
    pub fn to_vid_disperse<'a, I>(mut it: I) -> Option<AvidMDisperse<TYPES>>
    where
        I: Iterator<Item = &'a Self>,
    {
        let first_vid_disperse_share = it.next()?.clone();
        let payload_byte_len = first_vid_disperse_share.share.payload_byte_len();
        let mut share_map = BTreeMap::new();
        share_map.insert(
            first_vid_disperse_share.recipient_key,
            first_vid_disperse_share.share,
        );
        let mut vid_disperse = AvidMDisperse {
            view_number: first_vid_disperse_share.view_number,
            epoch: first_vid_disperse_share.epoch,
            target_epoch: first_vid_disperse_share.target_epoch,
            payload_commitment: first_vid_disperse_share.payload_commitment,
            data_epoch_payload_commitment: first_vid_disperse_share.data_epoch_payload_commitment,
            shares: share_map,
            payload_byte_len,
        };
        let _ = it.map(|vid_disperse_share| {
            vid_disperse.shares.insert(
                vid_disperse_share.recipient_key.clone(),
                vid_disperse_share.share.clone(),
            )
        });
        Some(vid_disperse)
    }

    /// Returns the payload length in bytes.
    pub fn payload_byte_len(&self) -> u32 {
        self.share.payload_byte_len() as u32
    }

    /// Split a VID share proposal into a proposal for each recipient.
    pub fn to_vid_share_proposals(
        vid_disperse: AvidMDisperse<TYPES>,
        signature: &<TYPES::SignatureKey as SignatureKey>::PureAssembledSignatureType,
    ) -> Vec<Proposal<TYPES, Self>> {
        vid_disperse
            .shares
            .into_iter()
            .map(|(recipient_key, share)| Proposal {
                data: Self {
                    share,
                    recipient_key,
                    view_number: vid_disperse.view_number,
                    payload_commitment: vid_disperse.payload_commitment,
                    data_epoch_payload_commitment: vid_disperse.data_epoch_payload_commitment,
                    epoch: vid_disperse.epoch,
                    target_epoch: vid_disperse.target_epoch,
                },
                signature: signature.clone(),
                _pd: PhantomData,
            })
            .collect()
    }

    /// Internally verify the share given necessary information
    ///
    /// # Errors
    #[allow(clippy::result_unit_err)]
    pub fn verify_share(&self, total_nodes: usize) -> std::result::Result<(), ()> {
        let avidm_param = init_avidm_param(total_nodes).map_err(|_| ())?;
        AvidMScheme::verify_share(&avidm_param, &self.payload_commitment, &self.share)
            .unwrap_or(Err(()))
    }
}
