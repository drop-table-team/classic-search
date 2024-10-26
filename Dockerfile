FROM rust:latest AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

COPY --from=builder /app/target/release/classic-search .

ENV ADDRESS=0.0.0.0:8080

EXPOSE 8080

CMD ["./classic-search"]
