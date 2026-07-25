use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub mod get_candles_request;

pub const CANDLE_INTERVALS: &[&str] = &["1m", "5m", "15m", "1h"];

pub fn interval_seconds(interval: &str) -> Option<i64> {
    match interval {
        "1m" => Some(60),
        "5m" => Some(300),
        "15m" => Some(900),
        "1h" => Some(3600),
        _ => None,
    }
}

pub fn is_valid_interval(interval: &str) -> bool {
    interval_seconds(interval).is_some()
}

pub fn bucket_start(ts: DateTime<Utc>, interval_secs: i64) -> DateTime<Utc> {
    let epoch = ts.timestamp();
    let start = epoch - epoch.rem_euclid(interval_secs);
    DateTime::from_timestamp(start, 0).unwrap_or(ts)
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Candle {
    pub market_symbol: String,
    pub interval: String,
    pub bucket_start: DateTime<Utc>,
    pub open: i64,
    pub high: i64,
    pub low: i64,
    pub close: i64,
    pub volume: i64,
    pub trade_count: i32,
}
