# --- build ---
FROM rust:1.96-slim-bookworm AS builder
WORKDIR /cex
COPY Cargo.toml Cargo.lock ./
COPY common ./common
COPY api ./api
COPY db ./db
COPY types ./types
COPY order_engine ./order_engine
COPY ws_server ./ws_server
COPY candle_builder ./candle_builder
COPY demo_bot ./demo_bot
RUN cargo build --release -p api

# --- run ---
FROM debian:bookworm-slim
WORKDIR /cex

COPY --from=builder /cex/target/release/api /cex/api
EXPOSE 3000
CMD ["/cex/api"]
