use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(name = "demo_bot", about = "Demo liquidity + trade bot for Phoenix")]
pub struct BotConfig {
    /// API base URL
    #[arg(
        long,
        env = "DEMO_BOT_API_URL",
        default_value = "http://localhost:3000"
    )]
    pub api_url: String,

    /// Comma-separated markets
    #[arg(long, env = "DEMO_BOT_MARKETS", default_value = "SOL_USDC,BTC_USDC")]
    pub markets: String,

    /// Target resting orders per market (split bid/ask)
    #[arg(long, env = "DEMO_BOT_ORDERS", default_value_t = 40)]
    pub orders: u32,

    /// Crossing trades per market per cycle (keep small so book stays dense)
    #[arg(long, env = "DEMO_BOT_TRADES", default_value_t = 1)]
    pub trades: u32,

    /// Max create/cancel HTTP calls per second
    #[arg(long, env = "DEMO_BOT_RATE", default_value_t = 20)]
    pub rate: u32,

    /// Max book mutations (cancel or create) per market per cycle
    #[arg(long, env = "DEMO_BOT_MAX_ACTIONS", default_value_t = 6)]
    pub max_actions: u32,

    /// Max mid move per cycle in ticks (random walk)
    #[arg(long, env = "DEMO_BOT_WALK_TICKS", default_value_t = 1)]
    pub walk_ticks: u32,

    /// Spread from mid to first level, in ticks
    #[arg(long, env = "DEMO_BOT_SPREAD_TICKS", default_value_t = 2)]
    pub spread_ticks: u32,

    /// Pause between cycles (ms)
    #[arg(long, env = "DEMO_BOT_CYCLE_MS", default_value_t = 800)]
    pub cycle_ms: u64,

    /// Re-run demo credit every N cycles (0 = only at start)
    #[arg(long, env = "DEMO_BOT_RECREDIT_EVERY", default_value_t = 40)]
    pub recredit_every: u32,

    #[arg(
        long,
        env = "DEMO_BOT_MAKER_EMAIL",
        default_value = "demo-maker@cex.local"
    )]
    pub maker_email: String,

    #[arg(
        long,
        env = "DEMO_BOT_MAKER_PASSWORD",
        default_value = "DemoMaker1!pass"
    )]
    pub maker_password: String,

    #[arg(
        long,
        env = "DEMO_BOT_TAKER_EMAIL",
        default_value = "demo-taker@cex.local"
    )]
    pub taker_email: String,

    #[arg(
        long,
        env = "DEMO_BOT_TAKER_PASSWORD",
        default_value = "DemoTaker1!pass"
    )]
    pub taker_password: String,
}

impl BotConfig {
    pub fn market_list(&self) -> Vec<String> {
        self.markets
            .split(',')
            .map(|s| s.trim().to_uppercase())
            .filter(|s| !s.is_empty())
            .collect()
    }

    pub fn request_delay(&self) -> std::time::Duration {
        let rate = self.rate.max(1) as u64;
        std::time::Duration::from_millis((1000 / rate).max(1))
    }
}
