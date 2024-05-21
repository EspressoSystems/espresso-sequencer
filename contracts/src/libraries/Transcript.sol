// SPDX-License-Identifier: Unlicensed

pragma solidity ^0.8.0;

import { BN254, Utils } from "bn254/BN254.sol";
import { BytesLib } from "solidity-bytes-utils/BytesLib.sol";
import { IPlonkVerifier } from "../interfaces/IPlonkVerifier.sol";

library Transcript {
    struct TranscriptData {
        bytes transcript;
        bytes32 state;
    }

    // ================================
    // Primitive functions
    // ================================
    function appendMessage(TranscriptData memory self, bytes memory message) internal pure {
        self.transcript = abi.encodePacked(self.transcript, message);
    }

    // ================================
    // Transcript APIs
    // ================================
    function appendChallenge(TranscriptData memory self, uint256 challenge) internal pure {
        self.transcript = abi.encodePacked(self.transcript, Utils.reverseEndianness(challenge));
    }

    function appendCommitments(TranscriptData memory self, BN254.G1Point[] memory comms)
        internal
        pure
    {
        for (uint256 i = 0; i < comms.length; i++) {
            appendCommitment(self, comms[i]);
        }
    }

    function appendCommitment(TranscriptData memory self, BN254.G1Point memory comm)
        internal
        pure
    {
        self.transcript = abi.encodePacked(self.transcript, BN254.g1Serialize(comm));
    }

    function getAndAppendChallenge(TranscriptData memory self) internal pure returns (uint256) {
        bytes32 hash;

        bytes32 a = self.state;
        bytes memory b = self.transcript;

        // Computes keccak256(bytes32 a, bytes b)
        assembly {
            // Load the length of 'b'
            let bLength := mload(b)

            // Allocate memory for 'a' + 'b'
            let data := mload(0x40) // Load free memory pointer

            // Store 'a' in memory
            mstore(data, a)

            // Copy 'self.transcript' to memory after 'self.state'
            let dataOffset := add(data, 32) // Start right after 'self.state'
            for { let i := 0 } lt(i, bLength) { i := add(i, 0x20) } {
                mstore(add(dataOffset, i), mload(add(add(b, i), 0x20)))
            }

            // Compute the keccak256 hash of the data
            hash := keccak256(data, add(32, bLength))
        }

        self.state = hash;

        uint256 ret = uint256(hash) % BN254.R_MOD;
        return ret;
    }

    /// @dev Append the verifying key and the public inputs to the transcript.
    /// @param verifyingKey verifying key
    /// @param publicInput a list of field elements
    function appendVkAndPubInput(
        TranscriptData memory self,
        IPlonkVerifier.VerifyingKey memory verifyingKey,
        uint256[] memory publicInput
    ) internal pure {
        uint32 sizeInBits = 254;

        self.transcript = abi.encodePacked(
            self.transcript,
            BytesLib.slice(abi.encodePacked(Utils.reverseEndianness(sizeInBits)), 0, 4), // Fr field
                // size in bits
            BytesLib.slice(abi.encodePacked(Utils.reverseEndianness(verifyingKey.domainSize)), 0, 8), // domain
                // size
            BytesLib.slice(abi.encodePacked(Utils.reverseEndianness(verifyingKey.numInputs)), 0, 8) // number
                // of inputs
        );

        // =====================
        // k: coset representatives
        // =====================
        // Currently, K is hardcoded, and there are 5 of them since
        // # wire types == 5

        self.transcript = abi.encodePacked(
            self.transcript,
            Utils.reverseEndianness(0x1), // k0 = 1
            Utils.reverseEndianness(
                0x2f8dd1f1a7583c42c4e12a44e110404c73ca6c94813f85835da4fb7bb1301d4a
            ), // k1
            Utils.reverseEndianness(
                0x1ee678a0470a75a6eaa8fe837060498ba828a3703b311d0f77f010424afeb025
            ), // k2
            Utils.reverseEndianness(
                0x2042a587a90c187b0a087c03e29c968b950b1db26d5c82d666905a6895790c0a
            ), // k3
            Utils.reverseEndianness(
                0x2e2b91456103698adf57b799969dea1c8f739da5d8d40dd3eb9222db7c81e881
            ) // k4
        );

        // selectors
        self.transcript = abi.encodePacked(
            self.transcript,
            BN254.g1Serialize(verifyingKey.q1),
            BN254.g1Serialize(verifyingKey.q2),
            BN254.g1Serialize(verifyingKey.q3),
            BN254.g1Serialize(verifyingKey.q4),
            BN254.g1Serialize(verifyingKey.qM12),
            BN254.g1Serialize(verifyingKey.qM34),
            BN254.g1Serialize(verifyingKey.qH1)
        );
        self.transcript = abi.encodePacked(
            self.transcript,
            BN254.g1Serialize(verifyingKey.qH2),
            BN254.g1Serialize(verifyingKey.qH3),
            BN254.g1Serialize(verifyingKey.qH4),
            BN254.g1Serialize(verifyingKey.qO),
            BN254.g1Serialize(verifyingKey.qC),
            BN254.g1Serialize(verifyingKey.qEcc)
        );

        // sigmas
        self.transcript = abi.encodePacked(
            self.transcript,
            BN254.g1Serialize(verifyingKey.sigma0),
            BN254.g1Serialize(verifyingKey.sigma1),
            BN254.g1Serialize(verifyingKey.sigma2),
            BN254.g1Serialize(verifyingKey.sigma3),
            BN254.g1Serialize(verifyingKey.sigma4)
        );

        // public inputs
        self.transcript = abi.encodePacked(
            self.transcript,
            Utils.reverseEndianness(publicInput[0]),
            Utils.reverseEndianness(publicInput[1]),
            Utils.reverseEndianness(publicInput[2]),
            Utils.reverseEndianness(publicInput[3]),
            Utils.reverseEndianness(publicInput[4]),
            Utils.reverseEndianness(publicInput[5]),
            Utils.reverseEndianness(publicInput[6]),
            Utils.reverseEndianness(publicInput[7])
        );
    }

    /// @dev Append the proof to the transcript. Only used for test purposes.
    function appendProofEvaluations(
        TranscriptData memory self,
        IPlonkVerifier.PlonkProof memory proof
    ) internal pure {
        self.transcript = abi.encodePacked(
            self.transcript, Utils.reverseEndianness(BN254.ScalarField.unwrap(proof.wireEval0))
        );

        self.transcript = abi.encodePacked(
            self.transcript, Utils.reverseEndianness(BN254.ScalarField.unwrap(proof.wireEval1))
        );

        self.transcript = abi.encodePacked(
            self.transcript, Utils.reverseEndianness(BN254.ScalarField.unwrap(proof.wireEval2))
        );

        self.transcript = abi.encodePacked(
            self.transcript, Utils.reverseEndianness(BN254.ScalarField.unwrap(proof.wireEval3))
        );

        self.transcript = abi.encodePacked(
            self.transcript, Utils.reverseEndianness(BN254.ScalarField.unwrap(proof.wireEval4))
        );

        self.transcript = abi.encodePacked(
            self.transcript, Utils.reverseEndianness(BN254.ScalarField.unwrap(proof.sigmaEval0))
        );

        self.transcript = abi.encodePacked(
            self.transcript, Utils.reverseEndianness(BN254.ScalarField.unwrap(proof.sigmaEval1))
        );

        self.transcript = abi.encodePacked(
            self.transcript, Utils.reverseEndianness(BN254.ScalarField.unwrap(proof.sigmaEval2))
        );

        self.transcript = abi.encodePacked(
            self.transcript, Utils.reverseEndianness(BN254.ScalarField.unwrap(proof.sigmaEval3))
        );

        self.transcript = abi.encodePacked(
            self.transcript,
            Utils.reverseEndianness(BN254.ScalarField.unwrap(proof.prodPermZetaOmegaEval))
        );
    }
}
