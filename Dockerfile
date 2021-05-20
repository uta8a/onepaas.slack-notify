FROM rust:latest

WORKDIR /work
COPY src src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN cargo install --path .
