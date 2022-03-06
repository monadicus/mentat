# Build Service Node
FROM alpine:3.15.0 as mentat-node-builder
ARG SERVICE="rosetta-snarkos"

RUN mkdir -p /app \
  && chown -R nobody:nogroup /app
WORKDIR /app

RUN apk -U upgrade && apk add cargo clang gcc git g++ libressl-dev linux-headers openssl rust

RUN git clone https://github.com/AleoHQ/snarkOS.git --depth 1 \
    && cd snarkOS \
    && cargo build --release \
    && mv ./target/release/snarkos /app/node-runner

# Build Rosetta Mentat
FROM alpine:edge as rosetta-mentat-builder

ARG SERVICE="rosetta-snarkos"
ARG BRANCH="containerized-deployment"

RUN mkdir -p /app \
  && chown -R nobody:nogroup /app
WORKDIR /app

RUN apk -U upgrade && apk add cargo git rust

RUN git clone -b $BRANCH https://github.com/monadicus/mentat.git \
    && cd mentat \
    && cargo build --release --bin "$SERVICE" --features "$SERVICE" \
    && mv ./target/release/"$SERVICE" /app

## Build Final Image
FROM alpine:3.15.0

ARG SERVICE="rosetta-snarkos"
ENV ADDRESS "0.0.0.0"

RUN apk update && apk add clang gcc git g++ libressl-dev linux-headers openssl

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

# Run the node
RUN /app/node-runner --trial --verbosity 2

CMD ["/app/rosetta-mentat-service"]
