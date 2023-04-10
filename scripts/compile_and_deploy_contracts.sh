#!/usr/bin/env bash

cd contracts
forge build
cd ..

cargo run --bin gen-bindings