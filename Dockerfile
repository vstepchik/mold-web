FROM rust:latest as builder

RUN apt update -y && apt install -y yarn

WORKDIR /usr/src/mold-web
COPY . .
RUN cargo build --release

# ---

FROM gcr.io/distroless/cc-debian12

COPY --from=builder --chown=nonroot /usr/src/mold-web/target/release/mold-web /opt/mold-web
USER nonroot

ENTRYPOINT ["/opt/mold-web"]
