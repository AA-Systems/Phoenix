INSERT INTO candles (
    market_symbol,
    interval,
    bucket_start,
    open,
    high,
    low,
    close,
    volume,
    trade_count
) VALUES ($1, $2, $3, $4, $4, $4, $4, $5, 1)
ON CONFLICT (market_symbol, interval, bucket_start) DO UPDATE SET
    high = GREATEST(candles.high, EXCLUDED.high),
    low = LEAST(candles.low, EXCLUDED.low),
    close = EXCLUDED.close,
    volume = candles.volume + EXCLUDED.volume,
    trade_count = candles.trade_count + 1;
