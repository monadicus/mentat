.PHONY: deps build run lint mocks run-mainnet-online run-mainnet-offline run-testnet-online \
	run-testnet-offline check-comments add-license check-license shorten-lines test \
	coverage spellcheck salus build-local coverage-local format check-format

PWD=$(shell pwd)
NOFILE=100000

build-snarkos:
	docker build -t rosetta-snarkos:latest https://github.com/monadicus/mentat.git --build-arg SERVICE=rosetta-snarkos

build-local-snarkos:
	docker build -t rosetta-snarkos:latest . --build-arg SERVICE=rosetta-snarkos

build-release-snarkos:
	# make sure to always set version with vX.X.X
	docker build -t rosetta-snarkos:$(version) .;  --build-arg SERVICE=rosetta-snarkos
	docker save rosetta-snarkos:$(version) | gzip > rosetta-snarkos-$(version).tar.gz;

build:
	docker build -t "$SERVICE":latest https://github.com/monadicus/mentat.git --build-arg SERIVE=$SERVICE

run-mainnet-online:
	docker run -d --rm --ulimit "nofile=${NOFILE}:${NOFILE}" -v "${PWD}/bitcoin-data:/data" -e "MODE=ONLINE" -e "NETWORK=MAINNET" -e "PORT=8080" -p 8080:8080 -p 8333:8333 rosetta-snarkok:latest

run-mainnet-offline:
	docker run -d --rm -e "MODE=OFFLINE" -e "NETWORK=MAINNET" -e "PORT=8081" -p 8081:8081 rosetta-snarkok:latest

run-testnet-online:
	docker run -d --rm --ulimit "nofile=${NOFILE}:${NOFILE}" -v "${PWD}/bitcoin-data:/data" -e "MODE=ONLINE" -e "NETWORK=TESTNET" -e "PORT=8080" -p 8080:8080 -p 18333:18333 rosetta-snarkok:latest

run-testnet-offline:
	docker run -d --rm -e "MODE=OFFLINE" -e "NETWORK=TESTNET" -e "PORT=8081" -p 8081:8081 rosetta-snarkok:latest

format:
	cargo +nightly fmt --all

check-format:
	cargo +nightly fmt --all
	cargo clippy --all

test:
	cargo test --all
