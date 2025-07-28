# Build stage
FROM rust:1.88.0-alpine AS builder

# Install dependencies for building Rust code
RUN apk add --no-cache clang lld musl-dev perl openssl-libs-static pkgconf openssl-dev  make ca-certificates

# Set working dir
WORKDIR /usr/src/cookie-classifier

# Copy manifests
COPY Cargo.toml ./
COPY Cargo.lock* ./

# Copy the source code to enable dependency fetching
COPY src ./src

# Pre-fetch dependencies to cache them
RUN cargo fetch

# Build the project in release mode
RUN cargo install --path . && cargo build --release

# Runtime stage
FROM alpine:3.18

# Install runtime dependencies
RUN apk add --no-cache libssl3 ca-certificates

# Copy the compiled binary from the build stage
COPY --from=builder /usr/src/cookie-classifier/target/release/cookie-classifier /usr/local/bin/cookie-classifier

# Expose the required port
EXPOSE 3000

# Set the entrypoint to the application
ENTRYPOINT [ "/usr/local/bin/cookie-classifier" ]
CMD [""]