FROM ubuntu:18.04

COPY target/release/mold-web /opt

ENTRYPOINT /opt/mold-web
