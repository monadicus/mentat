#!/usr/bin/env bash

# setup test wallets
cat <<SCRIPT | $(dirname $0)/node_bash.sh
if [ ! -d "/bitcoin/.bitcoin/regtest/wallets/wallet_1" ]; then
  bitcoin-cli createwallet "wallet_1"
  bitcoin-cli getnewaddress
  bitcoin-cli -generate -rpcwallet=wallet_1 100 >/dev/null
fi
SCRIPT
