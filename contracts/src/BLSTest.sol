pragma solidity ^0.8.0;

import {BN254} from "bn254/BN254.sol";
import {BLSSig} from "./libraries/BLSSig.sol";

contract BLSTest {
    /// This contract is for testing purposes only

    function hashToField(bytes memory message) public pure returns (uint256) {
        return BLSSig.hashToField(message);
    }

    function hashToCurve(bytes memory input) public view returns (uint256, uint256) {
        return BLSSig.hashToCurve(input);
    }

    function verifyBlsSig(bytes memory message, BN254.G1Point memory sig, BN254.G2Point memory pk) public view {
        BLSSig.verifyBlsSig(message, sig, pk);
    }
}
