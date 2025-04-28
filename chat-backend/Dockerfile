FROM rust:slim

WORKDIR /app

# Copy the entire directory
COPY . .

# Install dependencies and build
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    cargo build --release && \
    rm -rf /var/lib/apt/lists/*

# Expose the port
EXPOSE 8080

# Set environment variables
ENV PORT=8080
ENV RUST_BACKTRACE=1

# Run the application
CMD ["./target/release/chat-backend"]