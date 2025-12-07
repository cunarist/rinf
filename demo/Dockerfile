# Build Rinf example app for web
FROM debian:bookworm-slim AS builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    curl git unzip xz-utils zip \
    build-essential pkg-config libssl-dev

# Install Flutter
ENV FLUTTER_HOME=/opt/flutter
ENV PATH="${FLUTTER_HOME}/bin:${PATH}"
RUN git clone https://github.com/flutter/flutter.git -b stable ${FLUTTER_HOME}

# Install Rust
ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ENV PATH="/usr/local/cargo/bin:${PATH}"
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Set working directory
WORKDIR /workspace

# Copy project files
COPY flutter_package/ flutter_package/
COPY rust_crate/ rust_crate/
COPY rust_crate_proc/ rust_crate_proc/
COPY rust_crate_cli/ rust_crate_cli/
COPY Cargo.toml Cargo.toml

# Install Rinf CLI from local source
RUN cargo install --path rust_crate_cli

# Build web app
WORKDIR /workspace/flutter_package/example
RUN rinf gen
RUN rinf wasm --release
RUN flutter build web --release

# Serve the app
FROM python:3.13-slim

WORKDIR /app

# Copy built web app
COPY --from=builder /workspace/flutter_package/example/build/web .

# Copy server script
COPY demo/server.py .

EXPOSE 80

CMD ["python", "server.py"]
