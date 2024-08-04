// SPDX-License-Identifier: Unlicensed

pragma solidity ^0.8.0;

import { BN254 } from "bn254/BN254.sol";
import { IPlonkVerifier } from "../interfaces/IPlonkVerifier.sol";

library Transcript {
    struct TranscriptData {
        bytes transcript;
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
        self.transcript = abi.encodePacked(self.transcript, challenge);
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
        self.transcript = abi.encodePacked(self.transcript, comm.x, comm.y);
    }

    function getAndAppendChallenge(TranscriptData memory self)
        internal
        pure
        returns (uint256 ret)
    {
        bytes memory transcript = self.transcript;
        uint256 p = BN254.R_MOD;

        assembly {
            let len := mload(transcript)
            let newLen := add(len, 32)
            let dataPtr := add(transcript, 0x20)

            // uint256(keccak256(transcript)) % BN254.R_MOD;
            ret := mod(keccak256(dataPtr, len), p)

            // same as self.transcript = abi.encodePacked(self.transcript, ret);
            mstore(transcript, newLen)
            mstore(add(dataPtr, len), ret)

            // update free memory pointer since we extend the dynamic array
            // to prevent potential overwrite
            mstore(0x40, add(mload(0x40), 32))
        }
    }

    /// @dev Append the verifying key and the public inputs to the transcript.
    /// @param verifyingKey verifying key
    /// @param publicInput a list of field elements
    function appendVkAndPubInput(
        TranscriptData memory self,
        IPlonkVerifier.VerifyingKey memory verifyingKey,
        uint256[8] memory publicInput
    ) internal pure {
        uint32 sizeInBits = 254;

        self.transcript = abi.encodePacked(
            self.transcript,
            uint32(sizeInBits),
            uint64(verifyingKey.domainSize),
            uint64(verifyingKey.numInputs)
        );

        // =====================
        // k: coset representatives
        // =====================
        // Currently, K is hardcoded, and there are 5 of them since
        // # wire types == 5

        self.transcript = abi.encodePacked(
            self.transcript,
            uint256(0x1), // k0 = 1
            uint256(0x2f8dd1f1a7583c42c4e12a44e110404c73ca6c94813f85835da4fb7bb1301d4a), // k1
            uint256(0x1ee678a0470a75a6eaa8fe837060498ba828a3703b311d0f77f010424afeb025), // k2
            uint256(0x2042a587a90c187b0a087c03e29c968b950b1db26d5c82d666905a6895790c0a), // k3
            uint256(0x2e2b91456103698adf57b799969dea1c8f739da5d8d40dd3eb9222db7c81e881) // k4
        );

        // selectors
        self.transcript = abi.encodePacked(
            self.transcript,
            verifyingKey.q1.x,
            verifyingKey.q1.y,
            verifyingKey.q2.x,
            verifyingKey.q2.y,
            verifyingKey.q3.x,
            verifyingKey.q3.y,
            verifyingKey.q4.x,
            verifyingKey.q4.y
        );
        self.transcript = abi.encodePacked(
            self.transcript,
            verifyingKey.qM12.x,
            verifyingKey.qM12.y,
            verifyingKey.qM34.x,
            verifyingKey.qM34.y
        );
        self.transcript = abi.encodePacked(
            self.transcript,
            verifyingKey.qH1.x,
            verifyingKey.qH1.y,
            verifyingKey.qH2.x,
            verifyingKey.qH2.y,
            verifyingKey.qH3.x,
            verifyingKey.qH3.y,
            verifyingKey.qH4.x,
            verifyingKey.qH4.y
        );
        self.transcript = abi.encodePacked(
            self.transcript,
            verifyingKey.qO.x,
            verifyingKey.qO.y,
            verifyingKey.qC.x,
            verifyingKey.qC.y,
            verifyingKey.qEcc.x,
            verifyingKey.qEcc.y
        );

        // sigmas
        self.transcript = abi.encodePacked(
            self.transcript,
            verifyingKey.sigma0.x,
            verifyingKey.sigma0.y,
            verifyingKey.sigma1.x,
            verifyingKey.sigma1.y,
            verifyingKey.sigma2.x,
            verifyingKey.sigma2.y
        );
        self.transcript = abi.encodePacked(
            self.transcript,
            verifyingKey.sigma3.x,
            verifyingKey.sigma3.y,
            verifyingKey.sigma4.x,
            verifyingKey.sigma4.y
        );

        // public inputs
        self.transcript = abi.encodePacked(
            self.transcript,
            publicInput[0],
            publicInput[1],
            publicInput[2],
            publicInput[3],
            publicInput[4],
            publicInput[5],
            publicInput[6],
            publicInput[7]
        );
    }

    /// @dev Append the proof to the transcript. Only used for test purposes.
    function appendProofEvaluations(
        TranscriptData memory self,
        IPlonkVerifier.PlonkProof memory proof
    ) internal pure {
        self.transcript = abi.encodePacked(self.transcript, proof.wireEval0);

        self.transcript = abi.encodePacked(self.transcript, proof.wireEval1);

        self.transcript = abi.encodePacked(self.transcript, proof.wireEval2);

        self.transcript = abi.encodePacked(self.transcript, proof.wireEval3);

        self.transcript = abi.encodePacked(self.transcript, proof.wireEval4);

        self.transcript = abi.encodePacked(self.transcript, proof.sigmaEval0);

        self.transcript = abi.encodePacked(self.transcript, proof.sigmaEval1);

        self.transcript = abi.encodePacked(self.transcript, proof.sigmaEval2);

        self.transcript = abi.encodePacked(self.transcript, proof.sigmaEval3);

        self.transcript = abi.encodePacked(self.transcript, proof.prodPermZetaOmegaEval);
    }
}
