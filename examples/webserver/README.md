Commands to run da examples: 
1a)Start web servers by either running 3 servers:
```
just async_std example webserver -- <URL_FOR_CDN>
just async_std example webserver -- <URL_FOR_DA>
```

2) Start orchestrator:
```
just async_std example orchestrator-webserver -- --orchestrator_url <ORCHESTRATOR_URL> --config_file <ORCHESTRATOR_CONFIG_FILE> 
```

3a) Start validator:
```
just async_std example validator-webserver -- <ORCHESTRATOR_URL>
```

3b) Or start multiple validators:
```
just async_std example multi-validator-webserver -- <NUM_VALIDATORS> <ORCHESTRATOR_URL>
```

I.e. 
```
just async_std example webserver -- http://127.0.0.1:9000 
just async_std example webserver -- http://127.0.0.1:9001 
just async_std example orchestrator-webserver -- --config_file ./crates/orchestrator/run-config.toml --orchestrator_url http://127.0.0.1:4444 --total_nodes 10 --da_committee_size 5 --transactions_per_round 1 --transaction_size 512 --rounds 10 --fixed_leader_for_gpuvid 0 --webserver_url http://127.0.0.1:9000 --da_webserver_url http://127.0.0.1:9001 
just async_std example multi-validator-webserver -- 10 http://127.0.0.1:4444
```


OR:

`just async_std example all-webserver -- --config_file ./crates/orchestrator/run-config.toml --orchestrator_url http://localhost:4444`

For other argument setting, checkout `read_orchestrator_init_config` in `crates/examples/infra/mod.rs`.

One example is: `just async_std example all-webserver -- --config_file ./crates/orchestrator/run-config.toml --total_nodes 15`.

Another example is `just async_std example all-webserver -- --config_file ./crates/orchestrator/run-config.toml --total_nodes 20 --da_committee_size 5 --transactions_per_round 10 --transaction_size 512 --rounds 100`, I'll get throughput `0.29M/s` locally for this one.

If using gpu-vid, you have to run:
```
just async_std example webserver -- http://127.0.0.1:9000 
just async_std example webserver -- http://127.0.0.1:9001 
just async_std example orchestrator-webserver -- --config_file ./crates/orchestrator/run-config.toml --orchestrator_url http://127.0.0.1:4444 --total_nodes 10 --da_committee_size 5 --fixed_leader_for_gpuvid 1
just async_std example_gpuvid_leader multi-validator-webserver -- 1 http://127.0.0.1:4444
sleep 1m
just async_std example_fixed_leader multi-validator-webserver -- 9 http://127.0.0.1:4444
```

Where ones using `example_gpuvid_leader` could be the leader and should be running on a nvidia GPU, and other validators using `example_fixed_leader` will never be a leader. In practice, these url should be changed to the corresponding ip and port.


If you don't have a gpu but want to test out fixed leader, you can run:
```
just async_std example webserver -- http://127.0.0.1:9000 
just async_std example webserver -- http://127.0.0.1:9001 
just async_std example orchestrator-webserver -- --config_file ./crates/orchestrator/run-config.toml --orchestrator_url http://127.0.0.1:4444 --total_nodes 10 --da_committee_size 5 --transactions_per_round 1 --transaction_size 512 --rounds 10 --fixed_leader_for_gpuvid 2 --webserver_url http://127.0.0.1:9000 --da_webserver_url http://127.0.0.1:9001 
just async_std example_fixed_leader multi-validator-webserver -- 2 http://127.0.0.1:4444
sleep 1m
just async_std example_fixed_leader multi-validator-webserver -- 8 http://127.0.0.1:4444
```

Remember, you have to run leaders first, then other validators, so that leaders will have lower index.