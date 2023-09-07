Commands to run da examples: 
1a)Start web servers by either running 3 servers:
just async_std example web-server -- <PORT_FOR_CDN> 
just async_std example web-server -- <PORT_FOR_DA> 
just async_std example web-server -- <PORT_FOR_VIEW_SYNC> 

1b)Or use multi-web-server to spin up all three:
just async_std example multi-web-server -- <PORT_FOR_CDN> <PORT_FOR_DA> <PORT_FOR_VIEW_SYNC> 

2) Start orchestrator:
just async_std example web-server-da-orchestrator -- <ORCHESTRATOR_ADDR> <ORCHESTRATOR_PORT> <ORCHESTRATOR_CONFIG_FILE> 

3a) Start validator:
just async_std example web-server-da-validator -- <ORCHESTRATOR_ADDR> <ORCHESTRATOR_PORT>

3b) Or start multiple validators:
just async_std example multi-validator -- <NUM_VALIDATORS> <ORCHESTRATOR_ADDR> <ORCHESTRATOR_PORT>

I.e. 
just async_std example web-server -- 9000 
just async_std example web-server -- 9001 
just async_std example web-server -- 9002
just async_std example web-server-da-orchestrator -- 0.0.0.0 4444 ./orchestrator/default-run-config.toml 
just async_std example web-server-da-validator -- 2 0.0.0.0 4444

OR: 
just async_std example multi-web-server -- 9000 9001 9002
just async_std example web-server-da-orchestrator -- 0.0.0.0 4444 ./orchestrator/default-run-config.toml 
just async_std example multi-validator -- 10 0.0.0.0 4444