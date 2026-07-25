use sqlx::PgPool;
use types::candle::{CANDLE_INTERVALS, bucket_start, interval_seconds};
use types::trade::TradeView;
use uuid::Uuid;

pub async fn apply_trade(pool: &PgPool, trade: &TradeView) -> Result<bool, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let claimed: Option<Uuid> = sqlx::query_scalar(include_str!("sql/mark_processed.sql"))
        .bind(trade.id)
        .fetch_optional(&mut *tx)
        .await?;

    if claimed.is_none() {
        tx.commit().await?;
        return Ok(false);
    }

    let symbol = trade.market_symbol.trim().to_uppercase();
    for interval in CANDLE_INTERVALS {
        let Some(secs) = interval_seconds(interval) else {
            continue;
        };
        let start = bucket_start(trade.created_at, secs);
        sqlx::query(include_str!("sql/upsert.sql"))
            .bind(&symbol)
            .bind(*interval)
            .bind(start)
            .bind(trade.price)
            .bind(trade.quantity)
            .execute(&mut *tx)
            .await?;
    }

    tx.commit().await?;
    Ok(true)
}
