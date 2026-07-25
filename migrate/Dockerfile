FROM rust:1.96-slim-bookworm AS builder
RUN cargo install sqlx-cli --locked --no-default-features --features postgres,rustls

FROM debian:bookworm-slim
RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates \
  && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx
COPY migrations /migrations
WORKDIR /
CMD ["sqlx", "migrate", "run", "--source", "/migrations"]
