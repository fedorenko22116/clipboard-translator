FROM rust:stretch

RUN apt-get update && apt-get install -y dbus libdbus-glib-1-dev

RUN USER=root cargo new --bin data
WORKDIR /data

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN cargo build --release

ENV OUTPUT_DIR /output

RUN mkdir $OUTPUT_DIR

RUN mv ./target/release/clipboard-translator ${OUTPUT_DIR}/clipboard-translator
