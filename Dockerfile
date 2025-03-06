FROM rust:slim AS builder

RUN apt-get update && \
    apt-get -y upgrade && \
    apt-get -y install libpq-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY Rocket.toml Rocket.toml

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get -y install libpq-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/klearlink-api /app/klearlink-api
COPY --from=builder /app/Rocket.toml /app/Rocket.toml

RUN useradd -m runner
USER runner

WORKDIR /app
EXPOSE 8000

CMD ["/app/klearlink-api"]
