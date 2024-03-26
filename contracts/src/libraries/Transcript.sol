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
        self.transcript =
            abi.encodePacked(Utils.reverseEndianness(BN254.ScalarField.unwrap(fieldElement)));
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

    function appendGroupElement(TranscriptData memory self, BN254.G1Point memory comm)
        internal
        pure
    {
        self.transcript = abi.encodePacked(self.transcript, BN254.g1Serialize(comm));
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

    /// @dev Append the verifying key.
    /// @param verifyingKey verifying key
    // solhint-disable-next-line
    function appendVk(TranscriptData memory self, IPlonkVerifier.VerifyingKey memory verifyingKey)
        internal
        pure
    {
        // Fr field size in bits
        appendMessage(
            self,
            BytesLib.slice(abi.encodePacked(Utils.reverseEndianness(254)), 0, 4) //uint32
        );
        //sizeInBits
        // = 254;

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
    }
    /// @dev Append the public input to the transcript.
    /// @param publicInput a list of field elements

    function appendPubInput(TranscriptData memory self, uint256[] memory publicInput)
        internal
        pure
    {
        // This *if* statement is to avoid having many tests failures
        if (publicInput.length == 8) {
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
        } else {
            for (uint256 i = 0; i < publicInput.length; i++) {
                appendFieldElement(self, BN254.ScalarField.wrap(publicInput[i]));
            }
        }
    }

    /// @dev Append the proof to the transcript.
    function appendProofEvaluations(
        TranscriptData memory self,
        IPlonkVerifier.PlonkProof memory proof
    ) internal pure {
        append5FieldElements(
            self,
            proof.wireEval0,
            proof.wireEval1,
            proof.wireEval2,
            proof.wireEval3,
            proof.wireEval4
        );

        append5FieldElements(
            self,
            proof.sigmaEval0,
            proof.sigmaEval1,
            proof.sigmaEval2,
            proof.sigmaEval3,
            proof.prodPermZetaOmegaEval
        );
    }
}
