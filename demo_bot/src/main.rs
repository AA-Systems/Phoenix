mod api;
mod config;
mod engine;

use std::time::Duration;

use anyhow::Result;
use clap::Parser;
use tracing::{info, warn};

use crate::api::ApiClient;
use crate::config::BotConfig;
use crate::engine::{bootstrap, run_cycle};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let config = BotConfig::parse();
    info!(
        api = %config.api_url,
        markets = %config.markets,
        orders = config.orders,
        trades = config.trades,
        rate = config.rate,
        "starting demo bot"
    );

    let mut maker = login_bot(
        &config.api_url,
        "Demo Maker",
        &config.maker_email,
        &config.maker_password,
    )
    .await?;
    let mut taker = login_bot(
        &config.api_url,
        "Demo Taker",
        &config.taker_email,
        &config.taker_password,
    )
    .await?;

    let mut runtimes = bootstrap(&config, &maker, &taker).await?;
    let mut cycle: u32 = 0;

    loop {
        cycle += 1;
        if let Err(err) = run_cycle(&config, &maker, &taker, &mut runtimes).await {
            warn!(%err, cycle, "cycle failed");
        } else {
            info!(cycle, "cycle complete");
        }

        if config.recredit_every > 0 && cycle % config.recredit_every == 0 {
            // Refresh JWTs (access TTL is short) and top up balances.
            match login_bot(
                &config.api_url,
                "Demo Maker",
                &config.maker_email,
                &config.maker_password,
            )
            .await
            {
                Ok(client) => maker = client,
                Err(err) => warn!(%err, "maker re-login failed"),
            }
            match login_bot(
                &config.api_url,
                "Demo Taker",
                &config.taker_email,
                &config.taker_password,
            )
            .await
            {
                Ok(client) => taker = client,
                Err(err) => warn!(%err, "taker re-login failed"),
            }

            if let Err(err) = maker.demo_credit().await {
                warn!(%err, "maker recredit failed");
            }
            if let Err(err) = taker.demo_credit().await {
                warn!(%err, "taker recredit failed");
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }

        tokio::time::sleep(Duration::from_millis(config.cycle_ms)).await;
    }
}

async fn login_bot(api_url: &str, name: &str, email: &str, password: &str) -> Result<ApiClient> {
    let token = ApiClient::new(api_url)?
        .ensure_user(name, email, password)
        .await?;
    Ok(ApiClient::new(api_url)?.with_token(token))
}
