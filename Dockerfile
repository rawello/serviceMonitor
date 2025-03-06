FROM rust:1.85 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release

FROM debian:bookworm-20250224

RUN apt-get update && apt-get install -y tzdata && rm -rf /var/lib/apt/lists/*

RUN mkdir -p /app/logs

COPY --from=builder /app/target/release/monitor-service /usr/local/bin/monitor-service

WORKDIR /app

EXPOSE 8080

CMD ["monitor-service"]