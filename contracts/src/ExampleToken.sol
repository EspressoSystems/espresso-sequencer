// contracts/GLDToken.sol
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import { ERC20 } from "solmate/tokens/ERC20.sol";

contract ExampleToken is ERC20 {
    constructor(uint256 initialSupply) ERC20("Example", "EX", 18) {
        _mint(msg.sender, initialSupply);
    }
}
