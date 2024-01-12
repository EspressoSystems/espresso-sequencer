# Purpose

This subfolder was created to test various upgrade scenarios when using the UUPS proxy pattern.

## How to run the tests

`cd contracts` `forge test --match-contract Box -vvv --summary`

## Tests

The tests check for the following post upgrade:

- struct modification (addition of new member)
- enum modification (addition of new member)
- ETH deposit maintained in upgraded version
- introduction of a withdrawal function post first deployment works
