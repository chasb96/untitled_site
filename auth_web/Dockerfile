FROM rust AS build_host
WORKDIR /src

RUN USER=root cargo new --bin auth_web
WORKDIR /src/auth_web

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/auth_web*
RUN cargo build --release

WORKDIR /src

FROM rust:slim

RUN apt-get update
RUN apt-get install -y libpq-dev

HEALTHCHECK CMD curl --fail http://localhost/health || exit 1

WORKDIR /src

COPY --from=build_host /src/auth_web/target/release/auth_web ./auth_web

CMD ["./auth_web"]
