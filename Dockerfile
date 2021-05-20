FROM rust:latest as builder

WORKDIR /work
COPY src src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN cargo install --path .

FROM debian:stretch-slim
ENV DEBIAN_FRONTEND noninteractive
RUN apt-get update
RUN apt-get install -y git
# Copy bin from builder to this new image
COPY --from=builder /usr/local/cargo/bin/onepaas-slack-notify /bin/
