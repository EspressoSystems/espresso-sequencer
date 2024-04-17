# Espresso

[![Build](https://github.com/EspressoSystems/espresso-sequencer/actions/workflows/build.yml/badge.svg)](https://github.com/EspressoSystems/espresso-sequencer/actions/workflows/build.yml)
[![Contracts](https://github.com/EspressoSystems/espresso-sequencer/actions/workflows/contracts.yml/badge.svg)](https://github.com/EspressoSystems/espresso-sequencer/actions/workflows/contracts.yml)
[![Lint](https://github.com/EspressoSystems/espresso-sequencer/actions/workflows/lint.yml/badge.svg)](https://github.com/EspressoSystems/espresso-sequencer/actions/workflows/lint.yml)
[![Audit](https://github.com/EspressoSystems/espresso-sequencer/actions/workflows/audit.yml/badge.svg)](https://github.com/EspressoSystems/espresso-sequencer/actions/workflows/audit.yml)
[![Ubuntu](https://github.com/EspressoSystems/espresso-sequencer/actions/workflows/ubuntu-install-without-nix.yml/badge.svg)](https://github.com/EspressoSystems/espresso-sequencer/actions/workflows/ubuntu-install-without-nix.yml)
[![Coverage Status](https://coveralls.io/repos/github/EspressoSystems/espresso-sequencer/badge.svg?branch=main)](https://coveralls.io/github/EspressoSystems/espresso-sequencer?branch=main)

Espresso offers rollups credible neutrality and enhanced interoperability, without compromising on scale.
Consisting of a marketplace for shared sequencing, the HotShot finality gadget, and the Tiramisu data availability layer, applications such as
rollups and general layer-2 chains can leverage Espresso to give developers and end users fast confirmations, low fees,
and robust infrastructure.

[Documentation](https://docs.espressosys.com/sequencer/espresso-sequencer-architecture/readme)

![Architecture diagram](./doc/architecture.svg)

# Running the demo

Refer to [sequencer-example-l2](https://github.com/EspressoSystems/sequencer-example-l2) for instructions on how to run
a dockerized Espresso network with an example Layer 2 rollup application.

# Development

- Obtain code: `git clone git@github.com:EspressoSystems/espresso-sequencer`.
- Make sure [nix](https://nixos.org/download.html) is installed.
- Activate the environment with `nix-shell`, or `nix develop`, or `direnv allow` if using [direnv](https://direnv.net/).
- For installation without nix please see [ubuntu.md](./doc/ubuntu.md).

## Run the tests

    just pull # to pull docker images
    just test

## Building figures

    make doc

## Building and running

Docker images and the [docker-compose-demo.yaml](docker-compose-demo.yaml) file are provided for convenience. The
Docker-based demo fetches the images from the `ghcr` repository, where they are updated with every push to `main` on
GitHub. For testing uncommitted changes, you can also run the binaries by manually building and running the services.

Build all executables with `cargo build --release`. You may then start a sequencer network. First, start an
orchestrator. Choose a port `$PORT` to run it on and decide how many sequencer nodes `$N` you will use, then run
`target/release/orchestrator -p $PORT -n $N`.

The sequencer will distribute a HotShot configuration to all the nodes which connect to it, which specifies consensus
parameters like view timers. There is a default config, but you can override any parameters you want by passing
additional options to the `orchestrator` executable. Run `target/release/orchestrator --help` to see a list of available
options. Next, you must launch a `cdn` instance, which are necessary to facilitate consensus.

```bash
target/release/dev-cdn -p 1738
```

Once you have started the orchestrator and the web servers, you must connect `$N` sequencer nodes to them, after which
the network will start up automatically. To start one node, run

```bash
target/release/sequencer \
    --orchestrator-url http://localhost:$PORT \
    --cdn-endpoint "127.0.0.1:1738"  \
    -- http --port 8083 -- query --storage-path storage -- submit
```

A useful Bash snippet for running `$N` nodes simultaneously in the background of your shell is:

```bash
for i in `seq $N`; do
    target/release/sequencer \
        --orchestrator-url http://localhost:$PORT \
        --cdn-endpoint "127.0.0.1:1738"  \
done
```

For running a full demo natively run `just demo-native`.

### Contracts

#### Development

A foundry project for the contracts specific to HotShot can be found in the directory `contracts`.

To compile

```shell
forge build
```

To run the tests

```shell
forge test
```

In order to avoid constant warnings about checksum mismatches with [svm-rs](https://github.com/roynalnaruto/svm-rs)
managed `solc` we set `FOUNDRY_SRC` to solc installed via flake.nix.

- To use the contracts from rust generate the rust contracts bindings: `just gen-bindings`.
- Bindings are only generated for contracts in the `contracts/src` folder

To generate documentation in `./docs` for solidity code run

```shell
forge doc
```

#### Deployment

To deploy the contracts to a local testnet, first run a dev chain (e.g. `anvil`), then run

    forge script DeployHotShot --broadcast --rpc-url local

To deploy to sepolia set `SEPOLIA_RPC_URL` and `MNEMONIC` env vars and run

    forge script DeployHotShot --broadcast --rpc-url sepolia

To additionally verify the contact on etherscan set the `ETHERSCAN_API_KEY` env var and run

    forge script DeployHotShot --broadcast --rpc-url sepolia --verify

Running the script will save a file with details about the deployment in `contracts/broadcast/$CHAIN_ID`.

#### Folder Structure Rationale

- code for demo purposes goes into the `contracts/demo` folder
- code that eventually ends up in production goes into the `contracts/src` folder

## Misc

### Authenticate with GitHub container registry

This is only necessary to fetch private images.

- Go to your github profile
- Developer Settings > Personal access tokens > Personal access tokens (classic)
- Generate a new token
  - for the scope options of the token, tick the _repo_ box.
- Run `docker login ghcr.io --username <you_github_id> --password <your_personal_access_token>`

# License

## Copyright

**(c) 2022 Espresso Systems** `espresso-sequencer` was developed by Espresso Systems. While we plan to adopt an open
source license, we have not yet selected one. As such, all rights are reserved for the time being. Please reach out to
us if you have thoughts on licensing.

# Disclaimer

**DISCLAIMER:** This software is provided "as is" and its security has not been externally audited. Use at your own
risk.
