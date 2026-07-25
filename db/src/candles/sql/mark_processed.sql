INSERT INTO candle_processed_trades (trade_id)
VALUES ($1)
ON CONFLICT (trade_id) DO NOTHING
RETURNING trade_id;
