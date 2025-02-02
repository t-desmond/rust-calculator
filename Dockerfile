FROM rust:latest AS build

WORKDIR /app

COPY . .

RUN cargo build --release

FROM alpine:latest

WORKDIR /app

COPY --from=build /app/target/release/calculator /app/calculator

CMD [ "/app/calculator" ]