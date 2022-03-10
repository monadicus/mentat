.PHONY: deps build run lint mocks run-mainnet-online run-mainnet-offline run-testnet-online \
	run-testnet-offline check-comments add-license check-license shorten-lines test \
	coverage spellcheck salus build-local coverage-local format check-format

PWD=$(shell pwd)
NOFILE=100000

SERVICE=rosetta-snarkos

build:
	docker build -t mentat-$(SERVICE):latest https://github.com/monadicus/mentat.git --build-arg SERVICE=$(SERVICE)

build-local:
	docker build --no-cache -t mentat-$(SERVICE):latest . --build-arg SERVICE=$(SERVICE)

build-release:
	# make sure to always set version with vX.X.X
	docker build -t mentat-$(SERVICE):$(version) .;  --build-arg SERVICE=$(SERVICE)
	docker save mentat-$(SERVICE):$(version) | gzip > mentat-$(SERVICE)-$(version).tar.gz;

run-mainnet-online:
	docker run -d --rm --ulimit "nofile=${NOFILE}:${NOFILE}" -e "MODE=ONLINE" -e "NETWORK=MAINNET" -e "PORT=8080" -p 8080:8080 mentat-$(SERVICE):latest

run-mainnet-offline:
	docker run -d --rm -e "MODE=OFFLINE" -e "NETWORK=MAINNET" -e "PORT=8081" -p 8081:8081 -p 4132:4132 -p 3032:3032 mentat-$(SERVICE):latest

run-testnet-online:
	docker run -d --rm --ulimit "nofile=${NOFILE}:${NOFILE}" -e "MODE=ONLINE" -e "NETWORK=TESTNET" -e "PORT=8080" -p 8080:8080 mentat-$(SERVICE):latest

run-testnet-offline:
	docker run -d --rm -e "MODE=OFFLINE" -e "NETWORK=TESTNET" -e "PORT=8081" -p 8081:8081 mentat-$(SERVICE):latest

format:
	cargo +nightly fmt --all

check-format:
	cargo +nightly fmt --all
	cargo clippy --all

test:
	cargo test --all

clean:
	cargo clean
