export type CandleInterval = "1m" | "5m" | "15m" | "1h";

export type Candle = {
  market_symbol: string;
  interval: string;
  bucket_start: string;
  open: number;
  high: number;
  low: number;
  close: number;
  volume: number;
  trade_count: number;
};

export const CANDLE_INTERVALS: CandleInterval[] = ["1m", "5m", "15m", "1h"];

export function intervalSeconds(interval: CandleInterval): number {
  switch (interval) {
    case "1m":
      return 60;
    case "5m":
      return 300;
    case "15m":
      return 900;
    case "1h":
      return 3600;
  }
}

export function bucketStartUnix(
  isoTime: string,
  interval: CandleInterval,
): number {
  const secs = intervalSeconds(interval);
  const epoch = Math.floor(new Date(isoTime).getTime() / 1000);
  return epoch - (epoch % secs);
}

export function applyTradeToCandles(
  candles: Candle[],
  trade: { id: string; price: number; quantity: number; created_at: string },
  marketSymbol: string,
  interval: CandleInterval,
): Candle[] {
  const start = bucketStartUnix(trade.created_at, interval);
  const startIso = new Date(start * 1000).toISOString();
  const next = [...candles];
  const last = next[next.length - 1];

  if (
    last &&
    Math.floor(new Date(last.bucket_start).getTime() / 1000) === start
  ) {
    next[next.length - 1] = {
      ...last,
      high: Math.max(last.high, trade.price),
      low: Math.min(last.low, trade.price),
      close: trade.price,
      volume: last.volume + trade.quantity,
      trade_count: last.trade_count + 1,
    };
    return next;
  }

  next.push({
    market_symbol: marketSymbol,
    interval,
    bucket_start: startIso,
    open: trade.price,
    high: trade.price,
    low: trade.price,
    close: trade.price,
    volume: trade.quantity,
    trade_count: 1,
  });
  return next.slice(-500);
}
