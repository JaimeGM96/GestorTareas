FROM rust:alpine3.19

LABEL maintainer="JaimeGM96" version="1.1"

WORKDIR /app/test

COPY Cargo.toml Cargo.lock ./

RUN adduser -D jaime \
    && chown -R jaime:jaime /app/test
USER jaime

RUN mkdir src \
    && echo "// dummy file" > src/ruta.rs \
    && cargo build

ENTRYPOINT [ "cargo", "build", "&&", "cargo", "test" ]
