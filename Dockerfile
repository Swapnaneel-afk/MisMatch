FROM rust:slim as builder

WORKDIR /app

# Copy only the Cargo.toml and Cargo.lock first to leverage Docker caching
COPY chat-backend/Cargo.toml chat-backend/Cargo.lock ./

# Create a dummy src/main.rs to build dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy the actual source code
COPY chat-backend/src ./src

# Rebuild with actual source code
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

WORKDIR /app

# Install only runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/chat-backend /app/chat-backend

ENV PORT=8080

EXPOSE 8080

CMD ["./chat-backend"] 