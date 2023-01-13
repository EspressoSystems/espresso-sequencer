# Contract bindings
To generate the bindings run
```sh
cargo run --bin gen-bindings
```

Note that the contract `ProofOfEfficiency` was renamed to `PolygonZkEVM` in the
`zkevm-contracts` repo but that rename has not made its way into the
`zkevm-node`. We may want to follow the rename, if they do.

- Upstream source: https://github.com/0xPolygonHermez/zkevm-contracts/tree/main/contracts
- ABIs: https://github.com/0xPolygonHermez/zkevm-node/tree/develop/etherman/smartcontracts/abi
- Compiled contracts: https://github.com/0xPolygonHermez/zkevm-node/tree/develop/etherman/smartcontracts/bin
