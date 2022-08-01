ARG RUST_VERSION=latest

FROM rust:$RUST_VERSION as build_ui

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk
RUN cargo install wasm-bindgen-cli

RUN USER=root cargo new --bin /app
WORKDIR /app

COPY Cargo.toml .
COPY Cargo.lock .

RUN echo "<!DOCTYPE html><html><head><meta charset=\"utf-8\"/><title>Hello, World!</title></head></html>" > index.html
RUN cat index.html

RUN cargo build --release
RUN trunk build --release

RUN trunk clean
RUN rm index.html
RUN rm -rf ./src

COPY index.html .
COPY ./src ./src
# COPY ./tests ./tests
# COPY ./examples ./examples

RUN rm ./target/release/deps/dfs*
RUN trunk clean

RUN trunk build --release

# FROM rust:$RUST_VERSION as ui

# WORKDIR /app
# COPY --from=build_ui /app /app
# CMD [""]