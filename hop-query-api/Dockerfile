# Builder stage
FROM rust:1.80-slim-bullseye AS builder

# Install necessary build dependencies including ca-certificates
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Update certificates
RUN update-ca-certificates

# Set environment variables to help with SSL issues
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR=/etc/ssl/certs

# Create a new empty shell project
RUN USER=root cargo new --bin app
WORKDIR /app

# Copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./.sqlx ./.sqlx

RUN echo -n | openssl s_client -showcerts -connect crates.io:443 -servername crates.io > /tmp/crates_io_cert.pem
RUN cat /tmp/crates_io_cert.pem >> /etc/ssl/certs/ca-certificates.crt

# This build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy your source tree
COPY ./src ./src

# Build for release
RUN rm ./target/release/deps/hop_query_api*
RUN cargo build --release

# Final stage
FROM debian:bullseye-slim AS final

# Install OpenSSL and ca-certificates - required at runtime
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl1.1 \
    && rm -rf /var/lib/apt/lists/*

# Update certificates
RUN update-ca-certificates

# Create a non-root user
RUN useradd -ms /bin/bash appuser

# Copy the build artifact from the builder stage
COPY --from=builder /app/target/release/hop-query-api /usr/local/bin/

# Copy the .env file (make sure this is in your .dockerignore to avoid accidental commits)
COPY .env /.env

# Set the user to run the application
USER appuser

# Run the binary
CMD ["hop-query-api"]