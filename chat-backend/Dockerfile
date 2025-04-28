FROM rust:slim

WORKDIR /app

# Copy everything
COPY . .

# Install dependencies, build, and clean up
RUN apt-get update && \
    apt-get install -y --no-install-recommends pkg-config libssl-dev && \
    cargo build --release && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Set environment variables
ENV PORT=8080

# Expose the port
EXPOSE 8080

# Run the application
CMD ["./target/release/chat-backend"]