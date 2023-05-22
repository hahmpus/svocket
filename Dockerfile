FROM rust:1.69 as rust-builder
WORKDIR /usr/src/calorie
COPY rocket/ .
RUN cargo build --release

FROM debian:bullseye
RUN apt-get update && apt-get -y install libssl1.1 openssl ca-certificates
COPY --from=rust-builder /usr/src/calorie/target/release/calorie /app/calorie

WORKDIR /app

CMD ["./calorie"]