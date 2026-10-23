### State peers must serve the light client at startup

Urgency: Medium

A node whose local storage lacks the stake tables for the epoch window of the L1 light-client anchor (fresh join or
storage far behind) fetches them at startup. `ESPRESSO_NODE_STATE_PEERS` must then include at least one node serving the
query module with SQL storage (the `/light-client` and `/availability` APIs). Without one, startup exits.

Startup also reads the light-client contract on L1.

`ESPRESSO_NODE_BOOTSTRAP_DEADLINE` (default 15m) bounds the startup catchup. At the deadline the node exits.
