FROM rust:latest AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/imoost /usr/local/bin/imoost

COPY .env /app/.env

EXPOSE 8080

CMD ["imoost"]
