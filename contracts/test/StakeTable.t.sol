// SPDX-License-Identifier: Unlicensed

/* solhint-disable contract-name-camelcase, func-name-mixedcase, one-contract-per-file */

pragma solidity ^0.8.0;

// Libraries
import "forge-std/Test.sol";
import { BN254 } from "bn254/BN254.sol";
import { BLSSig } from "../src/libraries/BLSSig.sol";

// Target contract
import { StakeTable as S } from "../src/StakeTable.sol";

contract StableTable_keyRegister_Test is Test {
    S public stakeTable;

    function setUp() public {
        stakeTable = new S();
    }

    /// @dev Tests the key registering process
    function testKeyRegister() external {
        // Generate a BLS signature in rust and read it in solidity
        string[] memory cmds = new string[](2);
        cmds[0] = "diff-test";
        cmds[1] = "gen-bls-sig";

        bytes memory result = vm.ffi(cmds);
        (
            uint256 blsSigX,
            uint256 blsSigY,
            uint256 blsVkx0,
            uint256 blsVkx1,
            uint256 blsVky0,
            uint256 blsVky1,
            bytes memory message
        ) = abi.decode(result, (uint256, uint256, uint256, uint256, uint256, uint256, bytes));

        BN254.G1Point memory sig = BN254.G1Point(blsSigX, blsSigY);

        // Note: (x,y) coordinates for each field component must be inverted.
        BN254.G2Point memory vk = BN254.G2Point(blsVkx1, blsVkx0, blsVky1, blsVky0);

        BLSSig.verifyBlsSig(message, sig, vk);
    }
}
