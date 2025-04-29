FROM rust:slim

WORKDIR /app

# Copy everything
COPY . .

# Build with release profile
RUN cargo build --release

# Expose port
EXPOSE 8080

# Set environment variables 
ENV PORT=8080

# Run the application
CMD ["./target/release/chat-backend"]