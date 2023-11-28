Commands to run da examples: 
1a)Start web servers by either running 3 servers:
just async_std example webserver -- <PORT_FOR_CDN> 
just async_std example webserver -- <PORT_FOR_DA> 
just async_std example webserver -- <PORT_FOR_VIEW_SYNC> 

1b)Or use multi-webserver to spin up all three:
just async_std example multi-webserver -- <PORT_FOR_CDN> <PORT_FOR_DA> <PORT_FOR_VIEW_SYNC> 

2) Start orchestrator:
just async_std example orchestrator-webserver -- <ORCHESTRATOR_ADDR> <ORCHESTRATOR_PORT> <ORCHESTRATOR_CONFIG_FILE> 

3a) Start validator:
just async_std example validator-webserver -- <ORCHESTRATOR_ADDR> <ORCHESTRATOR_PORT>

3b) Or start multiple validators:
just async_std example multi-validator-webserver -- <NUM_VALIDATORS> <ORCHESTRATOR_ADDR> <ORCHESTRATOR_PORT>

I.e. 
just async_std example webserver -- 9000 
just async_std example webserver -- 9001 
just async_std example webserver -- 9002
just async_std example orchestrator-webserver -- 0.0.0.0 4444 ./orchestrator/default-run-config.toml 
just async_std example validator-webserver -- 2 0.0.0.0 4444

OR: 
just async_std example multi-webserver -- 9000 9001 9002
just async_std example orchestrator-webserver -- 0.0.0.0 4444 ./orchestrator/default-run-config.toml 
just async_std example multi-validator-webserver -- 10 0.0.0.0 4444