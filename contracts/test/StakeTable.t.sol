// SPDX-License-Identifier: Unlicensed

/* solhint-disable contract-name-camelcase, func-name-mixedcase, one-contract-per-file */

pragma solidity ^0.8.0;

// Libraries
import "forge-std/Test.sol";
import { BN254 } from "bn254/BN254.sol";
import { BLSSig } from "../src/libraries/BLSSig.sol";
import { EdOnBN254 } from "../src/libraries/EdOnBn254.sol";
import "../src/interfaces/IStakeTable.sol";

// Target contract
import { StakeTable as S } from "../src/StakeTable.sol";

contract StableTable_keyRegister_Test is Test {
    S public stakeTable;

    function setUp() public {
        stakeTable = new S();
    }

    // TODO move to some utils library
    // https://ethereum.stackexchange.com/a/126928
    function iToHex(bytes memory buffer) public pure returns (string memory) {
        // Fixed buffer size for hexadecimal convertion
        bytes memory converted = new bytes(buffer.length * 2);

        bytes memory _base = "0123456789abcdef";

        for (uint256 i = 0; i < buffer.length; i++) {
            converted[i * 2] = _base[uint8(buffer[i]) / _base.length];
            converted[i * 2 + 1] = _base[uint8(buffer[i]) % _base.length];
        }

        return string(abi.encodePacked("0x", converted));
    }

    /// @dev Tests the key registering process
    function testRegister() external {
        // Generate a BLS signature in rust and read it in solidity
        string[] memory cmds = new string[](3);
        cmds[0] = "diff-test";
        cmds[1] = "gen-bls-sig";
        cmds[2] = iToHex(abi.encode(msg.sender));

        bytes memory result = vm.ffi(cmds);
        (
            uint256 blsSigX,
            uint256 blsSigY,
            uint256 blsVkx0,
            uint256 blsVkx1,
            uint256 blsVky0,
            uint256 blsVky1,
            uint256 schnorrVKx,
            uint256 schnorrVKy,
            address msgSenderAddress
        ) = abi.decode(
            result,
            (uint256, uint256, uint256, uint256, uint256, uint256, uint256, uint256, address)
        );

        // Note: (x,y) coordinates for each field component must be inverted.
        BN254.G2Point memory blsVk = BN254.G2Point(blsVkx1, blsVkx0, blsVky1, blsVky0);
        BN254.G1Point memory sig = BN254.G1Point(blsSigX, blsSigY);
        EdOnBN254.EdOnBN254Point memory schnorrVK = EdOnBN254.EdOnBN254Point(schnorrVKx, schnorrVKy);

        assertEq(msg.sender, msgSenderAddress);
        vm.prank(msgSenderAddress);

        // Failed signature verification
        BN254.G1Point memory badSig = BN254.P1();
        vm.expectRevert(BLSSig.BLSSigVerificationFailed.selector);
        stakeTable.register(blsVk, schnorrVK, 10, IStakeTable.StakeType.Native, badSig, 5);

        // Invalid next registration epoch
        vm.prank(msgSenderAddress);
        vm.expectRevert(bytes("Invalid next registration epoch."));
        stakeTable.register(blsVk, schnorrVK, 10, IStakeTable.StakeType.Native, sig, 0);

        // Happy path
        vm.prank(msgSenderAddress);
        stakeTable.register(blsVk, schnorrVK, 10, IStakeTable.StakeType.Native, sig, 5);

        // Check the BEANS tokens have been transfered

        // Check event is emitted

        // The node is already registered
        vm.prank(msgSenderAddress);
        vm.expectRevert(bytes("The node has already been registered"));
        stakeTable.register(blsVk, schnorrVK, 10, IStakeTable.StakeType.Native, sig, 5);
    }
}
