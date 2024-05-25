FROM rust:1-bookworm as builder

ADD . /app
WORKDIR /app

ENV LIBSQLITE3_FLAGS="SQLITE_MAX_VARIABLE_NUMBER=1000000"
RUN cargo build -p datagen-rs-cli -r --features embedded-plugins

RUN apt-get update && apt-get install -y dos2unix && \
  dos2unix /app/docker/docker-entrypoint.sh && \
  chmod +x /app/docker/docker-entrypoint.sh

FROM debian:bookworm-slim
LABEL org.opencontainers.image.description = "Default datagen image based on debian"

RUN apt-get update && apt-get install -y openssl

COPY --from=builder /app/target/release/datagen /usr/local/bin/datagen
COPY --from=builder /app/docker/docker-entrypoint.sh /usr/local/bin/docker-entrypoint.sh

ENTRYPOINT ["docker-entrypoint.sh"]
