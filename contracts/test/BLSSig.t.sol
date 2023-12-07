// SPDX-License-Identifier: Unlicensed

/* solhint-disable contract-name-camelcase, func-name-mixedcase, one-contract-per-file */

pragma solidity ^0.8.0;

// Libraries
import "forge-std/Test.sol";
import { BN254 } from "bn254/BN254.sol";

// Target contract
import { BLSSig } from "../src/libraries/BLSSig.sol";

contract BLSSig_Test is Test {
    function genBLSSig(bytes memory message)
        private
        returns (BN254.G2Point memory, BN254.G1Point memory)
    {
        // Generate a BLS verification key and signature  using rust code
        string[] memory cmds = new string[](3);
        cmds[0] = "diff-test";
        cmds[1] = "gen-bls-sig";
        cmds[2] = vm.toString(message);

        bytes memory result = vm.ffi(cmds);

        (BN254.G2Point memory vk, BN254.G1Point memory sig) =
            abi.decode(result, (BN254.G2Point, BN254.G1Point));
        return (vk, sig);
    }

    // TODO Philippe Tests low level functions

    function test_SigVerification_Succeeds() external {
        bytes memory message = "Hi";
        (BN254.G2Point memory vk, BN254.G1Point memory sig) = genBLSSig(message);
        BLSSig.verifyBlsSig(message, sig, vk);
    }

    // This is due to a current limitation/bug of foundry. See
    // https://github.com/foundry-rs/foundry/issues/4405
    function wrapVerifyBlsSig(
        bytes memory message,
        BN254.G1Point memory sig,
        BN254.G2Point memory vk
    ) public view {
        BLSSig.verifyBlsSig(message, sig, vk);
    }

    function test_RevertWhen_SignatureIsInvalid() external {
        bytes memory message = "Hi";

        BN254.G1Point memory badSig = BN254.P1();
        (BN254.G2Point memory vk,) = genBLSSig(message);
        vm.expectRevert(BLSSig.BLSSigVerificationFailed.selector);
        this.wrapVerifyBlsSig(message, badSig, vk);
    }

    function test_RevertWhen_usingWrongVK() external {
        bytes memory message = "Hi";
        (BN254.G2Point memory vk, BN254.G1Point memory sig) = genBLSSig(message);
        vk; // To avoid compiler warning
        BN254.G2Point memory badVK = BN254.P2();
        vm.expectRevert(BLSSig.BLSSigVerificationFailed.selector);
        this.wrapVerifyBlsSig(message, sig, badVK);
    }
}
