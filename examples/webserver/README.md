Commands to run da examples: 
1a)Start web servers by either running 3 servers:
just async_std example webserver -- <URL_FOR_CDN> <PORT_FOR_CDN>
just async_std example webserver -- <URL_FOR_DA> <PORT_FOR_DA> 

1b)Or use multi-webserver to spin up all three:
just async_std example multi-webserver -- <URL_FOR_CDN> <URL_FOR_DA> <PORT_FOR_CDN> <PORT_FOR_DA>

2) Start orchestrator:
just async_std example orchestrator-webserver -- <ORCHESTRATOR_URL> <ORCHESTRATOR_PORT> <ORCHESTRATOR_CONFIG_FILE> 

3a) Start validator:
just async_std example validator-webserver -- <ORCHESTRATOR_URL> <ORCHESTRATOR_PORT>

3b) Or start multiple validators:
just async_std example multi-validator-webserver -- <NUM_VALIDATORS> <ORCHESTRATOR_URL> <ORCHESTRATOR_PORT>

I.e. 
just async_std example webserver -- http://127.0.0.1:9000 
just async_std example webserver -- http://127.0.0.1:9001 
just async_std example webserver -- http://127.0.0.1:9002
just async_std example orchestrator-webserver -- http://127.0.0.1:4444 ./crates/orchestrator/run-config.toml 
just async_std example validator-webserver -- 2 http://127.0.0.1:4444

OR: 
just async_std example multi-webserver -- 9000 9001 9002
just async_std example orchestrator-webserver -- http://127.0.0.1:4444 ./crates/orchestrator/run-config.toml 
just async_std example multi-validator-webserver -- 10 http://127.0.0.1:4444

================All of the above are out-dated================
OR:
`just async_std example all-webserver -- --config_file ./crates/orchestrator/run-config.toml --orchestrator_url http://localhost:4444`
For other argument setting, checkout `read_orchestrator_initialization_config` in `crates/examples/infra/mod.rs`.
One example is: `just async_std example all-webserver -- --config_file ./crates/orchestrator/run-config.toml --total_nodes 15`.
Another example is `just async_std example all-webserver -- --config_file ./crates/orchestrator/run-config.toml --total_nodes 20 --da_committee_size 5 --transactions_per_round 10 --transaction_size 512 --rounds 100`, I'll get throughput `0.29M/s` for this one.