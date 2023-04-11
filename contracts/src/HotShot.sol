pragma solidity ^0.8.16;

import "forge-std/console.sol";

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

    function expand(uint8[] memory message) public pure returns (bytes32) {
        uint8 block_size = 48;
        uint256 b_len = 32; // Output length of sha256 in number of bytes
        uint256 ell = 2; // n+(b_len-1))/b_len where n=48

        // Final value of buffer must be: z_pad || message || lib_str || 0 || dst_prime

        uint8 zero_u8 = 0;
        uint8 one_u8 = 1;

        bytes memory buffer = abi.encodePacked(zero_u8);

        // TODO optimize gas?
        // z_pad
        for (uint256 i = 0; i < block_size - 1; i++) {
            // block_size -1 because we already have a 0
            buffer = abi.encodePacked(buffer, zero_u8);
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
        buffer = abi.encodePacked(buffer, one_u8);
        buffer = abi.encodePacked(buffer, one_u8);

        bytes32 b0 = keccak256(buffer);

        // bytes memory b = abi.encodePacked(bi);
        return b0;
    }
}
