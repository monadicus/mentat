#!/usr/bin/env bash

apk add cargo clang gcc git g++ libressl-dev linux-headers openssl rust

git clone https://github.com/AleoHQ/snarkOS.git --depth 1 \
    && cd snarkOS \
    && cargo build --release \
    && mv ./target/release/snarkos /app/node-runner
