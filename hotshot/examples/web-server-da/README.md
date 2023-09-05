Commands to run da examples: 
1a)Start web servers by either running 3 servers:
cargo run --example web-server --profile=release-lto --features="full-ci" <PORT_FOR_CDN> 
cargo run --example web-server --profile=release-lto --features="full-ci" <PORT_FOR_DA> 
cargo run --example web-server --profile=release-lto --features="full-ci" <PORT_FOR_VIEW_SYNC> 

1b)Or use multi-web-server to spin up all three:
cargo run --example multi-web-server --profile=release-lto --features="full-ci" <PORT_FOR_CDN> <PORT_FOR_DA> <PORT_FOR_VIEW_SYNC> 

2) Start orchestrator:
cargo run --example web-server-da-orchestrator --features="full-ci,channel-async-std" --profile=release-lto -- <ORCHESTRATOR_ADDR> <ORCHESTRATOR_PORT> <ORCHESTRATOR_CONFIG_FILE> 

3a) Start validator:
cargo run --profile=release-lto --example web-server-da-validator --features="full-ci" <ORCHESTRATOR_ADDR> <ORCHESTRATOR_PORT>

3b) Or start multiple validators:
cargo run --profile=release-lto --example multi-validator --features="full-ci" <NUM_VALIDATORS> <ORCHESTRATOR_ADDR> <ORCHESTRATOR_PORT>

I.e. 
cargo run --example web-server --profile=release-lto --features="full-ci" 9000 
cargo run --example web-server --profile=release-lto --features="full-ci" 9001 
cargo run --example web-server --profile=release-lto --features="full-ci" 9002
cargo run --example web-server-da-orchestrator --features="full-ci,channel-async-std" --profile=release-lto -- 0.0.0.0 4444 ./orchestrator/default-run-config.toml 
cargo run --profile=release-lto --example web-server-da-validator --features="full-ci" 2 0.0.0.0 4444

OR: 
cargo run --example multi-web-server --profile=release-lto --features="full-ci" 9000 9001 9002
cargo run --example web-server-da-orchestrator --features="full-ci,channel-async-std" --profile=release-lto -- 0.0.0.0 4444 ./orchestrator/default-run-config.toml 
cargo run --profile=release-lto --example multi-validator --features="full-ci" 10 0.0.0.0 4444