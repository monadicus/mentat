# Mentat Bitcoin Rosetta Implementation

## Running

### Regtest Node

Running this regtest bitcoin example requires `docker` and uses docker for the running the node.

This node is setup for you and works out of the box.

Start the rosetta-bitcoin with: `cargo run -- regtest.toml`

While this is running, you can generate a "wallet_1" with `./regtest/node_bootstrap.sh`
