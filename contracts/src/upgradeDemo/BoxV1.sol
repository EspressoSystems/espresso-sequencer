// SPDX-License-Identifier: Unlicensed

pragma solidity ^0.8.0;

import { OwnableUpgradeable } from
    "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import { Initializable } from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import { UUPSUpgradeable } from
    "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

/// @title An arbitrary Box contract that is only used for upgrade Sanity Checks
/// @notice The Box can be used by an account and they can deposit into their box
contract BoxV1 is Initializable, OwnableUpgradeable, UUPSUpgradeable {
    /// @notice Box status which might be upgraded
    enum BoxStatus {
        EMPTY,
        FULL
    }

    /// @notice Box struct which might be upgraded
    struct Box {
        uint256 size;
        BoxStatus status;
        uint256 balance;
    }

    /// @notice Mapping between owner and box, the data types here will not be upgraded for now
    /// @dev If we want to test data migrations then this should be upgraded
    mapping(address boxOwner => Box box) public boxes;

    /// @notice upgrade event when the proxy updates the implementation it's pointing to
    event Upgrade(address implementation);

    /// @notice since the constuctor initializes storage on this contract we disable it
    /// @dev storage is on the proxy contract since it calls this contract via delegatecall
    constructor() {
        _disableInitializers();
    }

    /// @notice This contract is called by the proxy when you deploy this contract
    function initialize() public initializer {
        __Ownable_init(msg.sender); //sets owner to msg.sender
        __UUPSUpgradeable_init();
    }

    /// @notice add a box for the user if it does not exist
    function addBox(uint256 _size) public {
        require(boxes[msg.sender].size == 0, "box already created for this user");

        require(_size > 0, "box size must be > 0");
        boxes[msg.sender] = Box({ size: _size, status: BoxStatus.EMPTY, balance: 0 });
    }

    /// @notice update a box for the user if it exists
    function updateBox(uint256 _newSize) public {
        require(_newSize > 0, "box size must be > 0");
        require(boxes[msg.sender].size != 0, "No box exists for this user");
        Box memory thisBox = boxes[msg.sender];
        thisBox.size = _newSize;
        boxes[msg.sender] = thisBox;
    }

    /// @notice update a box status for the user if it exists
    function updateBoxStatus(BoxStatus _status) public {
        require(boxes[msg.sender].size != 0, "No box exists for this user");
        boxes[msg.sender].status = _status;
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

    /// @notice only the owner can authorize an upgrade
    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {
        emit Upgrade(newImplementation);
    }
}
