CREATE TABLE candles (
    market_symbol TEXT NOT NULL,
    interval TEXT NOT NULL,
    bucket_start TIMESTAMPTZ NOT NULL,
    open BIGINT NOT NULL,
    high BIGINT NOT NULL,
    low BIGINT NOT NULL,
    close BIGINT NOT NULL,
    volume BIGINT NOT NULL DEFAULT 0,
    trade_count INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (market_symbol, interval, bucket_start),
    CONSTRAINT candles_interval_check CHECK (interval IN ('1m', '5m', '15m', '1h')),
    CONSTRAINT candles_ohlc_check CHECK (
        high >= open
        AND high >= close
        AND high >= low
        AND low <= open
        AND low <= close
        AND volume >= 0
        AND trade_count >= 0
    )
);

CREATE INDEX candles_lookup_idx
    ON candles (market_symbol, interval, bucket_start DESC);

-- Idempotency for candle_builder: skip trades already applied after a crash/retry.
CREATE TABLE candle_processed_trades (
    trade_id UUID PRIMARY KEY,
    processed_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
