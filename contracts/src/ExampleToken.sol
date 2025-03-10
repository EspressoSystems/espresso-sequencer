// contracts/GLDToken.sol
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

// TODO MA: should we really use solmate, or just stick with openzeppelin?
import { ERC20 } from "solmate/tokens/ERC20.sol";

contract ExampleToken is ERC20 {
    constructor(uint256 initialSupply) ERC20("Example", "EX", 18) {
        _mint(msg.sender, initialSupply);
    }
}
