default:
    just --list

demo *args:
    docker compose up {{args}}

down *args:
    docker compose down {{args}}

docker-cli *cmd:
    docker exec -it espresso-sequencer-example-rollup-1 bin/cli {{cmd}}

cli *cmd:
    target/release/cli {{cmd}}

pull:
    docker compose pull

docker-stop-rm:
    docker stop $(docker ps -aq); docker rm $(docker ps -aq)

anvil *args:
    docker run -p 127.0.0.1:8545/8545 ghcr.io/foundry-rs/foundry:latest "anvil {{args}}"

test:
    cargo test --release --all-features

# Helpful shortcuts for local development
dev-orchestrator:
    target/release/orchestrator -p 8080 -n 1 

dev-da-server:
    target/release/web-server -p 8081

dev-consensus-server:
    target/release/web-server -p 8082

dev-sequencer:
    target/release/sequencer \
    --orchestrator-url http://localhost:8080 \
    --da-server-url http://localhost:8081 \
    --consensus-server-url http://localhost:8082 \
    -- http --port 8083  -- query --storage-path storage

dev-commitment:
     target/release/commitment-task --sequencer-url http://localhost:50000 \
     --l1-provider http://localhost:8545 \
     --eth-mnemonic "test test test test test test test test test test test junk" \
     --deploy

build-docker-images:
    scripts/build-docker-images

# generate rust bindings for contracts
gen-bindings:
    forge bind --crate-name contract-bindings --force
    cargo fmt --all
    cargo sort -g -w

# Lint solidity files
sol-lint:
    forge fmt
    solhint --fix 'contracts/{script,src,test}/**/*.sol'
