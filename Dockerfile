# Build Service Node
FROM debian:buster-slim as mentat-node-builder
ARG SERVICE="rosetta-snarkos"
ARG BRANCH="containerized-deployment"

RUN mkdir -p /app \
    && chown -R nobody:nogroup /app
WORKDIR /app

ENV DEBIAN_FRONTEND noninteractive
RUN apt-get update && apt-get install -y curl

RUN curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/monadicus/mentat/$BRANCH/$SERVICE/install.sh | bash

# Build Rosetta Mentat
FROM debian:buster-slim as rosetta-mentat-builder
ARG SERVICE="rosetta-snarkos"
ARG BRANCH="containerized-deployment"

RUN mkdir -p /app \
    && chown -R nobody:nogroup /app
WORKDIR /app

ENV DEBIAN_FRONTEND noninteractive
RUN apt-get update && apt-get install -y curl clang gcc git

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH=/root/.cargo/bin:$PATH

RUN git clone -b $BRANCH https://github.com/monadicus/mentat.git \
    && cd mentat \
    && cargo build --release --bin "$SERVICE" --features "$SERVICE" \
    && mv ./target/release/"$SERVICE" /app

## Build Final Image
FROM debian:buster-slim

ARG SERVICE="rosetta-snarkos"
ENV ADDRESS "0.0.0.0"
ENV ROCKET_ENV "production"

ENV DEBIAN_FRONTEND noninteractive
RUN apt-get update && apt-get install -y git

RUN mkdir -p /app \
    && chown -R nobody:nogroup /app \
    && mkdir -p /data \
    && chown -R nobody:nogroup /data

WORKDIR /app

# Copy binary from mentat-node-builder
COPY --from=mentat-node-builder /app/node-runner /app/node-runner

# Copy binary from rosetta-mentat-builder
COPY --from=rosetta-mentat-builder /app/$SERVICE /app/rosetta-mentat-service

# Set permissions for everything added to /app
RUN chmod -R 755 /app/*

CMD ["/app/rosetta-mentat-service"]
