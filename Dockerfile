FROM rust:alpine

LABEL maintainer="JaimeGM96" version="1.2"

WORKDIR /app/test

COPY Cargo.toml Cargo.lock /app/

RUN adduser -D jdoc \
    && chown -R jdoc:jdoc /app/
USER jdoc

RUN mkdir -p /app/src \
	&& touch /app/src/main.rs \
	&& cargo update \
	&& rm -rf /app/src \
	&& ln -s /app/test/src /app/src

ENV CARGO_TARGET_DIR=/tmp/cache/
ENTRYPOINT [ "cargo", "test" ]
