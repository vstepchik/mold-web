FROM rust as builder

RUN rustup toolchain install nightly

WORKDIR /build/
COPY build.rs Cargo.toml Cargo.lock ./
COPY data ./data/
COPY src ./src/

RUN cargo +nightly build --release
# todo: UPX the binary

# ---

FROM ubuntu:18.04

RUN apt-get update && apt-get install libssl1.1 && apt-get clean
COPY --from=builder /build/target/release/mold-web /opt

ENTRYPOINT /opt/mold-web
