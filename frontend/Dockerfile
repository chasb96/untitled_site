FROM rust AS build_host
WORKDIR /app

COPY ./host .

RUN cargo build --release

FROM node
WORKDIR /host

COPY --from=build_host /app/target/release/host ./host

WORKDIR /host/app

RUN npm i -g sass

COPY ./package.json ./

RUN npm i

COPY . .

RUN sass ./src/scss/custom.scss ./src/assets/css/custom.css
RUN npm run build

EXPOSE 80

WORKDIR /host
CMD [ "./host" ]