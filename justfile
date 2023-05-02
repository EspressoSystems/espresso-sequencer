# The zkevm-node docker-compose file currently only works if run from the zkevm-node/test directory.
compose-base := "docker compose --project-name demo -f docker-compose.yaml -f permissionless-docker-compose.yaml"
compose-espresso := "docker compose --project-name demo -f docker-compose.yaml"
compose-anvil := compose-base + " -f docker-compose-anvil.yaml"
compose := compose-base + " -f docker-compose-geth.yaml"
compose-zkevm-node := "docker compose --project-name demo -f permissionless-docker-compose.yaml -f docker-compose-geth.yaml"

zkevm-node:
    cargo run --all-features --bin zkevm-node

demo:
    cargo run --all-features --bin zkevm-node -- --detach
    {{compose-espresso}} up -V --force-recreate --abort-on-container-exit || just down

rollup-demo:
    docker compose -f docker-compose-demo.yaml up   

rollup-demo-down:
    docker compose -f docker-compose-demo.yaml down

down:
    {{compose}} down -v --remove-orphans

pull:
    {{compose}} pull && {{compose-anvil}} pull

hardhat *args:
    cd zkevm-contracts && nix develop -c bash -c "npx hardhat {{args}}"

update-contract-bindings:
    cargo run --bin gen-bindings

update-zkevm-node-contract-bindings:
    scripts/update-zkevm-node-contract-bindings

npm *args:
   cd zkevm-contracts && nix develop -c bash -c "npm {{args}}"

compose *args:
   {{compose}} {{args}}

docker-stop-rm:
    docker stop $(docker ps -aq); docker rm $(docker ps -aq)

anvil *args:
    docker run ghcr.io/foundry-rs/foundry:latest "anvil {{args}}"

build-docker-zkevm-node:
    cd zkevm-node && nix develop -c bash -c "make build-docker && docker tag zkevm-node:latest ghcr.io/espressosystems/zkevm-node:hotshot-integration"

build-docker-l1-geth:
    cd zkevm-contracts && nix develop -c bash -c "npm run docker:contracts && docker tag hermeznetwork/geth-zkevm-contracts:latest ghcr.io/espressosystems/geth-zkevm-contracts:hotshot-integration"

build-docker: build-docker-l1-geth build-docker-zkevm-node

test:
    cargo test --release --all-features

# Helpful shortcuts for local development
dev-cdn:
    target/release/cdn-server -p 8080 -n 1 

dev-sequencer:
    target/release/sequencer --cdn-url tcp://127.0.0.1:8080 --port 8081 --storage-path storage

dev-l1:
    docker compose -f docker-compose-anvil.yaml up

dev-demo:
     target/release/example-l2 --sequencer-url http://localhost:8081 \
     --l1-provider http://localhost:8545 \
     --hotshot-address 0x5fbdb2315678afecb367f032d93f642f64180aa3 \
     --rollup-address 0xe7f1725e7734ce288f8367e1bb143e90bb3f0512 \
     --rollup-mnemonic "test test test test test test test test test test test junk"
