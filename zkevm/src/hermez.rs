use crate::EvmTransaction;
use ethers::prelude::*;
use std::borrow::Borrow;

/// Encode transactions as expected by Hermez.
///
/// Hermez uses a non-standard EVM transaction encoding which mixes the legacy (for the base
/// transaction) and EIP-1559 (for the signature) encodings. This implementation is a direct port
/// from Go of the `state.helper.EncodeTransactions` function in the Hermez zkEVM node.
pub fn encode_transactions<T: Borrow<EvmTransaction>>(txs: impl IntoIterator<Item = T>) -> Bytes {
    txs.into_iter()
        .flat_map(|tx| {
            let tx = tx.borrow();

            let Signature { v, r, s } = tx.signature();
            let sign = (1 - (v & 1)) as u8;

            let tx_coded_rlp = tx.rlp_base();

            let new_v = 27 + sign;

            // The Hermez Go implementation does some hacky format-to-hex-with-padding and then
            // parsing hex in order to get the byte representation of `r`, `s`, and `new_v` padded
            // out to 32, 32, and 1 bytes, respectively. We can use Rust's strong typing to avoid
            // this step, since all three parts of the signature are already stored in types with
            // the appropriate lengths: `v` and `r` are `U256`, which is 32 bytes, and `new_v` is a
            // `u8`, which is 1 byte. Therefore we can simply append the byte representation of
            // these integers directly.
            let mut sig_bytes = [0; 65];
            r.to_big_endian(&mut sig_bytes[0..32]);
            s.to_big_endian(&mut sig_bytes[32..64]);
            sig_bytes[64] = new_v;

            tx_coded_rlp.into_iter().chain(sig_bytes)
        })
        .collect::<Vec<u8>>()
        .into()
}
