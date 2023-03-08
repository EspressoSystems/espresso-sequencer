pragma solidity ^0.8.13;

import "forge-std/console.sol";

contract HotShot {
    mapping(uint256 => uint256) public commitments;

    event NewBlock(uint256 _blockNumber, uint256 commmitment);

    function verifyQC(uint256 blockNumber, uint256 commitment, bytes memory qc) private pure returns (bool) {
        // TODO Check the QC
        return true;
    }

    function newBlock(uint256 blockNumber, uint256 commitment, bytes memory qc) public {
        require(verifyQC(blockNumber, commitment, qc), "Invalid QC");

        commitments[blockNumber] = commitment;
        emit NewBlock(blockNumber, commitment);
    }
}
