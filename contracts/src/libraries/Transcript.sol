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

    function append5FieldElements(
        TranscriptData memory self,
        BN254.ScalarField f1,
        BN254.ScalarField f2,
        BN254.ScalarField f3,
        BN254.ScalarField f4,
        BN254.ScalarField f5
    ) internal pure {
        self.transcript = abi.encodePacked(
            self.transcript,
            Utils.reverseEndianness(BN254.ScalarField.unwrap(f1)),
            Utils.reverseEndianness(BN254.ScalarField.unwrap(f2)),
            Utils.reverseEndianness(BN254.ScalarField.unwrap(f3)),
            Utils.reverseEndianness(BN254.ScalarField.unwrap(f4)),
            Utils.reverseEndianness(BN254.ScalarField.unwrap(f5))
        );
    }

    function append8FieldElements(
        TranscriptData memory self,
        BN254.ScalarField f1,
        BN254.ScalarField f2,
        BN254.ScalarField f3,
        BN254.ScalarField f4,
        BN254.ScalarField f5,
        BN254.ScalarField f6,
        BN254.ScalarField f7,
        BN254.ScalarField f8
    ) internal pure {
        self.transcript = abi.encodePacked(
            self.transcript,
            Utils.reverseEndianness(BN254.ScalarField.unwrap(f1)),
            Utils.reverseEndianness(BN254.ScalarField.unwrap(f2)),
            Utils.reverseEndianness(BN254.ScalarField.unwrap(f3)),
            Utils.reverseEndianness(BN254.ScalarField.unwrap(f4)),
            Utils.reverseEndianness(BN254.ScalarField.unwrap(f5)),
            Utils.reverseEndianness(BN254.ScalarField.unwrap(f6)),
            Utils.reverseEndianness(BN254.ScalarField.unwrap(f7)),
            Utils.reverseEndianness(BN254.ScalarField.unwrap(f8))
        );
    }

    function append2GroupElements(
        TranscriptData memory self,
        BN254.G1Point memory p1,
        BN254.G1Point memory p2
    ) internal pure {
        self.transcript =
            abi.encodePacked(self.transcript, BN254.g1Serialize(p1), BN254.g1Serialize(p2));
    }

    function append3GroupElements(
        TranscriptData memory self,
        BN254.G1Point memory p1,
        BN254.G1Point memory p2,
        BN254.G1Point memory p3
    ) internal pure {
        self.transcript = abi.encodePacked(
            self.transcript, BN254.g1Serialize(p1), BN254.g1Serialize(p2), BN254.g1Serialize(p3)
        );
    }

    function append5GroupElements(
        TranscriptData memory self,
        BN254.G1Point memory p1,
        BN254.G1Point memory p2,
        BN254.G1Point memory p3,
        BN254.G1Point memory p4,
        BN254.G1Point memory p5
    ) internal pure {
        self.transcript = abi.encodePacked(
            self.transcript,
            BN254.g1Serialize(p1),
            BN254.g1Serialize(p2),
            BN254.g1Serialize(p3),
            BN254.g1Serialize(p4),
            BN254.g1Serialize(p5)
        );
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

    // @dev This function computes keccak256(bytes32 a, bytes b)
    function computeHash(bytes32 a, bytes memory b) public pure returns (bytes32 result) {
        assembly {
            // Load the length of 'b'
            let bLength := mload(b)

            // Calculate total data length (32 bytes for 'a' + length of 'b')
            let totalLength := add(32, bLength)

            // Allocate memory for 'a' + 'b'
            let data := mload(0x40) // Load free memory pointer

            // Store 'a' in memory
            mstore(data, a)

            // Copy 'b' to memory after 'a'
            let dataOffset := add(data, 32) // Start right after 'a'
            for { let i := 0 } lt(i, bLength) { i := add(i, 0x20) } {
                mstore(add(dataOffset, i), mload(add(add(b, i), 0x20)))
            }

            // Compute the keccak256 hash of the data
            result := keccak256(data, totalLength)
        }
    }

    function getAndAppendChallenge(TranscriptData memory self) internal pure returns (uint256) {
        bytes32 hash = computeHash(self.state, self.transcript);

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
        append5FieldElements(
            self,
            BN254.ScalarField.wrap(0x1), // k0 = 1
            BN254.ScalarField.wrap(
                0x2f8dd1f1a7583c42c4e12a44e110404c73ca6c94813f85835da4fb7bb1301d4a
            ), // k1
            BN254.ScalarField.wrap(
                0x1ee678a0470a75a6eaa8fe837060498ba828a3703b311d0f77f010424afeb025
            ), // k2
            BN254.ScalarField.wrap(
                0x2042a587a90c187b0a087c03e29c968b950b1db26d5c82d666905a6895790c0a
            ), // k3
            BN254.ScalarField.wrap(
                0x2e2b91456103698adf57b799969dea1c8f739da5d8d40dd3eb9222db7c81e881
            ) // k4
        );

        // selectors
        append5GroupElements(
            self,
            verifyingKey.q1,
            verifyingKey.q2,
            verifyingKey.q3,
            verifyingKey.q4,
            verifyingKey.qM12
        );
        append5GroupElements(
            self,
            verifyingKey.qM34,
            verifyingKey.qH1,
            verifyingKey.qH2,
            verifyingKey.qH3,
            verifyingKey.qH4
        );

        append3GroupElements(self, verifyingKey.qO, verifyingKey.qC, verifyingKey.qEcc);

        // sigmas
        append5GroupElements(
            self,
            verifyingKey.sigma0,
            verifyingKey.sigma1,
            verifyingKey.sigma2,
            verifyingKey.sigma3,
            verifyingKey.sigma4
        );

        // public inputs
        append8FieldElements(
            self,
            BN254.ScalarField.wrap(publicInput[0]),
            BN254.ScalarField.wrap(publicInput[1]),
            BN254.ScalarField.wrap(publicInput[2]),
            BN254.ScalarField.wrap(publicInput[3]),
            BN254.ScalarField.wrap(publicInput[4]),
            BN254.ScalarField.wrap(publicInput[5]),
            BN254.ScalarField.wrap(publicInput[6]),
            BN254.ScalarField.wrap(publicInput[7])
        );
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
