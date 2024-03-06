FROM rust AS build
WORKDIR /src

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk

RUN USER=root cargo new --bin frontend
WORKDIR /src/frontend

COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./index.html ./index.html
COPY ./Trunk.toml ./Trunk.toml
COPY ./assets/js ./assets/js
COPY ./assets/css ./assets/css
COPY ./src ./src

EXPOSE 80

CMD ["trunk", "serve", "--release"]