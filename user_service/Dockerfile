# https://dev.to/rogertorres/first-steps-with-docker-rust-30oi
FROM rust:1.65.0
COPY ./user_service ./

RUN cargo build --release

CMD ["./target/release/user_service"]
