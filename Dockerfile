FROM rust:slim

RUN apt-get update && \
    apt-get -y upgrade && \
    apt-get -y install libpq-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

RUN rustup default nightly \
    && rustup update

RUN useradd -m runner

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY src ./src
COPY Rocket.toml Rocket.toml
COPY .env .env

RUN cargo fetch

RUN cargo build --release

RUN chown -R runner:runner /app

USER runner

EXPOSE 8000

CMD ["/app/target/release/klearlink"]
