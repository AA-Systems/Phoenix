use std::time::Duration;

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString},
};
use dotenv::dotenv;
use rand_core::OsRng;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};
use uuid::Uuid;

const SEED_EMAIL: &str = "akshatarora130@gmail.com";
const SEED_PASSWORD: &str = "akshat1!2!3arora";
const SEED_NAME: &str = "Akshat Arora";

struct AssetSpec {
    symbol: &'static str,
    name: &'static str,
    decimals: i32,
}

struct MarketSpec {
    symbol: &'static str,
    name: &'static str,
    base: &'static str,
    quote: &'static str,
    price_tick_size: i64,
    quantity_step_size: i64,
    min_order_quantity: i64,
    min_order_notional: i64,
}

const ASSETS: &[AssetSpec] = &[
    AssetSpec {
        symbol: "USDC",
        name: "USD Coin",
        decimals: 6,
    },
    AssetSpec {
        symbol: "USDT",
        name: "Tether",
        decimals: 6,
    },
    AssetSpec {
        symbol: "BTC",
        name: "Bitcoin",
        decimals: 8,
    },
    AssetSpec {
        symbol: "ETH",
        name: "Ethereum",
        decimals: 8,
    },
    AssetSpec {
        symbol: "SOL",
        name: "Solana",
        decimals: 9,
    },
    AssetSpec {
        symbol: "HYPE",
        name: "Hyperliquid",
        decimals: 6,
    },
    AssetSpec {
        symbol: "DOGE",
        name: "Dogecoin",
        decimals: 8,
    },
    AssetSpec {
        symbol: "LINK",
        name: "Chainlink",
        decimals: 8,
    },
    AssetSpec {
        symbol: "AVAX",
        name: "Avalanche",
        decimals: 8,
    },
    AssetSpec {
        symbol: "SUI",
        name: "Sui",
        decimals: 9,
    },
];

const MARKETS: &[MarketSpec] = &[
    MarketSpec {
        symbol: "BTC_USDC",
        name: "BTC / USDC",
        base: "BTC",
        quote: "USDC",
        price_tick_size: 10_000,
        quantity_step_size: 1_000,
        min_order_quantity: 10_000,
        min_order_notional: 10_000_000,
    },
    MarketSpec {
        symbol: "ETH_USDC",
        name: "ETH / USDC",
        base: "ETH",
        quote: "USDC",
        price_tick_size: 10_000,
        quantity_step_size: 10_000,
        min_order_quantity: 100_000,
        min_order_notional: 10_000_000,
    },
    MarketSpec {
        symbol: "SOL_USDC",
        name: "SOL / USDC",
        base: "SOL",
        quote: "USDC",
        price_tick_size: 10_000,
        quantity_step_size: 1_000_000,
        min_order_quantity: 10_000_000,
        min_order_notional: 5_000_000,
    },
    MarketSpec {
        symbol: "HYPE_USDC",
        name: "HYPE / USDC",
        base: "HYPE",
        quote: "USDC",
        price_tick_size: 1_000,
        quantity_step_size: 100_000,
        min_order_quantity: 1_000_000,
        min_order_notional: 5_000_000,
    },
    MarketSpec {
        symbol: "DOGE_USDC",
        name: "DOGE / USDC",
        base: "DOGE",
        quote: "USDC",
        price_tick_size: 100,
        quantity_step_size: 100_000_000,
        min_order_quantity: 1_000_000_000,
        min_order_notional: 5_000_000,
    },
    MarketSpec {
        symbol: "LINK_USDC",
        name: "LINK / USDC",
        base: "LINK",
        quote: "USDC",
        price_tick_size: 1_000,
        quantity_step_size: 10_000,
        min_order_quantity: 100_000,
        min_order_notional: 5_000_000,
    },
    MarketSpec {
        symbol: "AVAX_USDC",
        name: "AVAX / USDC",
        base: "AVAX",
        quote: "USDC",
        price_tick_size: 1_000,
        quantity_step_size: 10_000,
        min_order_quantity: 100_000,
        min_order_notional: 5_000_000,
    },
    MarketSpec {
        symbol: "SUI_USDC",
        name: "SUI / USDC",
        base: "SUI",
        quote: "USDC",
        price_tick_size: 100,
        quantity_step_size: 10_000_000,
        min_order_quantity: 100_000_000,
        min_order_notional: 5_000_000,
    },
    MarketSpec {
        symbol: "BTC_USDT",
        name: "BTC / USDT",
        base: "BTC",
        quote: "USDT",
        price_tick_size: 10_000,
        quantity_step_size: 1_000,
        min_order_quantity: 10_000,
        min_order_notional: 10_000_000,
    },
    MarketSpec {
        symbol: "ETH_USDT",
        name: "ETH / USDT",
        base: "ETH",
        quote: "USDT",
        price_tick_size: 10_000,
        quantity_step_size: 10_000,
        min_order_quantity: 100_000,
        min_order_notional: 10_000_000,
    },
];

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await
        .expect("cannot connect to database");

    let password_hash = hash_password(SEED_PASSWORD);

    let user_id = upsert_user(&pool, SEED_NAME, SEED_EMAIL, &password_hash)
        .await
        .expect("failed to seed user");

    let mut asset_ids = std::collections::HashMap::<&str, Uuid>::new();
    for asset in ASSETS {
        let id = upsert_asset(&pool, asset.symbol, asset.name, asset.decimals)
            .await
            .unwrap_or_else(|err| panic!("failed to seed {}: {err}", asset.symbol));
        asset_ids.insert(asset.symbol, id);
    }

    let mut market_ids = Vec::new();
    for market in MARKETS {
        let base_id = *asset_ids
            .get(market.base)
            .unwrap_or_else(|| panic!("missing base asset {}", market.base));
        let quote_id = *asset_ids
            .get(market.quote)
            .unwrap_or_else(|| panic!("missing quote asset {}", market.quote));
        let id = upsert_market(
            &pool,
            market.symbol,
            market.name,
            base_id,
            quote_id,
            market.price_tick_size,
            market.quantity_step_size,
            market.min_order_quantity,
            market.min_order_notional,
        )
        .await
        .unwrap_or_else(|err| panic!("failed to seed {}: {err}", market.symbol));
        market_ids.push((market.symbol, id));
    }

    println!("seed complete (catalog only — use demo credit for balances)");
    println!("  user_id:    {user_id}");
    println!("  email:      {SEED_EMAIL}");
    println!("  password:   {SEED_PASSWORD}");
    println!("  assets:     {}", ASSETS.len());
    println!("  markets:    {}", MARKETS.len());
    for asset in ASSETS {
        println!("  {:<8} {}", asset.symbol, asset_ids[asset.symbol]);
    }
    for (symbol, id) in market_ids {
        println!("  {symbol:<12} {id}");
    }
}

fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("failed to hash password")
        .to_string()
}

async fn upsert_user(
    pool: &PgPool,
    name: &str,
    email: &str,
    password_hash: &str,
) -> Result<Uuid, sqlx::Error> {
    let row = sqlx::query(
        r#"
        INSERT INTO users (name, email, password_hash)
        VALUES ($1, $2, $3)
        ON CONFLICT (email) DO UPDATE
        SET
            name = EXCLUDED.name,
            password_hash = EXCLUDED.password_hash,
            updated_at = NOW()
        RETURNING id
        "#,
    )
    .bind(name)
    .bind(email)
    .bind(password_hash)
    .fetch_one(pool)
    .await?;

    Ok(row.get("id"))
}

async fn upsert_asset(
    pool: &PgPool,
    symbol: &str,
    name: &str,
    decimals: i32,
) -> Result<Uuid, sqlx::Error> {
    let row = sqlx::query(
        r#"
        INSERT INTO assets (symbol, name, decimals, status)
        VALUES ($1, $2, $3, 'active')
        ON CONFLICT (symbol) DO UPDATE
        SET
            name = EXCLUDED.name,
            decimals = EXCLUDED.decimals,
            status = 'active'
        RETURNING id
        "#,
    )
    .bind(symbol)
    .bind(name)
    .bind(decimals)
    .fetch_one(pool)
    .await?;

    Ok(row.get("id"))
}

async fn upsert_market(
    pool: &PgPool,
    symbol: &str,
    name: &str,
    base_asset_id: Uuid,
    quote_asset_id: Uuid,
    price_tick_size: i64,
    quantity_step_size: i64,
    min_order_quantity: i64,
    min_order_notional: i64,
) -> Result<Uuid, sqlx::Error> {
    let row = sqlx::query(
        r#"
        INSERT INTO markets (
            symbol,
            name,
            base_asset_id,
            quote_asset_id,
            status,
            price_tick_size,
            quantity_step_size,
            min_order_quantity,
            min_order_notional
        )
        VALUES ($1, $2, $3, $4, 'trading', $5, $6, $7, $8)
        ON CONFLICT (symbol) DO UPDATE
        SET
            name = EXCLUDED.name,
            base_asset_id = EXCLUDED.base_asset_id,
            quote_asset_id = EXCLUDED.quote_asset_id,
            status = 'trading',
            price_tick_size = EXCLUDED.price_tick_size,
            quantity_step_size = EXCLUDED.quantity_step_size,
            min_order_quantity = EXCLUDED.min_order_quantity,
            min_order_notional = EXCLUDED.min_order_notional
        RETURNING id
        "#,
    )
    .bind(symbol)
    .bind(name)
    .bind(base_asset_id)
    .bind(quote_asset_id)
    .bind(price_tick_size)
    .bind(quantity_step_size)
    .bind(min_order_quantity)
    .bind(min_order_notional)
    .fetch_one(pool)
    .await?;

    Ok(row.get("id"))
}
