FROM rust:latest

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/chat-backend"]