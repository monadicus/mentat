# Build Service Node
FROM alpine:edge as mentat-node-builder
ARG SERVICE="rosetta-snarkos"
ARG BRANCH="containerized-deployment"

RUN mkdir -p /app \
  && chown -R nobody:nogroup /app
WORKDIR /app

RUN apk -U upgrade && apk add curl

RUN curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/monadicus/mentat/$BRANCH/$SERVICE/install.sh | sh

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
FROM alpine:edge

ARG SERVICE="rosetta-snarkos"
ENV ADDRESS "0.0.0.0"
ENV ROCKET_ENV "production"

RUN apk update && apk add ca-certificates clang curl gcc git g++ libressl-dev linux-headers openssl

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
