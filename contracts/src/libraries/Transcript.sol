// SPDX-License-Identifier: Unlicensed

pragma solidity ^0.8.0;

import { BN254, Utils } from "bn254/BN254.sol";
import { BytesLib } from "solidity-bytes-utils/BytesLib.sol";
import { IPlonkVerifier } from "../interfaces/IPlonkVerifier.sol";

library Transcript {
    struct TranscriptData {
        bytes transcript;
        bytes32[2] state;
    }

    // ================================
    // Primitive functions
    // ================================
    function appendMessage(TranscriptData memory self, bytes memory message) internal pure {
        self.transcript = abi.encodePacked(self.transcript, message);
    }

    function appendFieldElement(TranscriptData memory self, BN254.ScalarField fieldElement)
        internal
        pure
    {
        appendMessage(
            self, abi.encodePacked(Utils.reverseEndianness(BN254.ScalarField.unwrap(fieldElement)))
        );
    }

    function appendGroupElement(TranscriptData memory self, BN254.G1Point memory comm)
        internal
        pure
    {
        bytes memory commBytes = BN254.g1Serialize(comm);
        appendMessage(self, commBytes);
    }

    // ================================
    // Transcript APIs
    // ================================
    function appendChallenge(TranscriptData memory self, uint256 challenge) internal pure {
        appendFieldElement(self, BN254.ScalarField.wrap(challenge));
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
        appendGroupElement(self, comm);
    }

    function computeHash0(bytes32 a, bytes32 b, bytes memory c) private pure returns (bytes32) {
        bytes32 hash;
        assembly {
            // Allocate memory for the data to hash
            let data := mload(0x40) // Find the current free memory location
            mstore(data, a) // Store 'a' at the current free memory location
            mstore(add(data, 32), b) // Store 'b' immediately after 'a'

            // Calculate the offset for 'c' (dynamic bytes)
            // and store 'c' there. Since 'c' is dynamic, copy its data.
            let cData := add(data, 64) // 'c' starts after 'a' and 'b'
            let cLength := mload(c) // Load the length of 'c'

            // Copy the contents of 'c'
            for { let i := 0 } lt(i, cLength) { i := add(i, 32) } {
                mstore(add(cData, i), mload(add(c, add(i, 32))))
            }

            // Append uint8(0) at the end. Since it's a single byte, we don't need a full word.
            // Be aware of potential issues with zero padding in other contexts.
            let end := add(cData, cLength) // Calculate end of 'c' data
            mstore(end, 0)

            // Compute the hash
            // The total length is 64 bytes (for 'a' and 'b') + cLength + 1 (for uint8(0))
            hash := keccak256(data, add(add(64, cLength), 1))
        }
        return hash;
    }

    function computeHash1(bytes32 a, bytes32 b, bytes memory c) private pure returns (bytes32) {
        bytes32 hash;
        assembly {
            // Allocate memory for the data to hash
            let data := mload(0x40) // Current free memory location
            mstore(data, a) // Store 'a' at the current location
            mstore(add(data, 32), b) // Store 'b' immediately after 'a'

            // Prepare to store 'c' after 'a' and 'b'
            let cData := add(data, 64) // Location after 'a' and 'b'
            let cLength := mload(c) // Length of 'c'

            // Copy the bytes of 'c' into memory
            for { let i := 0 } lt(i, cLength) { i := add(i, 32) } {
                mstore(add(cData, i), mload(add(c, add(i, 32))))
            }

            // Append uint8(1) at the end
            let end := add(cData, cLength) // Calculate end of 'c' data
            mstore8(end, 1)

            // Compute the keccak256 hash of the data
            // The data length is 64 bytes ('a' and 'b') + cLength + 1 byte for the uint8
            hash := keccak256(data, add(add(64, cLength), 1))
        }
        return hash;
    }

    function getAndAppendChallenge(TranscriptData memory self) internal pure returns (uint256) {
        bytes32 h1 = computeHash0(self.state[0], self.state[1], self.transcript);
        bytes32 h2 = computeHash1(self.state[0], self.state[1], self.transcript);

        self.state[0] = h1;
        self.state[1] = h2;

        return BN254.fromLeBytesModOrder48(BytesLib.slice(abi.encodePacked(h1, h2), 0, 48));
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

        // Fr field size in bits
        appendMessage(
            self, BytesLib.slice(abi.encodePacked(Utils.reverseEndianness(sizeInBits)), 0, 4)
        );

        // domain size
        appendMessage(
            self,
            BytesLib.slice(abi.encodePacked(Utils.reverseEndianness(verifyingKey.domainSize)), 0, 8)
        );

        // number of inputs
        appendMessage(
            self,
            BytesLib.slice(abi.encodePacked(Utils.reverseEndianness(verifyingKey.numInputs)), 0, 8)
        );

        // =====================
        // k: coset representatives
        // =====================
        // Currently, K is hardcoded, and there are 5 of them since
        // # wire types == 5
        appendFieldElement(self, BN254.ScalarField.wrap(0x1)); // k0 = 1
        appendFieldElement(
            self,
            BN254.ScalarField.wrap(
                0x2f8dd1f1a7583c42c4e12a44e110404c73ca6c94813f85835da4fb7bb1301d4a
            )
        ); // k1
        appendFieldElement(
            self,
            BN254.ScalarField.wrap(
                0x1ee678a0470a75a6eaa8fe837060498ba828a3703b311d0f77f010424afeb025
            )
        ); // k2
        appendFieldElement(
            self,
            BN254.ScalarField.wrap(
                0x2042a587a90c187b0a087c03e29c968b950b1db26d5c82d666905a6895790c0a
            )
        ); // k3
        appendFieldElement(
            self,
            BN254.ScalarField.wrap(
                0x2e2b91456103698adf57b799969dea1c8f739da5d8d40dd3eb9222db7c81e881
            )
        ); // k4

        // selectors
        appendGroupElement(self, verifyingKey.q1);
        appendGroupElement(self, verifyingKey.q2);
        appendGroupElement(self, verifyingKey.q3);
        appendGroupElement(self, verifyingKey.q4);
        appendGroupElement(self, verifyingKey.qM12);
        appendGroupElement(self, verifyingKey.qM34);
        appendGroupElement(self, verifyingKey.qH1);
        appendGroupElement(self, verifyingKey.qH2);
        appendGroupElement(self, verifyingKey.qH3);
        appendGroupElement(self, verifyingKey.qH4);
        appendGroupElement(self, verifyingKey.qO);
        appendGroupElement(self, verifyingKey.qC);
        appendGroupElement(self, verifyingKey.qEcc);

        // sigmas
        appendGroupElement(self, verifyingKey.sigma0);
        appendGroupElement(self, verifyingKey.sigma1);
        appendGroupElement(self, verifyingKey.sigma2);
        appendGroupElement(self, verifyingKey.sigma3);
        appendGroupElement(self, verifyingKey.sigma4);

        // public inputs
        for (uint256 i = 0; i < publicInput.length; i++) {
            appendFieldElement(self, BN254.ScalarField.wrap(publicInput[i]));
        }
    }

    /// @dev Append the proof to the transcript.
    function appendProofEvaluations(
        TranscriptData memory self,
        IPlonkVerifier.PlonkProof memory proof
    ) internal pure {
        appendFieldElement(self, proof.wireEval0);
        appendFieldElement(self, proof.wireEval1);
        appendFieldElement(self, proof.wireEval2);
        appendFieldElement(self, proof.wireEval3);
        appendFieldElement(self, proof.wireEval4);

        appendFieldElement(self, proof.sigmaEval0);
        appendFieldElement(self, proof.sigmaEval1);
        appendFieldElement(self, proof.sigmaEval2);
        appendFieldElement(self, proof.sigmaEval3);

        appendFieldElement(self, proof.prodPermZetaOmegaEval);
    }
}
