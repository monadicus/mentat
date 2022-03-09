# Build Service Node
ARG SERVICE="rosetta-snarkos"


# Build Rosetta Mentat
FROM alpine:3.15.0 as rosetta-mentat-builder

ARG SERVICE="rosetta-snarkos"
ARG BRANCH="containerized-deployment"

RUN mkdir -p /app \
  && chown -R nobody:nogroup /app
WORKDIR /app

# Source: https://github.com/bitcoin/bitcoin/blob/master/doc/build-unix.md#ubuntu--debian
ARG DEBIAN_FRONTEND=noninteractive
ENV TZ Etc/UTC

RUN apk -U upgrade && apk add alpine-sdk

# Install Rust stable
RUN curl --proto '=https' --tlsv1.3 -sSf https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN git clone -b $BRANCH https://github.com/monadicus/mentat.git \
    && cd mentat \
    && cargo build --release --bin "$SERVICE" --features "$SERVICE" \
    && mv ./target/release/"$SERVICE" /app

## Build Final Image
FROM alpine:3.15.0

ARG SERVICE="rosetta-snarkos"
ENV ADDRESS "0.0.0.0"

RUN apk update

RUN mkdir -p /app \
  && chown -R nobody:nogroup /app \
  && mkdir -p /data \
    && chown -R nobody:nogroup /data

WORKDIR /app

# Copy binary from rosetta-mentat-builder
COPY --from=rosetta-mentat-builder /app/$SERVICE /app/rosetta-mentat-service

# Set permissions for everything added to /app
RUN chmod -R 755 /app/*

CMD ["/app/rosetta-mentat-service"]
