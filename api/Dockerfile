FROM rust AS build_host
WORKDIR /src

RUN USER=root cargo new --bin api
WORKDIR /src/api

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/api*
RUN cargo build --release

WORKDIR /src

FROM rust:slim

RUN apt-get update
RUN apt-get install -y libpq-dev

WORKDIR /src

COPY ./src/config.yaml ./config.yaml
COPY --from=build_host /src/api/target/release/api ./api

CMD ["./api"]
