FROM rust:1.50.0-slim

WORKDIR /src/app

RUN apt update && apt upgrade -y
RUN apt install -y \
    git \
    gcc \
    nano \
    openssl \
    libssl-dev

RUN rustup default nightly
RUN cargo install cargo-watch
