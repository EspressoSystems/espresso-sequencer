// SPDX-License-Identifier: Unlicensed

pragma solidity ^0.8.0;

import { OwnableUpgradeable } from
    "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import { Initializable } from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import { UUPSUpgradeable } from
    "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

/// @title An arbitrary Box contract that is only used for upgrade Sanity Checks
/// @notice This is an upgrade of BoxV1 to allow users to withdraw, adds another enum type & struct
/// member
contract DemoBoxV2 is Initializable, OwnableUpgradeable, UUPSUpgradeable {
    error BoxAlreadyExists();
    error BoxSizeTooSmall();
    error BoxCapacityTooSmall();
    error NoBoxExists();
    error YouMustDepositETH();
    error ZeroBalance();
    error FailedToSendEther();

    /// @notice Box status with new enum type
    enum BoxStatus {
        EMPTY,
        FULL,
        ALMOST_FULL
    }

    /// @notice Box struct with new member
    struct Box {
        uint256 size;
        BoxStatus status;
        uint256 balance;
        uint256 maxItems;
    }

    /// @notice A simple way to track contract versions
    uint32 public immutable version = 2;

    /// @notice Box struct with new member
    mapping(address boxOwner => Box box) public boxes;

    /// @notice upgrade event when the proxy updates the implementation it's pointing to
    event Upgrade(address implementation);

    /// @notice since the constructor initializes storage on this contract we disable it
    /// @dev storage is on the proxy contract since it calls this contract via delegatecall
    constructor() {
        _disableInitializers();
    }

    function initialize() public initializer {
        __Ownable_init(msg.sender); //sets owner to msg.sender
        __UUPSUpgradeable_init();
    }

    /// @notice add a box for the user if it does not exist
    /// @dev added support for updated struct and enum
    function addBox(uint256 _size, uint256 _maxItems) public {
        if (boxes[msg.sender].size != 0) {
            revert BoxAlreadyExists();
        }

        if (_size == 0) {
            revert BoxSizeTooSmall();
        }

        if (_maxItems == 0) {
            revert BoxCapacityTooSmall();
        }

        boxes[msg.sender] =
            Box({ size: _size, status: BoxStatus.EMPTY, balance: 0, maxItems: _maxItems });
    }

    /// @notice gets a box for the sender
    function getBox() public view returns (Box memory) {
        return boxes[msg.sender];
    }

    /// @notice allows sender to deposit into the contract and their balance is recorded in the box
    function deposit(address receiver) public payable {
        if (msg.value == 0) {
            revert YouMustDepositETH();
        }

        if (boxes[msg.sender].size == 0) {
            revert NoBoxExists();
        }

        boxes[receiver].balance += msg.value;
    }

    /// @notice allows sender to deposit into the contract and their balance is recorded in the box
    function deposit(address receiver, address receiver2) public payable {
        if (msg.value == 0) {
            revert YouMustDepositETH();
        }

        if (boxes[receiver].size == 0 && boxes[receiver2].size == 0) {
            revert NoBoxExists();
        }

        address recipient = receiver;
        if (receiver == address(0)) {
            recipient = receiver2;
        }
        boxes[recipient].balance += msg.value;
    }

    /// @notice update a box for the user if it exists
    function updateBox(uint256 _newSize, uint256 _maxItems) public {
        if (_newSize == 0) {
            revert BoxSizeTooSmall();
        }
        if (_maxItems == 0) {
            revert BoxCapacityTooSmall();
        }
        if (boxes[msg.sender].size == 0) {
            revert NoBoxExists();
        }
        Box memory thisBox = boxes[msg.sender];
        thisBox.size = _newSize;
        thisBox.maxItems = _maxItems;
        boxes[msg.sender] = thisBox;
    }

    /// @notice update a box status for the user if it exists
    function updateBoxStatus(BoxStatus _status) public {
        if (boxes[msg.sender].size == 0) {
            revert NoBoxExists();
        }
        boxes[msg.sender].status = _status;
    }

    /// @notice update a box capacity for the user if it exists
    function updateBoxCapacity(uint256 _maxItems) public {
        if (_maxItems == 0) {
            revert BoxCapacityTooSmall();
        }
        if (boxes[msg.sender].size == 0) {
            revert NoBoxExists();
        }
        boxes[msg.sender].maxItems = _maxItems;
    }

    /// @notice withdraw ETH for user if they have a balance
    function withdrawETH() public payable {
        if (boxes[msg.sender].size == 0) {
            revert NoBoxExists();
        }
        if (boxes[msg.sender].balance == 0) {
            revert ZeroBalance();
        }
        uint256 balanceToWithdraw = boxes[msg.sender].balance;
        boxes[msg.sender].balance = 0; //C-E-I pattern to prevent reentrancy

        (bool sent,) = msg.sender.call{ value: balanceToWithdraw }("");
        if (!sent) {
            revert FailedToSendEther();
        }
    }

    /// @notice only the owner can authorize an upgrade
    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {
        emit Upgrade(newImplementation);
    }
}
