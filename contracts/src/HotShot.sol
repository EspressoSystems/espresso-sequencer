pragma solidity ^0.8.0;

import {BN254} from "./libraries/BN254.sol";
import {BytesLib} from "./libraries/BytesLib.sol";

contract HotShot {
    uint256 public constant MAX_BLOCKS = 1000;
    mapping(uint256 => uint256) public commitments;
    uint256 public blockHeight;

    event NewBlocks(uint256 firstBlockNumber, uint256 numBlocks);

    error WrongNumberOfQCs(uint256 numBlocks, uint256 numQCs);
    error TooManyBlocks(uint256 numBlocks);
    error InvalidQC(uint256 blockNumber);

    function verifyQC(uint256, /*blockNumber*/ uint256, /*commitment*/ bytes calldata /*qc*/ )
        private
        pure
        returns (bool)
    {
        // TODO Check the QC
        // TODO Check the block number
        return true;
    }

    function newBlocks(uint256[] calldata newCommitments, bytes[] calldata qcs) external {
        if (newCommitments.length != qcs.length) {
            revert WrongNumberOfQCs(newCommitments.length, qcs.length);
        }
        if (newCommitments.length > MAX_BLOCKS) {
            revert TooManyBlocks(newCommitments.length);
        }

        uint256 firstBlockNumber = blockHeight;
        for (uint256 i = 0; i < newCommitments.length; ++i) {
            if (!verifyQC(blockHeight, newCommitments[i], qcs[i])) {
                revert InvalidQC(blockHeight);
            }

            commitments[blockHeight] = newCommitments[i];
            blockHeight += 1;
        }

        emit NewBlocks(firstBlockNumber, newCommitments.length);
    }

    ////// BLS signature verification

    // Helpers

    // TODO gas optimization
    function bytes32ToUint8Array(bytes32 input) public pure returns (uint8[] memory output) {
        output = new uint8[](32);
        for (uint256 i = 0; i < 32; i++) {
            output[i] = uint8(uint256(input) / (2 ** (8 * (31 - i))));
        }
    }

    // TODO gas optimization
    function big_int_from_bytes(uint8[] memory input) private pure returns (uint256) {
        uint256 r = 0;
        for (uint256 i = 0; i < input.length; i++) {
            r += 2 ** (8 * i) * input[i];
        }
        return r;
    }

    function expand(bytes memory message) public pure returns (uint8[] memory) {
        uint8 block_size = 48;
        uint256 b_len = 32; // Output length of sha256 in number of bytes
        uint8 ell = 2; // (n+(b_len-1))/b_len where n=48

        // Final value of buffer must be: z_pad || message || lib_str || 0 || dst_prime

        uint8 zero_u8 = 0;
        uint8 one_u8 = 1;

        // z_pad
        bytes memory buffer = new bytes(block_size);

        // message
        buffer = abi.encodePacked(buffer, message);

        // lib_str
        buffer = abi.encodePacked(buffer, zero_u8);
        buffer = abi.encodePacked(buffer, block_size);

        // 0 separator
        uint8 single_zero = zero_u8; //
        buffer = abi.encodePacked(buffer, single_zero);

        // dst_prime = [1,1]
        bytes2 dst_prime = 0x0101;

        buffer = abi.encodePacked(buffer, dst_prime);

        bytes32 b0 = keccak256(buffer);

        buffer = abi.encodePacked(b0);
        buffer = abi.encodePacked(buffer, one_u8);
        buffer = abi.encodePacked(buffer, dst_prime);

        bytes32 bi = keccak256(buffer);

        // Building uniform_bytes
        uint8[] memory uniform_bytes = new uint8[](block_size);

        // Copy bi into uniform_bytes
        uint8[] memory bi_u8arr = bytes32ToUint8Array(bi);
        for (uint256 i = 0; i < bi_u8arr.length; i++) {
            uniform_bytes[i] = bi_u8arr[i];
        }

        uint8[] memory b0_u8arr = bytes32ToUint8Array(b0);

        // In our case ell=2 so we do not have an outer loop
        // https://github.com/arkworks-rs/algebra/blob/1f7b3c6b215e98fa3130b39d2967f6b43df41e04/ff/src/fields/field_hashers/expander/mod.rs#L100

        for (uint256 j = 0; j < b_len; j++) {
            uint8 v = b0_u8arr[j] ^ bi_u8arr[j];
            if (j == 0) {
                buffer = abi.encodePacked(v); // v
            } else {
                buffer = abi.encodePacked(buffer, v); // buffer,v
            }
        }
        buffer = abi.encodePacked(buffer, ell);
        buffer = abi.encodePacked(buffer, dst_prime);

        bi = keccak256(buffer);
        bi_u8arr = bytes32ToUint8Array(bi);

        //uint256 number_of_extra_elements = block_size - b_len; // Complete until block_size elements
        for (uint256 i = 0; i < block_size - b_len; i++) {
            uniform_bytes[b_len + i] = bi_u8arr[i];
        }

        return uniform_bytes;
    }

    function hash_to_field(bytes memory message) public pure returns (uint256) {
        uint8[] memory uniform_bytes = expand(message);

        // Reverse uniform_bytes
        uint256 n = uniform_bytes.length;
        assert(n == 48);
        uint8[] memory uniform_bytes_reverted = new uint8[](n);

        for (uint256 i = 0; i < n; i++) {
            uniform_bytes_reverted[i] = uniform_bytes[n - i - 1];
        }

        // Following https://github.com/arkworks-rs/algebra/blob/bc991d44c5e579025b7ed56df3d30267a7b9acac/ff/src/fields/prime.rs#L72

        // Do the split
        uint256 num_bytes_directly_to_convert = 31; // Fixed for Fq

        // Create the initial field element
        uint256 res = 0;

        // Process the second slice
        uint8[] memory second_slice = new uint8[](num_bytes_directly_to_convert);

        for (uint256 i = 0; i < num_bytes_directly_to_convert; i++) {
            second_slice[i] = uniform_bytes_reverted[n - num_bytes_directly_to_convert + i];
        }

        res = big_int_from_bytes(second_slice);

        uint256 window_size = 256;
        uint256 p = BN254.P_MOD;
        // Handle the first slice
        uint256 arr_size = n - num_bytes_directly_to_convert;
        for (uint256 i = 0; i < arr_size; i++) {
            // Compute field element from a single byte
            uint256 field_elem = uint256(uniform_bytes_reverted[arr_size - i - 1]); // In reverse

            res = mulmod(res, window_size, p);
            res = addmod(res, field_elem, p);
        }
        return res;
    }

    function hash_to_curve(bytes memory input) public view returns (uint256, uint256) {
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

    function verify_bls_sig(bytes memory message, BN254.G1Point memory sig, BN254.G2Point memory pk)
        public
        view
        returns (bool)
    {
        uint256 x;
        uint256 y;

        // Check the signature is a valid G1 point
        // Note: checking pk belong to G2 is not possible in practice https://ethresear.ch/t/fast-mathbb-g-2-subgroup-check-in-bn254/13974
        BN254.validateG1Point(sig);

        // Hardcoded suffix "BLS_SIG_BN254G1_XMD:KECCAK_NCTH_NUL_"
        bytes memory csid_suffix = hex"424c535f5349475f424e32353447315f584d443a4b454343414b5f4e4354485f4e554c5f";

        bytes memory input = BytesLib.concat(message, csid_suffix);

        (x, y) = hash_to_curve(input);
        BN254.G1Point memory hash = BN254.G1Point(x, y);

        bool res = BN254.pairingProd2(hash, pk, BN254.negate(sig), BN254.P2());

        return res;
    }
}
