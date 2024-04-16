default:
    just --list

demo *args:
    docker compose up {{args}}

demo-native:
    cargo build --release
    scripts/demo-native

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
    docker run -p 127.0.0.1:8545:8545 ghcr.io/foundry-rs/foundry:latest "anvil {{args}}"

test:
    cargo build --bin diff-test --release
    cargo test --release --all-features

# Helpful shortcuts for local development
dev-orchestrator:
    target/release/orchestrator -p 8080 -n 1

dev-cdn:
    RUST_LOG=info target/release/dev-cdn

dev-state-relay-server:
    target/release/state-relay-server -p 8083

dev-sequencer:
    target/release/sequencer \
    --orchestrator-url http://localhost:8080 \
    --cdn-endpoint "127.0.0.1:1738" \
    --state-relay-server-url http://localhost:8083 \
    -- http --port 8083  -- query --storage-path storage

dev-commitment:
     target/release/commitment-task --sequencer-url http://localhost:50000 \
     --l1-provider http://localhost:8545 \
     --eth-mnemonic "test test test test test test test test test test test junk" \
     --deploy

build-docker-images:
    scripts/build-docker-images

# generate rust bindings for contracts
REGEXP := "^LightClient$|^LightClientStateUpdateVK$|^FeeContract$|^HotShot$|PlonkVerifier$|^ERC1967Proxy$"
gen-bindings:
    forge bind --contracts ./contracts/src/ --crate-name contract-bindings --bindings-path contract-bindings --select "{{REGEXP}}" --overwrite --force

    # Foundry doesn't include bytecode in the bindings for LightClient.sol, since it links with
    # libraries. However, this bytecode is still needed to link and deploy the contract. Copy it to
    # the source tree so that the deploy script can be compiled whenever the bindings are up to
    # date, without needed to recompile the contracts.
    mkdir -p contract-bindings/artifacts
    jq '.bytecode.object' < contracts/out/LightClient.sol/LightClient.json > contract-bindings/artifacts/LightClient_bytecode.json

    cargo fmt --all
    cargo sort -g -w

# Lint solidity files
sol-lint:
    forge fmt
    solhint --fix 'contracts/{script,src,test}/**/*.sol'

# Build diff-test binary and forge test
# Note: we use an invalid etherscan api key in order to avoid annoying warnings. See https://github.com/EspressoSystems/espresso-sequencer/issues/979
sol-test:
    cargo build --bin diff-test --release
    forge test

# Deploy contracts to local blockchain for development and testing
dev-deploy url="" mnemonics="" num_blocks_per_epoch="10" num_init_validators="5" admin="0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266" private_key="":
    MNEMONICS="{{mnemonics}}" forge script contracts/test/DeployLightClientTestScript.s.sol:DeployLightClientTestScript \
    --sig "run(uint32 numBlocksPerEpoch, uint64 numInitValidators, address owner)" {{num_blocks_per_epoch}} {{num_init_validators}} {{admin}} \
    --fork-url {{url}} --broadcast \
    --private-key {{private_key}}


# This is meant for local development and produces HTML output. In CI
# the lcov output is pushed to coveralls.
code-coverage:
  @echo "Running code coverage"
  nix develop .#coverage -c cargo test --all-features --no-fail-fast --release --workspace -- --skip service::test::test_
  grcov . -s . --binary-path $CARGO_TARGET_DIR/debug/ -t html --branch --ignore-not-existing -o $CARGO_TARGET_DIR/coverage/ \
      --ignore 'contract-bindings/*' --ignore 'contracts/*'
  @echo "HTML report available at: $CARGO_TARGET_DIR/coverage/index.html"
