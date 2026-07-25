SELECT
    market_symbol,
    interval,
    bucket_start,
    open,
    high,
    low,
    close,
    volume,
    trade_count
FROM candles
WHERE market_symbol = $1
  AND interval = $2
ORDER BY bucket_start DESC
LIMIT $3;
