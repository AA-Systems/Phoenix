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
RUN cargo build --release -p ws_server

# --- run ---
FROM debian:bookworm-slim
WORKDIR /cex

COPY --from=builder /cex/target/release/ws_server /cex/ws_server
EXPOSE 3002
CMD ["/cex/ws_server"]
