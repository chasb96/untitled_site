FROM rust

RUN apt-get install -y libpq-dev
RUN cargo install diesel_cli

WORKDIR /migrations

COPY ./migrations .

CMD [ "diesel", "migration", "run" ]