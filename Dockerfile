FROM rust:latest as builder

RUN rustup toolchain install nightly

WORKDIR /build/
COPY build.rs Cargo.toml Cargo.lock ./
COPY data ./data/
COPY src ./src/

RUN pwd && ls -lah && cargo +nightly build --release

# ---

FROM ubuntu:18.04

COPY --from=builder /build/target/release/mold-web /opt

ENTRYPOINT /opt/mold-web
