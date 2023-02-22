# The zkevm-node docker-compose file currently only works if run from the zkevm-node/test directory.
compose-base := "docker compose --project-name demo -f docker-compose.yaml -f permissionless-docker-compose.yaml"
compose-espresso := "docker compose --project-name demo -f docker-compose.yaml"
compose-anvil := compose-base + " -f docker-compose-anvil.yaml --env-file .env.anvil"
compose := compose-base + " -f docker-compose-geth.yaml"
compose-zkevm-node := "docker compose --project-name demo -f permissionless-docker-compose.yaml -f docker-compose-geth.yaml"

zkevm-node:
    cargo run --all-features --bin zkevm-node

demo:
    cargo run --all-features --bin zkevm-node
    {{compose-espresso}} up -V --force-recreate --abort-on-container-exit

demo-anvil:
    cargo run --all-features --bin zkevm-node -- --backend anvil
    {{compose-espresso}} up -V --force-recreate --abort-on-container-exit

demo-background:
    {{compose}} --project-name demo up -d -V --force-recreate

down:
    {{compose}} down -v --remove-orphans

pull:
    {{compose}} pull && {{compose-anvil}} pull

hardhat *args:
    cd zkevm-contracts && nix develop -c bash -c "npx hardhat {{args}}"

update-contract-bindings: (hardhat "compile")
   cargo run --bin gen-bindings

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
