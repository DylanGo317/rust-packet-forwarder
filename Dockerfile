FROM rust:1.82 AS builder

WORKDIR /forwarder

COPY Cargo.toml Cargo.lock ./

# Create a dummy src so cargo can fetch dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true

# Copy source code
COPY . .
RUN cargo build --release

FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
    libssl-dev ca-certificates iproute2 iputils-ping \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from build stage
COPY --from=builder /forwarder/target/release/packet_forwarder /usr/local/bin/packet_forwarder

CMD ["packet_forwarder"]
