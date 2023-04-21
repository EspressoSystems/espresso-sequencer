pragma solidity ^0.8.0;

import {BN254} from "bn254/BN254.sol";
import {BLSSig} from "./libraries/BLSSig.sol";

contract BLSTest {
    /// This contract is for testing purposes only

    function hash_to_field(bytes memory message) public pure returns (uint256) {
        return BLSSig.hash_to_field(message);
    }

    function hash_to_curve(bytes memory input) public view returns (uint256, uint256) {
        return BLSSig.hash_to_curve(input);
    }

    function verify_bls_sig(bytes memory message, BN254.G1Point memory sig, BN254.G2Point memory pk) public view {
        BLSSig.verify_bls_sig(message, sig, pk);
    }
}
