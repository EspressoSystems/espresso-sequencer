use anyhow::{Context, Result, anyhow, ensure};
use espresso_types::{Header, Payload};
use hotshot_types::{
    data::{VidCommitment, VidCommon, ns_table::parse_ns_table},
    traits::block_contents::EncodeBytes,
    vid::{
        advz::{ADVZScheme, advz_scheme},
        avidm::{AvidMScheme, init_avidm_param},
        avidm_gf2::{AvidmGf2Scheme, init_avidm_gf2_param},
    },
};
use jf_advz::VidScheme;
use serde::{Deserialize, Serialize};

/// Information required to verify a payload.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct PayloadProof {
    /// The payload to be verified.
    payload: Payload,

    /// VID common data.
    ///
    /// This is data necessary to recompute the VID commitment of the payload, which can then be
    /// verified against a commitment in a previously-verified header.
    vid_common: VidCommon,
}

impl PayloadProof {
    /// Construct a [`PayloadProof`].
    ///
    /// Takes the payload to be verified, plus corresponding [`VidCommon`] data to allow a client to
    /// recompute and verify the commitment of the data.
    pub fn new(payload: Payload, vid_common: VidCommon) -> Self {
        Self {
            payload,
            vid_common,
        }
    }

    /// Verify a [`PayloadProof`].
    ///
    /// `total_weight` is the VID total weight of the stake table the block was dispersed with,
    /// which the light client derives itself rather than taking from the proof. If the data in
    /// this proof matches the expected `header`, the full payload data is returned.
    pub fn verify(self, header: &Header, total_weight: usize) -> Result<Payload> {
        Ok(self.verify_with_vid_common(header, total_weight)?.0)
    }

    /// Verify a [`PayloadProof`] and get the corresponding [`VidCommon`].
    ///
    /// Takes the same arguments as [`verify`](Self::verify). If the data in this proof matches
    /// the expected `header`, the full payload data is returned.
    pub fn verify_with_vid_common(
        self,
        header: &Header,
        total_weight: usize,
    ) -> Result<(Payload, VidCommon)> {
        ensure!(
            self.payload.ns_table() == header.ns_table(),
            "namespace table of payload does not match namespace table in header"
        );
        check_vid_param(&self.vid_common, total_weight)?;
        let commit = match &self.vid_common {
            VidCommon::V0(common) => {
                advz_scheme(ADVZScheme::get_num_storage_nodes(common) as usize)
                    .commit_only(self.payload.encode())
                    .map(VidCommitment::V0)
                    .context("computing ADVZ commitment")?
            },
            VidCommon::V1(avidm) => {
                let bytes = self.payload.encode();
                AvidMScheme::commit(
                    avidm,
                    &bytes,
                    parse_ns_table(bytes.len(), &header.ns_table().encode()),
                )
                .map(VidCommitment::V1)
                .map_err(|err| anyhow!("computing AvidM commitment: {err:#}"))?
            },
            VidCommon::V2(avidm_gf2) => {
                let bytes = self.payload.encode();
                AvidmGf2Scheme::commit(
                    &avidm_gf2.param,
                    &bytes,
                    parse_ns_table(bytes.len(), &header.ns_table().encode()),
                )
                .map(|(comm, _)| VidCommitment::V2(comm))
                .map_err(|err| anyhow!("computing AvidM commitment: {err:#}"))?
            },
        };
        ensure!(
            commit == header.payload_commitment(),
            "commitment of payload does not match commitment in header"
        );
        Ok((self.payload, self.vid_common))
    }
}

/// Check that `common` carries the VID parameters consensus derived from a stake table of
/// `total_weight`.
///
/// The commitment binds the payload but not these parameters, so a provider could otherwise pick
/// ones that make recomputing the commitment arbitrarily expensive, or that the scheme cannot
/// encode with at all.
pub(crate) fn check_vid_param(common: &VidCommon, total_weight: usize) -> Result<()> {
    let matches = match common {
        VidCommon::V0(common) => ADVZScheme::get_num_storage_nodes(common) as usize == total_weight,
        VidCommon::V1(param) => *param == init_avidm_param(total_weight)?,
        VidCommon::V2(common) => common.param == init_avidm_gf2_param(total_weight)?,
    };
    ensure!(
        matches,
        "VID common does not match the stake table, whose total weight is {total_weight}"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use espresso_types::{NamespaceId, NodeState, Transaction};
    use hotshot_types::{traits::block_contents::BlockPayload, vid::avidm::AvidMParam};

    use super::*;
    use crate::testing::TestClient;

    /// The commitment is checked against the header's namespace table, so the payload that comes
    /// back must carry that table too. A provider can pair genuine payload bytes with a different
    /// table of its own, which would reclassify the transactions into other namespaces.
    #[tokio::test]
    #[test_log::test]
    async fn test_verify_rejects_payload_with_forged_ns_table() {
        let client = TestClient::default();
        let tx = vec![1, 2, 3];
        client
            .add_block(
                1,
                vec![Transaction::new(NamespaceId::from(1u32), tx.clone())],
            )
            .await;
        let header = client.leaf(1).await.header().clone();
        let payload = client.payload(1).await;
        let vid_common = client.vid_common(1).await;
        let total_weight = client.vid_total_weight(1).await;

        let verified = PayloadProof::new(payload.clone(), vid_common.clone())
            .verify(&header, total_weight)
            .unwrap();
        assert_eq!(verified.ns_table(), header.ns_table());

        // The same transaction bytes under a different namespace ID.
        let other = Payload::from_transactions_sync(
            &[Transaction::new(NamespaceId::from(2u32), tx)],
            NodeState::mock_v3().chain_config,
        )
        .unwrap()
        .0;
        assert_ne!(other.ns_table(), payload.ns_table());
        let forged = Payload::from_bytes(payload.raw_payload(), other.ns_table());

        let err = PayloadProof::new(forged, vid_common)
            .verify(&header, total_weight)
            .unwrap_err();
        assert!(err.to_string().contains("namespace table"), "{err:#}");
    }

    /// The commitment does not bind the VID parameters a provider sends along with a payload, so
    /// they are checked against the stake table before they drive any computation.
    #[tokio::test]
    #[test_log::test]
    async fn test_verify_rejects_forged_vid_param() {
        let client = TestClient::default();
        let header = client.leaf(1).await.header().clone();
        let payload = client.payload(1).await;
        let VidCommon::V1(param) = client.vid_common(1).await else {
            panic!("the test client serves AvidM commons");
        };
        let forged = VidCommon::V1(AvidMParam {
            total_weights: usize::MAX,
            ..param
        });

        let err = PayloadProof::new(payload, forged)
            .verify(&header, client.vid_total_weight(1).await)
            .unwrap_err();
        assert!(err.to_string().contains("stake table"), "{err:#}");
    }
}
