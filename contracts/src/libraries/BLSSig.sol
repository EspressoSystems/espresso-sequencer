// SPDX-License-Identifier: GPL-3.0-or-later
//
// Copyright (c) 2022 Espresso Systems (espressosys.com)
// This file is part of the Configurable Asset Privacy for Ethereum (CAPE) library.
//
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
// You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.

pragma solidity ^0.8.0;

import {BN254} from "bn254/BN254.sol";
import {BytesLib} from "solidity-bytes-utils/BytesLib.sol";

library BLSSig {
    // This library implements the verification of the BLS signature scheme over the BN254 curve
    // following the rust implementation at https://github.com/EspressoSystems/jellyfish/blob/e1e683c287f20160738e6e737295dd8f9e70577a/primitives/src/signatures/bls_over_bn254.rs

    // TODO gas optimization
    function uint256FromBytesLittleEndian(uint8[] memory input) private pure returns (uint256) {
        uint256 r = 0;
        for (uint256 i = 0; i < input.length; i++) {
            r += 2 ** (8 * i) * input[i];
        }
        return r;
    }

    /// @dev Takes a sequence of bytes and turn in into another sequence of bytes with fixed size. Equivalent of https://github.com/arkworks-rs/algebra/blob/1f7b3c6b215e98fa3130b39d2967f6b43df41e04/ff/src/fields/field_hashers/expander/mod.rs#L37
    /// @param message message to be "expanded"
    /// @return fixed size array of bytes
    function expand(bytes memory message) internal pure returns (bytes memory) {
        uint8 block_size = 48;
        uint256 b_len = 32; // Output length of sha256 in number of bytes
        bytes1 ell = 0x02; // (n+(b_len-1))/b_len where n=48

        // Final value of buffer must be: z_pad || message || lib_str || 0 || dst_prime

        bytes1 zero_u8 = 0x00;
        bytes1 one_u8 = 0x01;

        // z_pad
        bytes memory buffer = new bytes(block_size);

        // message
        buffer = bytes.concat(buffer, message);

        // lib_str
        buffer = bytes.concat(buffer, zero_u8, bytes1(block_size));

        // 0 separator
        bytes1 single_zero = zero_u8;
        buffer = bytes.concat(buffer, single_zero);

        // dst_prime = [1,1]
        bytes2 dst_prime = 0x0101;

        buffer = bytes.concat(buffer, dst_prime);

        bytes32 b0 = keccak256(buffer);

        buffer = bytes.concat(b0, one_u8, dst_prime);

        bytes32 bi = keccak256(buffer);

        // Building uniform_bytes
        bytes memory uniform_bytes = new bytes(block_size);

        // Copy bi into uniform_bytes
        bytes memory bi_bytes = bytes.concat(bi);
        for (uint256 i = 0; i < bi_bytes.length; i++) {
            uniform_bytes[i] = bi_bytes[i];
        }

        bytes memory b0_bytes = bytes.concat(b0);

        // In our case ell=2 so we do not have an outer loop
        // https://github.com/arkworks-rs/algebra/blob/1f7b3c6b215e98fa3130b39d2967f6b43df41e04/ff/src/fields/field_hashers/expander/mod.rs#L100

        buffer = "";
        for (uint256 j = 0; j < b_len; j++) {
            bytes1 v = bytes1(b0_bytes[j] ^ bi_bytes[j]);
            buffer = bytes.concat(buffer, v);
        }
        buffer = bytes.concat(buffer, ell, dst_prime);

        bi = keccak256(buffer);
        bi_bytes = bytes.concat(bi);

        for (uint256 i = 0; i < block_size - b_len; i++) {
            uniform_bytes[b_len + i] = bi_bytes[i];
        }

        return uniform_bytes;
    }

    /// @dev Hash a sequence of bytes to a field element in Fq. Equivalent of https://github.com/arkworks-rs/algebra/blob/1f7b3c6b215e98fa3130b39d2967f6b43df41e04/ff/src/fields/field_hashers/mod.rs#L65
    /// @param message input message to be hashed
    /// @return field element in Fq
    function hash_to_field(bytes memory message) internal pure returns (uint256) {
        bytes memory uniform_bytes = expand(message);

        // Reverse uniform_bytes
        uint256 n = uniform_bytes.length;
        assert(n == 48);
        bytes memory uniform_bytes_reverted = new bytes(n);

        for (uint256 i = 0; i < n; i++) {
            uniform_bytes_reverted[i] = uniform_bytes[n - i - 1];
        }

        // Following https://github.com/arkworks-rs/algebra/blob/bc991d44c5e579025b7ed56df3d30267a7b9acac/ff/src/fields/prime.rs#L72

        // Do the split
        uint256 num_bytes_directly_to_convert = 31; // Fixed for Fq

        // Process the second slice
        uint8[] memory second_slice = new uint8[](num_bytes_directly_to_convert);

        for (uint256 i = 0; i < num_bytes_directly_to_convert; i++) {
            second_slice[i] = uint8(uniform_bytes_reverted[n - num_bytes_directly_to_convert + i]);
        }

        uint256 res = uint256FromBytesLittleEndian(second_slice);

        uint256 window_size = 256;
        uint256 p = BN254.P_MOD;
        // Handle the first slice
        uint256 arr_size = n - num_bytes_directly_to_convert;
        for (uint256 i = 0; i < arr_size; i++) {
            // Compute field element from a single byte
            uint256 field_elem = uint256(uint8(uniform_bytes_reverted[arr_size - i - 1])); // In reverse

            res = mulmod(res, window_size, p);
            res = addmod(res, field_elem, p);
        }
        return res;
    }

    /// @dev Hash a sequence of bytes to a group element in BN254.G_1. We use the hash-and-pray algorithm for now. Rust implementation can be found at https://github.com/EspressoSystems/jellyfish/blob/e1e683c287f20160738e6e737295dd8f9e70577a/primitives/src/signatures/bls_over_bn254.rs#L318
    /// @param input message to be hashed
    /// @return group element in G_1
    function hash_to_curve(bytes memory input) internal view returns (uint256, uint256) {
        uint256 x = hash_to_field(input);

        uint256 p = BN254.P_MOD;

        uint256 b = 3;

        uint256 Y = mulmod(x, x, p);
        Y = mulmod(Y, x, p);
        Y = addmod(Y, b, p);

        // Check Y is a quadratic residue
        uint256 y;
        bool is_qr;
        (is_qr, y) = BN254.quadraticResidue(Y);

        while (!is_qr) {
            x = addmod(x, 1, p);
            Y = mulmod(x, x, p);
            Y = mulmod(Y, x, p);
            Y = addmod(Y, b, p);
            (is_qr, y) = BN254.quadraticResidue(Y);
        }

        return (x, y);
    }

    /// @dev Verify a bls signature. Reverts if the signature is invalid
    /// @param message message to check the signature against
    /// @param sig signature represented as a point in BN254.G_1
    /// @param pk public key represented as a point in BN254.G_2
    function verify_bls_sig(bytes memory message, BN254.G1Point memory sig, BN254.G2Point memory pk) internal view {
        // Check the signature is a valid G1 point
        // Note: checking pk belong to G2 is not possible in practice https://ethresear.ch/t/fast-mathbb-g-2-subgroup-check-in-bn254/13974
        BN254.validateG1Point(sig);

        // Hardcoded suffix "BLS_SIG_BN254G1_XMD:KECCAK_NCTH_NUL_" See https://github.com/EspressoSystems/jellyfish/blob/e1e683c287f20160738e6e737295dd8f9e70577a/primitives/src/constants.rs#L30
        bytes memory csid_suffix = "BLS_SIG_BN254G1_XMD:KECCAK_NCTH_NUL_";

        bytes memory input = bytes.concat(message, csid_suffix);

        (uint256 x, uint256 y) = hash_to_curve(input);
        BN254.G1Point memory hash = BN254.G1Point(x, y);

        bool res = BN254.pairingProd2(hash, pk, BN254.negate(sig), BN254.P2());

        require(res, "BLS signature verification failed.");
    }
}
