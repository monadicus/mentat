<p align="center">
  <a href="https://www.rosetta-api.org">
    <img width="90%" alt="Rosetta" src="https://www.rosetta-api.org/img/rosetta_header.png">
  </a>
</p>

<h3 align="center">
   Mentat
</h3>

# Overview

`Mentant` is a zero dependency framework that makes implementing and calling the Rosetta API for a myriad of Blockchains easier.
It is written in Rust for high performance, and genercism such that it could be used for any Blockchain.

# Features

- Server.
  - Default Not Implemented Routes
  - Logging.
  - Default support for Offline and Online mode.
  - Easily run your Blockchain node with proper logging.
- Client.

- Keys(Different signature schemes are supported).
  - Aleo/Snarkos
- Easy To Containerize.

# System Requirements

`Mentat` itself is lightweight and easy to integrate anywhere.
However, please refer to each Blockchain implementation using `Mentat` to see more requirements.

# Usage

As specified in the [Rosetta API Principles](https://www.rosetta-api.org/docs/automated_deployment.html),
all Rosetta implementations must be deployable via Docker and support running via either an
[`online` or `offline` mode](https://www.rosetta-api.org/docs/node_deployment.html#multiple-modes).

**YOU MUST INSTALL DOCKER FOR THE FOLLOWING INSTRUCTIONS TO WORK. YOU CAN DOWNLOAD
DOCKER [HERE](https://www.docker.com/get-started).**

## Install

Running the following commands will create a Docker image called `rosetta-snaroks:latest`.

Change `rosetta-snarkos` with any other Mentat supported Blockchain.

### From GitHub

To download the pre-built Docker image from the latest release, run:

```text
curl -sSfL https://raw.githubusercontent.com/monadicus/mentat/main/rosetta-snarkos/install.sh | sh -s
```

### From Source

After cloning this repository, and changing directory to a service run:

```text
make build-local
```

#### Run

Running the following commands will start a Docker container in
[detached mode](https://docs.docker.com/engine/reference/run/#detached--d) with
a data directory at `<working directory>/service-data` and the Rosetta API accessible
at port `8080`.

##### Configuration Environment Variables

MAY VARY BETWEEN IMPLEMENTATIONS OF ROSETTA USING MENTAT.

- `MODE` (optional) - Determines if Rosetta can make outbound connections. Options: `ONLINE` or `OFFLINE` (which defaults to `ONLINE`).
- `NETWORK` (optional) - Service network to launch and/or communicate with. Options: `MAINNET` or `TESTNET` (which defaults to `MAINNET`).
- `PORT` (required) - Which port to use for the Rosetta service.

##### Mainnet:Online

```text
docker run -d --rm --ulimit "nofile=100000:100000" -v "$(pwd)/snarkos-data:/data" -e "MODE=ONLINE" -e "NETWORK=MAINNET" -e "PORT=8080" -p 8080:8080 -p 30303:30303 rosetta-snarkos:latest
```

_If you cloned the repository, you can run `make run-mainnet-online`._

##### Mainnet:Online (Remote)

```text
docker run -d --rm --ulimit "nofile=100000:100000" -e "MODE=ONLINE" -e "NETWORK=MAINNET" -e "PORT=8080" -p 8080:8080 -p 30303:30303 rosetta-snarkos:latest
```

_If you cloned the repository, you can run `make run-mainnet-remote`._

##### Mainnet:Offline

```text
docker run -d --rm -e "MODE=OFFLINE" -e "NETWORK=MAINNET" -e "PORT=8081" -p 8081:8081 rosetta-snarkos:latest
```

_If you cloned the repository, you can run `make run-mainnet-offline`._

##### Testnet:Online

```text
docker run -d --rm --ulimit "nofile=100000:100000" -v "$(pwd)/snarkos-data:/data" -e "MODE=ONLINE" -e "NETWORK=TESTNET" -e "PORT=8080" -p 8080:8080 -p 30303:30303 rosetta-snarkos:latest
```

_If you cloned the repository, you can run `make run-testnet-online`._

##### Testnet:Online (Remote)

```text
docker run -d --rm --ulimit "nofile=100000:100000" -e "MODE=ONLINE" -e "NETWORK=TESTNET" -e "PORT=8080" -p 8080:8080 -p 30303:30303 rosetta-snarkos:latest
```

_If you cloned the repository, you can run `make run-testnet-remote`._

##### Testnet:Offline

```text
docker run -d --rm -e "MODE=OFFLINE" -e "NETWORK=TESTNET" -e "PORT=8081" -p 8081:8081 rosetta-snarkos:latest
```

_If you cloned the repository, you can run `make run-testnet-offline`._

# Testing

TODO @MACS-J1149

# Issues

Interested in helping fix issues in this repository? You can find to-dos in the [Issues](https://github.com/monadicus/mentat/issues) section.

# Development

TODO after swapping to `bazel` from `make`.

# License

This project is available open source under the terms of the MIT License.

© 2022 Monadicus
