FROM rust:slim as build

COPY ./ ./

RUN apt-get update && apt-get -y install libpq-dev && \
    rustup toolchain install nightly && \
    rustup override set nightly && \
    rustup component add cargo && \
    cargo build --release

RUN mkdir -p /build-out/

RUN ls -la && cp target/release/rnotes_cli target/release/rnotes_server /build-out/

FROM ubuntu:18.04

ENV DATABASE_URL="postgres://postgres:postgres@localhost/postgres"
ENV DATABASE_SCHEMA="rnotes"
ENV ROCKET_ADDRESS="localhost"
ENV ROCKET_PORT="8080"
ENV JWT_SECRET_KEY="some_secret_key"
ENV JWT_SESSION_TIME="3600"

RUN apt-get update && apt-get -y install ca-certificates libssl-dev libpq-dev && rm -rf /var/lib/apt/lists/*

COPY --from=build /build-out/rnotes_* /

CMD /rnotes_server