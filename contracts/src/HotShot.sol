pragma solidity ^0.8.13;

import "forge-std/console.sol";

contract HotShot {

    mapping(uint => uint256) public hotshotBlocksCommitments;
    event newHotShotBlockCommitment(uint _blockNumber, uint256 comm);

    function publishHotShotBlockCommitment(uint _blockNumber, uint256 _comm, bytes memory _qc) public {

        // TODO Check the QC
        console.logBytes(_qc);

        hotshotBlocksCommitments[_blockNumber] = _comm;
        emit newHotShotBlockCommitment(_blockNumber, _comm);
    }

    function getHotShotBlockCommitment(uint _blockNumber) public view returns (uint256) {
        return hotshotBlocksCommitments[_blockNumber];
    }
}