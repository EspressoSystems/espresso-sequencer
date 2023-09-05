# Orchestrator

This crate implements an orchestrator that coordinates starting the network with a particular configuration.  It is useful for testing and benchmarking.  Like the web server, the orchestrator is built using [Tide Disco](https://github.com/EspressoSystems/tide-disco).  

To run the orchestrator for a libp2p network: `cargo run --example libp2p-orchestrator --features="full-ci,channel-async-std" 0.0.0.0 3333 ./orchestrator/default-libp2p-run-config.toml `

To run the orchestrator for a libp2p network: `cargo run --example web-server-orchestrator --features="full-ci,channel-async-std" 0.0.0.0 3333 ./orchestrator/default-web-server-run-config.toml `