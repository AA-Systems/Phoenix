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
RUN cargo build --release -p order_engine

# --- run ---
FROM debian:bookworm-slim
WORKDIR /cex

COPY --from=builder /cex/target/release/order_engine /cex/order_engine
CMD ["/cex/order_engine"]
