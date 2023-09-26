FROM rust:1-bookworm as builder

ADD . /app
WORKDIR /app

RUN cargo build -p datagen-rs-cli -r
ENV LIBSQLITE3_FLAGS="SQLITE_MAX_VARIABLE_NUMBER=1000000"
RUN cargo build -p datagen-rs-openaddresses-plugin -r -F sqlite,log

FROM debian:bookworm-slim

COPY --from=builder /app/target/release/datagen /usr/local/bin/datagen
COPY --from=builder /app/target/release/libopenaddresses_plugin.so /usr/local/lib/libopenaddresses_plugin.so

ENV DATAGEN_PLUGIN_DIR=/usr/local/lib
