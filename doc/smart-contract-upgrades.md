# Smart Contract Upgrades

Code on the EVM is immutable but via proxies, smart contracts can be “upgraded”. A proxy contract acts as an
intermediary that can delegate calls to other contracts, called implementation contracts. This allows the logic of a
smart contract to be upgraded while keeping the same address and state. This is made possible by two functions in
Solidity:

- `delegatecall`: contractA can delegatecall contractB but execute the logic within the context of contractA. This means
  that the called contract's code is executed with the storage, caller, and balance of the calling contract.
- `fallback`: executed when a function doesn't exist. This is often used in proxy contracts to catch all function calls
  and delegate them to the implementation contract.

OpenZeppelin's [implementation](https://docs.openzeppelin.com/contracts/4.x/api/proxy#UUPSUpgradeable) for the UUPS
proxy pattern was used.

## Writing Upgradable Smart Contracts

This is written with developers in mind to explain the process for upgrading contracts within our repo to ensure safe
upgrades.

### Upgrading via Contract Inheritance

Solidity supports inheritance which means that a function call always executes the function of the same name (and
parameter types) in the most derived contract in the inheritance hierarchy.

**Pros**:

- **Modularity**: each contract version can focus on specific features or upgrades
- **Reusability:** code doesn’t have to be duplicated in newer versions which reduces the likelihood of errors
- **Ease of extension:** easier to extend features without worrying about the rest of the implementation
- **C3 linearization**: C3 linearization provides a deterministic way to determine the storage layout of a contract
  based on its inheritance hierarchy. This means that if we maintain the same inheritance order in subsequent contract
  versions, the storage layout will remain consistent, even if we add new state variables.

**Cons**:

- **Deployment size:** Contracts that inherit from multiple base contracts can become large, potentially exceeding the
  maximum contract size limit on the Ethereum blockchain
- **State variable shadowing:** When a new version of a contract inherits from an older version, introducing a state
  variable with the same name as one in the base contract will lead to a compilation error due to shadowing. Therefore,
  one cannot simply redefine an existing struct with new members if inheritance is used for upgrades. This can become
  cumbersome in large projects with many inherited contracts
- **Multiple inheritance:** Solidity’s C3 linearization predicts the order in which base contracts are inherited, and
  their functions are called. So the order inherited is the order they will be in the storage layout and this needs to
  be taken under consideration. When multiple base contracts define state variables, the order of inheritance can affect
  the storage layout. Mismanaging this can lead to overlapping storage slots, causing data corruption.

### Upgrades supported

- new state variables
- new logic
- modifying existing logic (function overriding)

#### New State Variables

Thanks to C3 linearization, the order in which you inherit from the previous version of the contract dictates the
storage layout so be sure to be mindful when declaring new variables.

Criteria:

- New variable names must be used otherwise it leads to a compilation error due to shadowing.
- New variables added _must_ be declared as type `internal` or `public`, `private` can never be used.
- If the new variable needs to be initialized, then the initialize function with a separate initV[X] function should be
  added. The initV[X] function allows the contract to initialize only the new variables added in the new contract.

_Example:_

```solidity
// Version 1
contract V1 {
    uint public oldValue;
    uint8 internal _initializedVersion

    function initialize(uint _oldValue) external {
        require(_initializedVersion == 0);
        oldValue = _oldValue;
        _initializedVersion = 1;
    }
}

// Version 2
contract V2 is V1 {
    uint public newValue;

    function initializeV2(uint _newValue) external {
        require(_initializedVersion == 1);
        newValue = _newValue;
        _initializedVersion = 2;
    }
}
```

#### New Logic

Base functions can be overridden by inheriting contracts to change their behavior if they are marked as virtual. Thus if
you plan to keep a function upgradable, always mark it as virtual in new versions of the contract.

_Example:_

```solidity
// Version 1
contract V1 {
    function getValue() public view virtual returns (uint) {
        return 1;
    }
}

// Version 2
contract V2 is V1 {
    function getValue() public view virtual override returns (uint) {
        return 2;
    }
}
```

#### Modifying Existing Logic (Function Overriding)

The logic in exiting functions can be overridden but be sure not to introduce breaking changes.

Criteria:

- a function must be `virtual` to be able to be overridden in a derived contract.
- a function with `private` visibility cannot be overridden or inherited.
- Public state variables can override `external` functions if the parameter and return types of the function matches the
  getter function of the variable:

_Example:_

```solidity
// SPDX-License-Identifier: GPL-3.0
pragma solidity >=0.6.0 <0.9.0;

contract A {
    function f() external view virtual returns(uint) { return 5; }
}

contract B is A {
    uint public override f;
}
```

- The overriding function may only change the visibility of the overridden function from `external` to `public`.
- The mutability may be changed to a more strict one following the order: `nonpayable` can be overridden by `view` and
  `pure`. `view` can be overridden by `pure`. `payable` is an exception and cannot be changed to any other mutability.

### Upgrades that become complicated through inheritance

#### Upgrading Structs

Unfortunately, directly upgrading structs through inheritance is not straightforward due to Solidity's storage layout
rules. When a new version of a contract inherits from an older version, introducing a state variable with the same name
as one in the base contract will lead to a compilation error due to shadowing.

**Potential workarounds:**

- Additional struct: create a new struct with additional fields that extends the original struct so that both can be
  used in tandem.
- New Struct: Create a new struct with additional fields and migrate existing data to the new struct during the upgrade
  process. _Example:_

```solidity

// Version 1
contract V1 {
    struct Data {
        uint value;
    }

    Data public data;
}

// Version 2
contract V2 is V1 {
    struct NewData {
        uint value;
        uint newValue;
    }

    NewData public newData;

    function migrateData() public {
        newData = NewData(data.value, 0);
    }
}
```

## Deployment of Upgrades via the Proxy

- Deploy the new (modified) implementation contract If new state variables that need initialization were added then:
- Prepare an `upgradeToAndCall` transaction from the multisig admin account which is parameterized with the address of
  the new implementation contract and an internal call to invoke initializeV[X] with the new state variables data. Else:
- Prepare an `upgradeTo` transaction from the multisig admin account parameterized with the address of the new
  implementation contract: Then:
- Broadcast the transaction
