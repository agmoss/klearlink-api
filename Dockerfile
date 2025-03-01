FROM rust:latest

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

RUN apt-get update && \
  apt-get -y upgrade && \
  apt-get -y install libpq-dev

WORKDIR /app

COPY . /app/

COPY .env /app/.env

RUN cargo build --release

EXPOSE 8000

ENTRYPOINT ["/bin/bash", "-c", "cargo run --release"]
