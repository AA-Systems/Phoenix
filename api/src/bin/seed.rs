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

    let sol_id = upsert_asset(&pool, "SOL", "Solana", 9)
        .await
        .expect("failed to seed SOL");
    let usdc_id = upsert_asset(&pool, "USDC", "USD Coin", 6)
        .await
        .expect("failed to seed USDC");
    let usd_id = upsert_asset(&pool, "USD", "US Dollar", 2)
        .await
        .expect("failed to seed USD");
    let inr_id = upsert_asset(&pool, "INR", "Indian Rupee", 2)
        .await
        .expect("failed to seed INR");

    let sol_usdc = upsert_market(
        &pool,
        "SOL_USDC",
        "SOL / USDC",
        sol_id,
        usdc_id,
        10_000,     // 0.01 USDC
        1_000_000,  // 0.001 SOL
        10_000_000, // 0.01 SOL
        5_000_000,  // 5 USDC
    )
    .await
    .expect("failed to seed SOL_USDC");

    let sol_usd = upsert_market(
        &pool,
        "SOL_USD",
        "SOL / USD",
        sol_id,
        usd_id,
        1, // 0.01 USD
        1_000_000,
        10_000_000,
        500, // 5.00 USD
    )
    .await
    .expect("failed to seed SOL_USD");

    let sol_inr = upsert_market(
        &pool,
        "SOL_INR",
        "SOL / INR",
        sol_id,
        inr_id,
        1, // 0.01 INR
        1_000_000,
        10_000_000,
        50_000, // ₹500.00
    )
    .await
    .expect("failed to seed SOL_INR");

    // Generous demo balances (atomic units)
    credit_balance(&pool, user_id, sol_id, 100_000_000_000).await; // 100 SOL
    credit_balance(&pool, user_id, usdc_id, 1_000_000_000_000).await; // 1,000,000 USDC
    credit_balance(&pool, user_id, usd_id, 100_000_00).await; // 100,000.00 USD
    credit_balance(&pool, user_id, inr_id, 1_000_000_00).await; // ₹1,000,000.00

    println!("seed complete");
    println!("  user_id:    {user_id}");
    println!("  email:      {SEED_EMAIL}");
    println!("  password:   {SEED_PASSWORD}");
    println!("  SOL:        {sol_id}");
    println!("  USDC:       {usdc_id}");
    println!("  USD:        {usd_id}");
    println!("  INR:        {inr_id}");
    println!("  SOL_USDC:   {sol_usdc}");
    println!("  SOL_USD:    {sol_usd}");
    println!("  SOL_INR:    {sol_inr}");
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

async fn credit_balance(pool: &PgPool, user_id: Uuid, asset_id: Uuid, available: i64) {
    sqlx::query(
        r#"
        INSERT INTO balances (user_id, asset_id, available, locked, updated_at)
        VALUES ($1, $2, $3, 0, NOW())
        ON CONFLICT (user_id, asset_id) DO UPDATE
        SET
            available = EXCLUDED.available,
            locked = 0,
            updated_at = NOW()
        "#,
    )
    .bind(user_id)
    .bind(asset_id)
    .bind(available)
    .execute(pool)
    .await
    .expect("failed to seed balance");
}
