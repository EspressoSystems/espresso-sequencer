// SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.8.0;
// import { console } from "forge-std/console.sol";

contract FeeContract {
    // === Events ===
    //
    /// @notice Notify a new deposit
    event Deposit(address, uint256);
    event Log(string func, uint256 gas);
    // === Constants ===
    //
    /// @notice max amount allowed to be deposited to prevent fat finger errors
    // @TODO confirm this amount with product

    uint256 public immutable MAX_DEPOSIT_AMOUNT = 1 ether;

    // === Errors ===
    //
    /// @notice error types
    error InvalidUserAddress();
    error NonZeroDepositAmount();
    error NotEnoughDeposited();
    error DepositTooLarge();
    error FunctionDoesNotExist();
    error NoFunctionCalled();

    /// @notice store user balances in a mapping
    mapping(address user => uint256 amount) public balances;

    /// @notice Revert if a method name does not exist
    fallback() external payable {
        // send / transfer (forwards 2300 gas to this fallback function)
        // call (forwards all of the gas)
        revert FunctionDoesNotExist();
    }

    /// @notice Revert if no method name was called
    receive() external payable {
        revert NoFunctionCalled();
    }

    /// @notice Allows anyone to deposit an ETH balance for any user
    /// @dev the deposit amount is less than a specified threshold to prevent fat finger errors
    function deposit(address user) public payable {
        if (msg.value == 0) {
            revert NonZeroDepositAmount();
        }
        if (msg.value > MAX_DEPOSIT_AMOUNT) {
            revert DepositTooLarge();
        }
        if (user == address(0)) {
            revert InvalidUserAddress();
        }
        balances[user] += msg.value;
        emit Deposit(user, msg.value);
    }

    /// @notice Allows anyone to get the balance an ETH balance for any user
    function getBalance(address user) public view returns (uint256 amount) {
        if (user == address(0)) {
            revert InvalidUserAddress();
        }
        return balances[user];
    }
}
