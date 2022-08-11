FROM rust:latest

COPY server /OC_server
WORKDIR /OC_server/app
RUN rustup default nightly
ENTRYPOINT cargo run