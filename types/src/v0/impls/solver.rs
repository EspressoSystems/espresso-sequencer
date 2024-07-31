use committable::{Commitment, Committable};
use hotshot::types::SignatureKey;

use super::v0_3::{RollupRegistrationBody, RollupUpdatebody};

impl Committable for RollupRegistrationBody {
    fn tag() -> String {
        "ROLLUP_REG".to_string()
    }

    fn commit(&self) -> Commitment<Self> {
        // todo (ab): expose to_fixed_bytes() method of fee amount

        let active: [u8; 1] = if self.active { [1] } else { [0] };

        let mut comm = committable::RawCommitmentBuilder::new(&Self::tag())
            .u64_field("namespace_id", u64::from(self.namespace_id))
            .var_size_field("reserve_url", self.reserve_url.as_str().as_ref())
            .fixed_size_field("reserve_price", &self.reserve_price.to_fixed_bytes())
            .fixed_size_field("active", &active)
            .constant_str("signature_keys");

        for key in self.signature_keys.iter() {
            comm = comm.var_size_bytes(&key.to_bytes());
        }

        comm = comm
            .var_size_field("signature_key", &self.signature_key.to_bytes())
            .var_size_field("text", self.text.as_bytes());

        comm.finalize()
    }
}

impl Committable for RollupUpdatebody {
    fn tag() -> String {
        "ROLLUP_UPDATE".to_string()
    }

    fn commit(&self) -> Commitment<Self> {
        // todo (ab): expose to_fixed_bytes() method of fee amount

        let mut comm = committable::RawCommitmentBuilder::new(&Self::tag())
            .u64_field("namespace_id", u64::from(self.namespace_id));

        if let Some(reserve_url) = &self.reserve_url {
            comm = comm.var_size_field("reserve_url", reserve_url.as_str().as_ref())
        }

        if let Some(rp) = self.reserve_price {
            comm = comm.fixed_size_field("reserve_price", &rp.to_fixed_bytes())
        };

        if let Some(active) = self.active {
            let active: [u8; 1] = if active { [1] } else { [0] };

            comm = comm.fixed_size_field("active", &active);
        }

        if let Some(keys) = &self.signature_keys {
            comm = comm.constant_str("signature_keys");
            for key in keys.iter() {
                comm = comm.var_size_bytes(&key.to_bytes());
            }
        }

        comm = comm.var_size_field("signature_key", &self.signature_key.to_bytes());

        if let Some(text) = &self.text {
            comm = comm.var_size_field("text", text.as_bytes());
        }

        comm.finalize()
    }
}
