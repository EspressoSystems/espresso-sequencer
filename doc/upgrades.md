
# Upgrades

Hotshot protocol supports upgrades through an Upgrade proposal mechanism. The Upgrade proposal is broadcast separately from the `QuorumProposal`, typically several views in advance of its attachment. The goal is to ensure ample time for nodes to receive and prepare for the upgrade process.

After enough votes have been collected on the `UpgradeProposal`, an `UpgradeCertificate` is formed. This is attached to the next `QuorumProposal`, and any node that receives an `UpgradeCertificate` in this way re-attaches it to its own `QuorumProposal` until the network has upgraded, or (in rare cases) we failed to reach consensus on the `UpgradeCertificate`.

## Enabling an Upgrade

To enable an upgrade in Hotshot protocol, it is essential to define the base version, the upgrade version, and an upgrade hash:

- **Base Version:** Represents the current version of the protocol (`0.1` in this example).
- **Upgrade Version:** Specifies the version to which the protocol will upgrade once the process is successful (`0.2` in this example).
- **Upgrade Hash:** Acts as a unique identifier for the specific upgrade nodes are voting on. It distinguishes between different proposals of the same version upgrade, ensuring nodes vote and execute the correct one. It consists of a sequence of 32 bytes.

These are defined in [NodeType implementation](../types/src/v0/mod.rs) for the Types (`SeqTypes` in our case).
```rust
impl NodeType for SeqTypes {
    type Base = StaticVersion<0, 1>;
    type Upgrade = StaticVersion<0, 2>;
    const UPGRADE_HASH: [u8; 32] = [
        1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
    ], 
    ..
}
```

These parameters are fetched from the genesis TOML file and set in Hotshot config:

- **start_voting_view:**  view at which voting for the upgrade proposal starts. In our implementation, this is set to 1 so that voting is enabled as soon as the node is started.
- **stop_voting_view:**  view at which voting for the upgrade proposal stops. To disable an upgrade, set this parameter to 0 or ensure `stop_voting_view` is less than `start_voting_view`.
- **start_proposing_view:** the earliest view in which the node can propose an upgrade. This should be set to when an upgrade is intended. If the current view > `start_proposing_view`, the node will send out an `UpgradeProposal`.
- **stop_proposing_view:** view after which the node stops proposing an upgrade. If the upgrade proposal fails and the current view > stop_proposing_view then the upgrade is never proposed again.

The window between `start_proposing_view` and `stop_proposing_view` should provide sufficient time for nodes to continue proposing the upgrade until successful.

Ensure that the `ESPRESSO_SEQUENCER_GENESIS_FILE` environment variable is defined to point to the path of the genesis TOML file. For an example with upgrades enabled, refer to [`data/genesis/demo.toml`](../data/genesis/demo.toml).

Note: We currently support only chain config upgrade.
### Example TOML Configuration

```toml
[[upgrade]]
version = "0.2"
view = 5
propose_window = 10

[upgrade.chain_config]
chain_id = 999999999
base_fee = '2 wei'
max_block_size = '1mb'
fee_recipient = '0x0000000000000000000000000000000000000000'
fee_contract = '0xa15bb66138824a1c7167f5e85b957d04dd34e468'
```
In the TOML configuration example above, the `upgrade` section defines an array of tables, each specifying upgrade parameters:

- **Version:** the new version after an upgrade is successful.
- **View:** Represents the `start_proposing_view` value at which the upgrade is proposed.
- **Propose Window:** Refers to the view window between `start_proposing_view` and `stop_proposing_view`.

The `upgrade.chain_config` table contains the complete set of chain config parameters, which can be used, for example, to enable protocol fees or modify other parameters.


## Fee upgrade

A successful Hotshot upgrade results in a new version, which allows us to update the `ChainConfig` and execute the upgrade if there exists any. `Chainconfig` includes the fee parameters. The sequencer node has two states: `NodeState` and `ValidatedState`. `NodeState` is an immutable state that contains `ResolvableChainConfig` (Enum of `ChainConfig`'s commitment and full `ChainConfig`), whereas `ValidatedState` is a mutable state. To make updates to the chain config post-upgrade possible, `ResolvableChainConfig` is also added to `ValidatedState`.

`NodeState` also includes two additional fields: `upgrades` and `current_version`. Functions like `Header::new()` and `ValidatedState::apply_header()` include a version parameter, which is used to apply upgrades by checking if this version is greater than  `current_version` in NodeState and fetching the upgrade, if available, from the upgrades BTreeMap in NodeState.

In scenarios where nodes join the network or restart, missing the upgrade window may result in their ValidatedState having only a chain config commitment. In such cases, nodes need to catch up from their peers to get the  full chain config for this chain config commitment.

Note: For the fee upgrade to work, the builder must have sufficient funds to cover the fees. The Espresso bridge can be used to fund the builder.
