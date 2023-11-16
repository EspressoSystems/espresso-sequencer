# Installing on ubuntu (without nix)

<!-- Note that all lines that start with four spaces will be executed in the CI -->

## Install system dependencies

    sudo apt-get update
    sudo apt-get install -y curl cmake pkg-config libssl-dev protobuf-compiler git

## Install rustup

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
    source $HOME/.cargo/env

## Install foundry

    curl --proto '=https' --tlsv1.2 -sSf -L https://foundry.paradigm.xyz | bash
    source $HOME/.bashrc
    foundryup

## Clone the repository

    git clone https://github.com/EspressoSystems/espresso-sequencer/
    cd espresso-sequencer

## Run the rust tests

    export RUSTFLAGS='--cfg async_executor_impl="async-std" --cfg async_channel_impl="async-std"'
    cargo test --release

## Run the foundry tests

    cargo build --release --bin diff-test
    export PATH=$PWD/target/release:$PATH
    forge test -v

