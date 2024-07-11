
# Upgrades

Hotshot protocol supports upgrades through an Upgrade proposal mechanism. The Upgrade proposal is broadcast separately from the `QuorumProposal`, typically several views in advance of its attachment. The goal is to ensure ample time for nodes to receive and prepare for the upgrade process.

Voting for the `UpgradeProposal` begins before its proposal. Sufficient votes are gathered to form an upgrade certificate. Once obtained, the proposal is broadcasted, and any node that receives it accepts and attaches it to its own `QuorumProposal`.

## Enabling an Upgrade

To enable an upgrade in Hotshot protocol:

When preparing for an upgrade, it's essential to define the base version, the upgrade version, and a unique upgrade hash:

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

- **start_voting_view:**  view at which voting for the upgrade proposal starts. In our implementation, this is set to 1 so that voting begins as soon as the node is bootup.
- **stop_voting_view:**  view at which voting for the upgrade proposal stops. To disable an upgrade, set this parameter to 0 or ensure `stop_voting_view` is less than `start_voting_view`.
- **start_proposing_view:**  view at which the node proposes an upgrade. This should be set to when an upgrade is intended. If the current view > `start_proposing_view`, the node proposes as soon as `UpgradeCertificate` is formed.
- **stop_proposing_view:** The view after which the upgrade proposal is no longer valid. If the upgrade proposal fails and the current view > stop_proposing_view then the upgrade is never proposed again.

The window between `start_proposing_view` and `stop_proposing_view` should provide sufficient time for nodes to continue proposing the upgrade until successful.

Ensure that the `ESPRESSO_SEQUENCER_GENESIS_FILE` environment variable is defined to point to the path of the genesis TOML file. For an example with upgrades enabled, refer to [`data/genesis/demo.toml`](../data/genesis/demo.toml).

### Example TOML Configuration

```toml
[[upgrade]]
version = "0.1"
view = 5
propose_window = 10

[upgrade.chain_config]
chain_id = 999999999
base_fee = '2 wei'
max_block_size = '1mb'
fee_recipient = '0x0000000000000000000000000000000000000000'
fee_contract = '0xa15bb66138824a1c7167f5e85b957d04dd34e468'
```
In the TOML configuration example above, the `upgrade` section defines an array of tables, each specifying upgrade parameters such as version, view, and propose window. 

- **Version:** Indicates the current version targeted for the upgrade.
- **View:** Represents the `start_proposing_view` value, marking when the upgrade proposal initiates.
- **Propose Window:** Refers to the view window between `start_proposing_view` and `stop_proposing_view`.

We currently support only chain config upgrades. The `upgrade.chain_config` table contains the complete set of chain config parameters, which can be used, for example, to enable protocol fees or modify other parameters during an upgrade.
