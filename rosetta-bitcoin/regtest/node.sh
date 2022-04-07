#!/usr/bin/env bash

SELF=$(readlink -f "$0")
BASEDIR=$(dirname "$SELF")

mkdir -p $BASEDIR/regtest_data

docker run -t --rm \
  -p 8332:8332 \
  -v "$BASEDIR/regtest_data:/bitcoin/.bitcoin" \
  -v "$BASEDIR/bitcoin.conf:/bitcoin/.bitcoin/bitcoin.conf" \
  --name bitcoin_regtest \
  kylemanna/bitcoind
