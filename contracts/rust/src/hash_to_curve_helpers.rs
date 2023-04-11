// TODO remove when test is done
// Only for testing purpose
// This is taken from https://github.com/arkworks-rs/algebra/blob/master/ff/src/fields/field_hashers/expander/mod.rs
// Easier to test here than forking arkworks

use ark_std::vec::Vec;
use digest::DynDigest;

pub trait Expander {
    fn construct_dst_prime(&self) -> Vec<u8>;
    fn expand(&self, msg: &[u8], length: usize) -> Vec<u8>;
}

pub(super) struct ExpanderXmd<T: DynDigest + Clone> {
    pub(super) hasher: T,
    pub(super) dst: Vec<u8>,
    pub(super) block_size: usize,
}

const MAX_DST_LENGTH: usize = 255;

const LONG_DST_PREFIX: [u8; 17] = [
    //'H', '2', 'C', '-', 'O', 'V', 'E', 'R', 'S', 'I', 'Z', 'E', '-', 'D', 'S', 'T', '-',
    0x48, 0x32, 0x43, 0x2d, 0x4f, 0x56, 0x45, 0x52, 0x53, 0x49, 0x5a, 0x45, 0x2d, 0x44, 0x53, 0x54,
    0x2d,
];

impl<T: DynDigest + Clone> Expander for ExpanderXmd<T> {
    fn construct_dst_prime(&self) -> Vec<u8> {
        let mut dst_prime = if self.dst.len() > MAX_DST_LENGTH {
            let mut hasher = self.hasher.clone();
            hasher.update(&LONG_DST_PREFIX);
            hasher.update(&self.dst);
            hasher.finalize_reset().to_vec()
        } else {
            self.dst.clone()
        };
        dst_prime.push(dst_prime.len() as u8);
        dst_prime
    }
    fn expand(&self, msg: &[u8], n: usize) -> Vec<u8> {
        let mut hasher = self.hasher.clone();
        // output size of the hash function, e.g. 32 bytes = 256 bits for sha2::Sha256
        let b_len = hasher.output_size();
        let ell = (n + (b_len - 1)) / b_len;
        assert!(
            ell <= 255,
            "The ratio of desired output to the output size of hash function is too large!"
        );

        let dst_prime = self.construct_dst_prime();
        let z_pad: Vec<u8> = vec![0; self.block_size];
        // // Represent `len_in_bytes` as a 2-byte array.
        // // As per I2OSP method outlined in https://tools.ietf.org/pdf/rfc8017.pdf,
        // // The program should abort if integer that we're trying to convert is too large.
        assert!(n < (1 << 16), "Length should be smaller than 2^16");
        let lib_str: [u8; 2] = (n as u16).to_be_bytes();

        hasher.update(&z_pad);
        hasher.update(msg);
        hasher.update(&lib_str);
        hasher.update(&[0u8]);
        hasher.update(&dst_prime);

        let b0 = hasher.finalize_reset();

        // hasher.update(&b0);
        // hasher.update(&[1u8]);
        // hasher.update(&dst_prime);
        // let mut bi = hasher.finalize_reset();
        //
        // let mut uniform_bytes: Vec<u8> = Vec::with_capacity(n);
        // uniform_bytes.extend_from_slice(&bi);
        // for i in 2..=ell {
        //     // update the hasher with xor of b_0 and b_i elements
        //     for (l, r) in b0.iter().zip(bi.iter()) {
        //         hasher.update(&[*l ^ *r]);
        //     }
        //     hasher.update(&[i as u8]);
        //     hasher.update(&dst_prime);
        //     bi = hasher.finalize_reset();
        //     uniform_bytes.extend_from_slice(&bi);
        // }
        // uniform_bytes[0..n].to_vec()

        b0.to_vec()
    }
}
