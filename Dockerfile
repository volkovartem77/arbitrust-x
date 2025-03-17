# Use the Rust official image as a parent image
FROM rust:1.83.0-alpine3.20 AS builder
RUN apk --no-cache add ca-certificates musl-dev build-base pkgconfig libressl libressl-dev

# Set the working directory in the docker image
WORKDIR /usr/src/

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy your source code into the image
COPY ./ .

# Build the binaries
RUN cargo build --release

# Start a new build stage
FROM alpine:3.20
RUN apk add --no-cache libressl libressl-dev

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/target/release/arbitrust-x /usr/local/bin/arbitrust-x

# Copy the JSON file
COPY --from=builder /usr/src/src/symbols_info.json /src/symbols_info.json

# Set the entry point
ENTRYPOINT ["/usr/local/bin/arbitrust-x"]

