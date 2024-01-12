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
contract BoxV2 is Initializable, OwnableUpgradeable, UUPSUpgradeable {
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

    /// @notice Box struct with new member
    mapping(address boxOwner => Box box) boxes;

    /// @notice upgrade event when the proxy updates the implementation it's pointing to
    event Upgrade(address implementation);

    /// @notice since the constuctor initializes storage on this contract we disable it
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
        require(boxes[msg.sender].size == 0, "box already created for this user");
        require(_size > 0 && _maxItems > 0, "box size & capacity must be > 0");

        boxes[msg.sender] =
            Box({ size: _size, status: BoxStatus.EMPTY, balance: 0, maxItems: _maxItems });
    }

    /// @notice gets a box for the sender
    function getBox() public view returns (Box memory) {
        return boxes[msg.sender];
    }

    /// @notice allows sender to deposit into the contract and their balance is recorded in the box
    function deposit() public payable {
        require(msg.value > 0, "You must deposit some ETH");
        require(boxes[msg.sender].size != 0, "No box exists for this user");
        boxes[msg.sender].balance += msg.value;
    }

    /// @notice update a box for the user if it exists
    function updateBox(uint256 _newSize, uint256 _maxItems) public {
        require(_newSize > 0 && _maxItems > 0, "box size & capacity must be > 0");
        require(boxes[msg.sender].size != 0, "No box exists for this user");
        Box memory thisBox = boxes[msg.sender];
        thisBox.size = _newSize;
        thisBox.maxItems = _maxItems;
        boxes[msg.sender] = thisBox;
    }

    /// @notice update a box status for the user if it exists
    function updateBoxStatus(BoxStatus _status) public {
        require(boxes[msg.sender].size != 0, "No box exists for this user");
        boxes[msg.sender].status = _status;
    }

    /// @notice update a box capacity for the user if it exists
    function updateBoxCapacity(uint256 _maxItems) public {
        require(_maxItems > 0, "box size & capacity must be > 0");
        require(boxes[msg.sender].size != 0, "No box exists for this user");
        boxes[msg.sender].maxItems = _maxItems;
    }

    /// @notice withdraw ETH for user if they have a balance
    function withdrawETH() public payable {
        require(boxes[msg.sender].size != 0, "No box exists for this user");
        require(boxes[msg.sender].balance > 0, "No balance for this user");
        uint256 balanceToWithdraw = boxes[msg.sender].balance;
        boxes[msg.sender].balance = 0; //C-E-I pattern to prevent reentrancy

        (bool sent,) = msg.sender.call{ value: balanceToWithdraw }("");
        require(sent, "Failed to send Ether");
    }

    /// @notice only the owner can authorize an upgrade
    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {
        emit Upgrade(newImplementation);
    }
}
