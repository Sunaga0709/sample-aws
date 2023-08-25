FROM rust:1.70.0-slim-bullseye AS common

WORKDIR /app

ENV CARGO_BUILD_JOBS=2
ENV RUST_BACKTRACE=1
ENV TZ=Asia/Tokyo

RUN apt update -y && apt upgrade -y
RUN apt install -y pkg-config libssl-dev
RUN apt install -y default-mysql-client default-libmysqlclient-dev
RUN apt install -y protobuf-compiler

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN mkdir src/
RUN echo "fn main() {}" > src/main.rs
RUN cargo build
RUN rm -rf ./target/debug/.fingerprint/server-*
COPY ./src ./src

FROM common AS dev
RUN rustup component add rustfmt
RUN rustup component add clippy
RUN cargo install sqlx-cli

# FROM common as builder
# RUN cargo build --release

# FROM debian:buster-slim as prod
# COPY --from=builder /app/target/release/server /usr/local/bin/server
# CMD ["server"]