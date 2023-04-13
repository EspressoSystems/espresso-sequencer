pragma solidity ^0.8.16;

import "forge-std/console.sol";

contract HotShot {
    // TODO comment
    uint256 constant PRIME_FIELD_MODULUS = 21888242871839275222246405745257275088696311157297823662689037894645226208583;

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

    // TODO gas optimization
    function bytes32ToUint8Array(bytes32 input) public pure returns (uint8[] memory output) {
        output = new uint8[](32);
        for (uint256 i = 0; i < 32; i++) {
            output[i] = uint8(uint256(input) / (2 ** (8 * (31 - i))));
        }
    }

    // Helpers
    function expand(uint8[] memory message) public pure returns (uint8[] memory) {
        uint8 block_size = 48;
        uint256 b_len = 32; // Output length of sha256 in number of bytes
        uint8 ell = 2; // (n+(b_len-1))/b_len where n=48

        // Final value of buffer must be: z_pad || message || lib_str || 0 || dst_prime

        uint8 zero_u8 = 0;
        uint8 one_u8 = 1;

        bytes memory buffer;

        // TODO optimize gas?
        // z_pad
        for (uint256 i = 0; i < block_size; i++) {
            if (i == 0) {
                buffer = abi.encodePacked(zero_u8);
            } else {
                buffer = abi.encodePacked(buffer, zero_u8);
            }
        }

        // message
        for (uint256 i = 0; i < message.length; i++) {
            buffer = abi.encodePacked(buffer, message[i]);
        }

        // lib_str
        buffer = abi.encodePacked(buffer, zero_u8);
        buffer = abi.encodePacked(buffer, block_size);

        // 0 separator
        uint8 single_zero = zero_u8; //
        buffer = abi.encodePacked(buffer, single_zero);

        // dst_prime = [1,1]
        uint8[2] memory dst_prime = [1, 1]; // TODO how to pass dst_prime directly to abi.encodePacked?

        buffer = abi.encodePacked(buffer, dst_prime[0], dst_prime[1]);

        bytes32 b0 = keccak256(buffer);

        buffer = abi.encodePacked(b0);
        buffer = abi.encodePacked(buffer, one_u8);
        buffer = abi.encodePacked(buffer, dst_prime[0], dst_prime[1]);

        bytes32 bi = keccak256(buffer);

        // Building uniform_bytes
        uint8[] memory uniform_bytes = new uint8[](block_size);

        // TODO gas optimizations
        // Copy bi into uniform_bytes
        uint8[] memory bi_u8arr = bytes32ToUint8Array(bi);
        for (uint256 i = 0; i < bi_u8arr.length; i++) {
            uniform_bytes[i] = bi_u8arr[i];
        }

        uint8[] memory b0_u8arr = bytes32ToUint8Array(b0);

        // In our case ell=2 so we do not have an outer loop
        // https://github.com/arkworks-rs/algebra/blob/1f7b3c6b215e98fa3130b39d2967f6b43df41e04/ff/src/fields/field_hashers/expander/mod.rs#L100

        for (uint256 j = 0; j < b_len; j++) {
            // uint8 v = b0_u8arr[i] ^ bi_u8arr[i];
            if (j == 0) {
                buffer = abi.encodePacked(b0_u8arr[j] ^ bi_u8arr[j]); // v
            } else {
                buffer = abi.encodePacked(buffer, b0_u8arr[j] ^ bi_u8arr[j]); // buffer,v
            }
        }
        buffer = abi.encodePacked(buffer, ell);
        buffer = abi.encodePacked(buffer, dst_prime[0], dst_prime[1]); // TODO refactor?

        bi = keccak256(buffer);
        bi_u8arr = bytes32ToUint8Array(bi);

        //uint256 number_of_extra_elements = block_size - b_len; // Complete until block_size elements
        for (uint256 i = 0; i < block_size - b_len; i++) {
            uniform_bytes[b_len + i] = bi_u8arr[i];
        }

        return uniform_bytes;
    }

    function hash_to_field(uint8[] memory message) public pure returns (uint256) {
        uint8[] memory uniform_bytes = expand(message);

        // Reverse uniform_bytes
        // TODO improve gas
        uint256 n = uniform_bytes.length;
        uint8[] memory uniform_bytes_reverted = new uint8[](n);
        for (uint256 i = 0; i < n; i++) {
            uniform_bytes_reverted[i] = uniform_bytes[n - i - 1];
        }

        // Following https://github.com/arkworks-rs/algebra/blob/bc991d44c5e579025b7ed56df3d30267a7b9acac/ff/src/fields/prime.rs#L72

        // Do the split

        // Create the initial field elements

        // Update the field element with the extra bytes

        uint256 res = 22;
        return res;
    }

    function field_from_random_bytes(uint8[] memory input) public pure returns (uint256) {
        // Adapted from https://github.com/arkworks-rs/algebra/blob/1f7b3c6b215e98fa3130b39d2967f6b43df41e04/ff/src/fields/models/fp/mod.rs#L246
        // Note that we do not need the serde logic.

        // constant field element to multiply the input with
        // See https://github.com/arkworks-rs/algebra/blob/1f7b3c6b215e98fa3130b39d2967f6b43df41e04/ff/src/fields/models/fp/montgomery_backend.rs#L23
        // Represented as BigInt in little endian, [u64;4]: v = [15230403791020821917, 754611498739239741, 7381016538464732716,  1011752739694698287]

        // TODO hardcode value
        // TODO document, reference to arkwors
        uint256 v = 15230403791020821917 + 2 ** 64 * 754611498739239741 + 2 ** 128 * 7381016538464732716
            + 2 ** 192 * 1011752739694698287;

        // TODO optimize
        uint256 r = 0;
        for (uint256 i = 0; i < input.length; i++) {
            r += 2 ** (8 * i) * input[i];
        }

        assert(r != PRIME_FIELD_MODULUS);

        uint256 res;
        assembly {
            res := mulmod(r, v, PRIME_FIELD_MODULUS)
        }

        return res;
    }
}
