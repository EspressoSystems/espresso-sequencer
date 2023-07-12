rollup-demo:
    docker compose -f docker-compose-demo.yaml up

rollup-demo-down:
    docker compose -f docker-compose-demo.yaml down

docker-cli *cmd:
    docker exec -it espresso-sequencer-example-rollup-1 bin/cli {{cmd}}

cli *cmd:
    target/release/cli {{cmd}}

pull-rollup-demo:
    docker compose -f docker-compose-demo.yaml pull

update-contract-bindings:
    cargo run --bin gen-bindings

docker-stop-rm:
    docker stop $(docker ps -aq); docker rm $(docker ps -aq)

anvil *args:
    docker run -p 127.0.0.1:8545/8545 ghcr.io/foundry-rs/foundry:latest "anvil {{args}}"

test:
    cargo test --release --all-features

# Helpful shortcuts for local development
dev-cdn:
    target/release/cdn-server -p 8080 -n 1 

dev-sequencer:
    target/release/sequencer --cdn-url tcp://127.0.0.1:8080 -- api --port 8081 --storage-path storage

dev-demo:
     target/release/example-l2 --sequencer-url http://localhost:8081 \
     --l1-provider http://localhost:8545 \
     --hotshot-address 0x5fbdb2315678afecb367f032d93f642f64180aa3 \
     --rollup-address 0xe7f1725e7734ce288f8367e1bb143e90bb3f0512 \
     --rollup-mnemonic "test test test test test test test test test test test junk"

build-docker-images:
    scripts/build-docker-images
