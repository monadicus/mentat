#!/usr/bin/env bash

apt-get install -y build-essential curl clang gcc git libssl-dev llvm make pkg-config xz-utils

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

git clone https://github.com/AleoHQ/snarkOS.git --depth 1 \
    && cd snarkOS \
    && cargo build --release \
    && mv ./target/release/snarkos /app/node-runner
