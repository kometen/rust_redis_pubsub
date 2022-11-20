# https://dev.to/rogertorres/first-steps-with-docker-rust-30oi
FROM rust:1.65.0
COPY ./web_service ./

RUN cargo build --release

CMD ["./target/release/web_service"]
