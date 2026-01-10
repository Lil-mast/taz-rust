# Multi-stage build for AuraMesh
FROM rust:1.92.0 AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libasound2-dev \
    libudev-dev \
    clang \
    libclang-dev \
    llvm-dev \
    cmake \
    && rm -rf /var/lib/apt/lists/*

# Install rustfmt
RUN rustup component add rustfmt

# Set working directory
WORKDIR /app

# Copy Cargo files (Cargo.lock is optional for workspaces)
COPY Cargo.toml ./
COPY auramesh-core/Cargo.toml ./auramesh-core/
COPY auramesh-cli/Cargo.toml ./auramesh-cli/
COPY auramesh-agents/Cargo.toml ./auramesh-agents/
COPY auramesh-mesh/Cargo.toml ./auramesh-mesh/

# Create empty source directories to cache dependencies
RUN mkdir -p auramesh-core/src auramesh-cli/src auramesh-agents/src auramesh-mesh/src

# Create dummy lib.rs files for caching (but not main.rs)
RUN echo "pub fn load_config() -> Result<(), String> { Ok(()) }" > auramesh-core/src/lib.rs
RUN echo "pub struct PlannerAgent; pub struct InfraAgent;" > auramesh-agents/src/lib.rs
RUN echo "pub async fn start_mesh() -> Result<(), String> { Ok(()) }" > auramesh-mesh/src/lib.rs
RUN echo "fn main() {}" > auramesh-cli/src/main.rs

# Build dependencies
RUN cargo build --release

# Remove dummy source files
RUN rm -rf auramesh-core/src auramesh-cli/src auramesh-agents/src auramesh-mesh/src

# Copy actual source code
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libasound2-dev \
    libudev-dev \
    pkg-config \
    alsa-utils \
    libasound2 \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false auramesh

# Copy binary from builder
COPY --from=builder /app/target/release/auramesh-cli /usr/local/bin/auramesh

# Create data directory
RUN mkdir -p /data && chown auramesh:auramesh /data

# Switch to non-root user
USER auramesh

# Set working directory
WORKDIR /data

# Expose mesh port (libp2p typically uses dynamic ports)
EXPOSE 4001/tcp

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD auramesh --help || exit 1

# Default command
ENTRYPOINT ["auramesh"]
CMD ["--help"]
