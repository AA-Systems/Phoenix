use std::collections::{HashMap, HashSet};
use std::time::Duration;

use anyhow::{Context, Result, bail};
use rand::Rng;
use tracing::{info, warn};
use types::markets::{Market, MarketStatus};
use types::order::{OpenOrderView, OrderType};

use crate::api::ApiClient;
use crate::config::BotConfig;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct LevelKey {
    side: Side,
    price: i64,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Side {
    Bid,
    Ask,
}

impl Side {
    fn order_type(self) -> OrderType {
        match self {
            Side::Bid => OrderType::Buy,
            Side::Ask => OrderType::Sell,
        }
    }
}

pub struct MarketRuntime {
    pub market: Market,
    pub base_decimals: i32,
    pub mid: i64,
    pub seeded: bool,
}

pub async fn bootstrap(
    config: &BotConfig,
    maker: &ApiClient,
    taker: &ApiClient,
) -> Result<Vec<MarketRuntime>> {
    maker.demo_credit().await.context("maker demo credit")?;
    taker.demo_credit().await.context("taker demo credit")?;
    tokio::time::sleep(Duration::from_secs(2)).await;

    let markets = maker.list_markets().await?;
    let assets = maker.list_assets().await?;
    let decimals: HashMap<String, i32> = assets
        .into_iter()
        .map(|a| (a.symbol.trim().to_uppercase(), a.decimals))
        .collect();

    let wanted = config.market_list();
    let mut runtimes = Vec::new();
    for symbol in wanted {
        let market = markets
            .iter()
            .find(|m| m.symbol.eq_ignore_ascii_case(&symbol))
            .cloned()
            .with_context(|| format!("market {symbol} not found — run seed?"))?;
        if market.status != MarketStatus::Trading {
            bail!("market {} is not trading", market.symbol);
        }
        let base = market.symbol.split('_').next().unwrap_or("").to_uppercase();
        let base_decimals = *decimals
            .get(&base)
            .with_context(|| format!("missing decimals for {base}"))?;
        let mid = align_price(default_mid(&market.symbol), market.price_tick_size);
        info!(%market.symbol, mid, "bootstrapped market");
        runtimes.push(MarketRuntime {
            market,
            base_decimals,
            mid,
            seeded: false,
        });
    }
    if runtimes.is_empty() {
        bail!("no markets configured");
    }

    // Seed full ladders once so the UI never opens on an empty book.
    let delay = config.request_delay();
    for runtime in runtimes.iter_mut() {
        seed_book(config, maker, runtime, delay).await?;
        runtime.seeded = true;
        tokio::time::sleep(Duration::from_millis(300)).await;
    }

    Ok(runtimes)
}

pub async fn run_cycle(
    config: &BotConfig,
    maker: &ApiClient,
    taker: &ApiClient,
    runtimes: &mut [MarketRuntime],
) -> Result<()> {
    let delay = config.request_delay();
    let mut rng = rand::rng();

    // Round-robin markets with small mutations so books stay continuous.
    for runtime in runtimes.iter_mut() {
        // Most cycles: tiny drift or none — feels like a live tape, not a wipe.
        if rng.random_bool(0.65) {
            walk_mid(runtime, config.walk_ticks, &mut rng);
        }

        sync_book_incremental(config, maker, runtime, delay, &mut rng).await?;

        if config.trades > 0 && rng.random_bool(0.7) {
            fire_trades(config, taker, runtime, delay, &mut rng).await?;
            // Immediately top up whatever the taker ate near the touch.
            sync_book_incremental(config, maker, runtime, delay, &mut rng).await?;
        }
    }
    Ok(())
}

async fn seed_book(
    config: &BotConfig,
    maker: &ApiClient,
    runtime: &MarketRuntime,
    delay: Duration,
) -> Result<()> {
    let symbol = &runtime.market.symbol;
    let targets = target_ladder(config, runtime);
    let qty = order_qty(&runtime.market, runtime.mid, runtime.base_decimals);

    // Place nearest levels first so the touch fills early.
    let mut ordered = targets;
    ordered.sort_by_key(|level| distance_from_mid(level.price, runtime.mid));

    for level in ordered {
        if let Err(err) = maker
            .create_order(symbol, level.side.order_type(), level.price, qty)
            .await
        {
            warn!(%err, %symbol, price = level.price, "seed place failed");
        }
        tokio::time::sleep(delay).await;
    }

    info!(%symbol, mid = runtime.mid, levels = targets_len(config), "seeded book");
    Ok(())
}

async fn sync_book_incremental(
    config: &BotConfig,
    maker: &ApiClient,
    runtime: &MarketRuntime,
    delay: Duration,
    rng: &mut impl Rng,
) -> Result<()> {
    let symbol = &runtime.market.symbol;
    let open = maker.open_orders().await?;
    let mine: Vec<OpenOrderView> = open
        .into_iter()
        .filter(|o| o.market_symbol.eq_ignore_ascii_case(symbol))
        .collect();

    let targets = target_ladder(config, runtime);
    let target_set: HashSet<LevelKey> = targets.iter().copied().collect();

    // Map occupied target keys → one resting order id (extras get cancelled).
    let mut occupied: HashMap<LevelKey, String> = HashMap::new();
    let mut cancel_queue: Vec<(i64, String)> = Vec::new(); // (priority, id) higher first

    for order in &mine {
        let side = match order.order_type {
            OrderType::Buy => Side::Bid,
            OrderType::Sell => Side::Ask,
        };
        let key = LevelKey {
            side,
            price: order.price,
        };

        // Crossed / invalid relative to mid → cancel urgently.
        let crossed = match side {
            Side::Bid => order.price >= runtime.mid,
            Side::Ask => order.price <= runtime.mid,
        };
        if crossed || !target_set.contains(&key) {
            let priority =
                distance_from_mid(order.price, runtime.mid) + if crossed { 1_000_000 } else { 0 };
            cancel_queue.push((priority, order.id.clone()));
            continue;
        }

        if let Some(existing) = occupied.insert(key, order.id.clone()) {
            // Duplicate at same level — cancel the older-looking one.
            cancel_queue.push((distance_from_mid(order.price, runtime.mid), existing));
        }
    }

    cancel_queue.sort_by(|a, b| b.0.cmp(&a.0));

    let mut missing: Vec<LevelKey> = targets
        .into_iter()
        .filter(|level| !occupied.contains_key(level))
        .collect();
    // Fill the touch first.
    missing.sort_by_key(|level| distance_from_mid(level.price, runtime.mid));

    let budget = config.max_actions.max(1) as usize;
    let mut used = 0usize;
    let mut cancelled = 0usize;
    let mut placed = 0usize;

    // Never wipe the book: keep at least half the target depth resting.
    let min_keep = (targets_len(config) / 2).max(4);
    let max_cancels = cancel_queue
        .len()
        .min(budget)
        .min(mine.len().saturating_sub(min_keep.min(mine.len())));

    for (_priority, order_id) in cancel_queue.into_iter().take(max_cancels) {
        if let Err(err) = maker.cancel_order(&order_id).await {
            warn!(%err, %order_id, "cancel failed");
        } else {
            cancelled += 1;
        }
        used += 1;
        tokio::time::sleep(delay).await;
        if used >= budget {
            break;
        }
    }

    let place_budget = budget.saturating_sub(used);
    let qty_base = order_qty(&runtime.market, runtime.mid, runtime.base_decimals);

    for level in missing.into_iter().take(place_budget) {
        // Mild size jitter so the book doesn't look synthetic.
        let mult = rng.random_range(1..=3);
        let step = runtime.market.quantity_step_size.max(1);
        let qty = ((qty_base.saturating_mul(mult)) / step) * step;
        let qty = qty.max(step);

        if let Err(err) = maker
            .create_order(symbol, level.side.order_type(), level.price, qty)
            .await
        {
            warn!(%err, %symbol, price = level.price, "place failed");
        } else {
            placed += 1;
        }
        tokio::time::sleep(delay).await;
    }

    if cancelled > 0 || placed > 0 {
        info!(
            %symbol,
            mid = runtime.mid,
            cancelled,
            placed,
            resting = mine.len().saturating_sub(cancelled) + placed,
            "book synced"
        );
    }
    Ok(())
}

async fn fire_trades(
    config: &BotConfig,
    taker: &ApiClient,
    runtime: &MarketRuntime,
    delay: Duration,
    rng: &mut impl Rng,
) -> Result<()> {
    let symbol = &runtime.market.symbol;
    let tick = runtime.market.price_tick_size;
    let spread = config.spread_ticks.max(1) as i64;
    let qty_base = order_qty(&runtime.market, runtime.mid, runtime.base_decimals);
    let step = runtime.market.quantity_step_size.max(1);

    let n = config.trades.max(1);
    for i in 0..n {
        // Mostly hit the touch; occasionally reach 1 level deeper.
        let depth = if rng.random_bool(0.25) { 1 } else { 0 };
        let buy = rng.random_bool(0.5);
        let (side, price) = if buy {
            (
                OrderType::Buy,
                align_price(runtime.mid + (spread + depth) * tick, tick),
            )
        } else {
            (
                OrderType::Sell,
                align_price(runtime.mid - (spread + depth) * tick, tick),
            )
        };
        if price < tick {
            continue;
        }

        // Take a slice of a level, not the whole ladder.
        let mult = rng.random_range(1..=2);
        let qty = ((qty_base.saturating_mul(mult)) / step) * step;
        let qty = qty.max(step);

        if let Err(err) = taker.create_order(symbol, side, price, qty).await {
            warn!(%err, %symbol, ?side, price, "taker trade failed");
        } else {
            info!(%symbol, ?side, price, qty, trade = i + 1, "tape print");
        }
        tokio::time::sleep(delay + Duration::from_millis(rng.random_range(80..220))).await;
    }
    Ok(())
}

fn target_ladder(config: &BotConfig, runtime: &MarketRuntime) -> Vec<LevelKey> {
    let levels = (config.orders.max(2) / 2) as i64;
    let spread = config.spread_ticks.max(1) as i64;
    let tick = runtime.market.price_tick_size;
    let mut out = Vec::with_capacity((levels * 2) as usize);

    for i in 1..=levels {
        let bid = align_price(runtime.mid - (spread + i - 1) * tick, tick);
        let ask = align_price(runtime.mid + (spread + i - 1) * tick, tick);
        if bid >= tick && bid < runtime.mid {
            out.push(LevelKey {
                side: Side::Bid,
                price: bid,
            });
        }
        if ask > runtime.mid {
            out.push(LevelKey {
                side: Side::Ask,
                price: ask,
            });
        }
    }
    out
}

fn targets_len(config: &BotConfig) -> usize {
    config.orders.max(2) as usize
}

fn walk_mid(runtime: &mut MarketRuntime, walk_ticks: u32, rng: &mut impl Rng) {
    let tick = runtime.market.price_tick_size;
    let max = walk_ticks.max(1) as i64;
    // Bias toward ±1 tick even when walk_ticks > 1.
    let delta = if max == 1 {
        rng.random_range(-1..=1)
    } else if rng.random_bool(0.7) {
        rng.random_range(-1..=1)
    } else {
        rng.random_range(-max..=max)
    };
    let next = runtime.mid + delta * tick;
    runtime.mid = align_price(next.max(tick * 10), tick);
}

fn distance_from_mid(price: i64, mid: i64) -> i64 {
    (price - mid).abs()
}

fn default_mid(symbol: &str) -> i64 {
    match symbol {
        "BTC_USDC" => 65_000_000_000,
        "ETH_USDC" => 3_500_000_000,
        "SOL_USDC" => 150_000_000,
        _ => 100_000_000,
    }
}

fn align_price(price: i64, tick: i64) -> i64 {
    if tick <= 0 {
        return price.max(1);
    }
    (price / tick) * tick
}

fn order_qty(market: &Market, price: i64, base_decimals: i32) -> i64 {
    let step = market.quantity_step_size.max(1);
    let scale = 10i64.saturating_pow(base_decimals.max(0) as u32);
    let mut qty = market.min_order_quantity.max(step);
    qty = (qty / step) * step;
    if qty == 0 {
        qty = step;
    }

    for _ in 0..10_000 {
        let notional = price.saturating_mul(qty) / scale.max(1);
        if notional >= market.min_order_notional {
            return qty;
        }
        qty = qty.saturating_add(step);
    }
    qty
}
