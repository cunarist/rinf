# Build Flutter web app
FROM debian:bookworm-slim AS flutter-builder

# Install dependencies
RUN apt-get update
RUN apt-get install -y \
    curl git unzip xz-utils zip libglu1-mesa \
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

# Copy only essential files for building
COPY flutter_package/ flutter_package/
COPY rust_crate/ rust_crate/
COPY rust_crate_proc/ rust_crate_proc/
COPY rust_crate_cli/ rust_crate_cli/
COPY Cargo.toml Cargo.toml

# Install Rinf CLI from local source
RUN cargo install --path rust_crate_cli

# Build Flutter web app
WORKDIR /workspace/flutter_package/example
RUN rinf gen
RUN rinf wasm --release
RUN flutter build web --release --base-href="/_static/web_app/"

# Build documentation and serve
FROM python:3.13-slim

# Set working directory
WORKDIR /app

# Copy documentation files
COPY documentation/ .

# Copy built web app from flutter-builder stage
COPY --from=flutter-builder \
    /workspace/flutter_package/example/build/web \
    source/_static/web_app/

# Install dependencies
RUN pip install --no-cache-dir .

# Build documentation
RUN sphinx-build -b dirhtml source dist/dirhtml

# Expose port 80
EXPOSE 80

# Run the server
CMD ["python", "server.py"]
