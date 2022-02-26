# Build Snarkos Mentat
FROM ubuntu:20.04 as snarkos-mentat-builder

RUN mkdir -p /app \
  && chown -R nobody:nogroup /app
WORKDIR /app

# Source: https://github.com/bitcoin/bitcoin/blob/master/doc/build-unix.md#ubuntu--debian
ARG DEBIAN_FRONTEND=noninteractive
ENV TZ Etc/UTC

RUN apt-get update && apt-get install -y build-essential curl git

# Install Rust stable
RUN curl --proto '=https' --tlsv1.3 -sSf https://sh.rustup.rs -sSf | sh -s -- -y \
    && echo 'source $HOME/.cargo/env' >> $HOME/.bashrc

RUN git clone git@github.com:monadicus/mentat.git \
    && cd mentat \
    && cargo build --release

# Build Rosetta Server Components
FROM ubuntu:20.04 as rosetta-builder

RUN mkdir -p /app \
  && chown -R nobody:nogroup /app
WORKDIR /app

RUN apt-get update && apt-get install -y curl make gcc g++
ENV GOLANG_VERSION 1.17.7
ENV GOLANG_DOWNLOAD_SHA256 9a58494e8da722c3aef248c9227b0e9c528c7318309827780f16220998180a0d
ENV GOLANG_DOWNLOAD_URL https://golang.org/dl/go$GOLANG_VERSION.linux-amd64.tar.gz

RUN curl -fsSL "$GOLANG_DOWNLOAD_URL" -o golang.tar.gz \
  && echo "$GOLANG_DOWNLOAD_SHA256  golang.tar.gz" | sha256sum -c - \
  && tar -C /usr/local -xzf golang.tar.gz \
  && rm golang.tar.gz

ENV GOPATH /go
ENV PATH $GOPATH/bin:/usr/local/go/bin:$PATH
RUN mkdir -p "$GOPATH/src" "$GOPATH/bin" && chmod -R 777 "$GOPATH"

# Use native remote build context to build in any directory
COPY . src 
RUN cd src \
  && go build \
  && cd .. \
  && mv src/rosetta-snarkos /app/rosetta-snarkos \
  && mv src/assets/* /app \
  && rm -rf src 

## Build Final Image
FROM ubuntu:20.04

RUN apt-get update && \
  apt-get install --no-install-recommends -y libevent-dev libboost-system-dev libboost-filesystem-dev libboost-test-dev libboost-thread-dev && \
    apt-get clean && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

RUN mkdir -p /app \
  && chown -R nobody:nogroup /app \
  && mkdir -p /data \
    && chown -R nobody:nogroup /data

WORKDIR /app

# Copy binary from snarkos-mentat-builder
COPY --from=snarkos-mentat-builder /app/snarkos-mentat /app/snarkos-mentat

# Copy binary from rosetta-builder
COPY --from=rosetta-builder /app/* /app/

# Set permissions for everything added to /app
RUN chmod -R 755 /app/*

CMD ["/app/rosetta-bitcoin"]
