# --- build ---
FROM rust:1.96-slim-bookworm AS builder
WORKDIR /cex
COPY Cargo.toml Cargo.lock ./
COPY common ./common
COPY api ./api
RUN cargo build --release -p api

# --- run ---
FROM debian:bookworm-slim
WORKDIR /cex

COPY --from=builder /cex/target/release/api /cex/api
EXPOSE 3000
CMD ["/cex/api"]
