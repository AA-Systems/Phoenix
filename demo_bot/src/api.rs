use anyhow::{Context, Result, bail};
use reqwest::Client;
use serde::Deserialize;
use types::assets::Asset;
use types::markets::Market;
use types::order::{OpenOrderView, OrderType};

#[derive(Clone)]
pub struct ApiClient {
    http: Client,
    base: String,
    token: String,
}

#[derive(Debug, Deserialize)]
struct AuthBody {
    access_token: String,
}

impl ApiClient {
    pub fn new(base: impl Into<String>) -> Result<Self> {
        let http = Client::builder()
            .timeout(std::time::Duration::from_secs(15))
            .build()?;
        Ok(Self {
            http,
            base: base.into().trim_end_matches('/').to_string(),
            token: String::new(),
        })
    }

    pub fn with_token(mut self, token: String) -> Self {
        self.token = token;
        self
    }

    pub async fn ensure_user(&self, name: &str, email: &str, password: &str) -> Result<String> {
        match self.login(email, password).await {
            Ok(token) => Ok(token),
            Err(_) => {
                let _ = self.register(name, email, password).await;
                self.login(email, password)
                    .await
                    .context("login after register failed")
            }
        }
    }

    async fn register(&self, name: &str, email: &str, password: &str) -> Result<()> {
        let res = self
            .http
            .post(format!("{}/api/v1/auth/register", self.base))
            .json(&serde_json::json!({
                "name": name,
                "email": email,
                "password": password,
            }))
            .send()
            .await?;
        // Already registered comes back as 400 unique violation — treat as ok.
        if res.status().is_success() || res.status().as_u16() == 400 {
            return Ok(());
        }
        let status = res.status();
        let body = res.text().await.unwrap_or_default();
        bail!("register failed ({status}): {body}");
    }

    async fn login(&self, email: &str, password: &str) -> Result<String> {
        let res = self
            .http
            .post(format!("{}/api/v1/auth/login", self.base))
            .json(&serde_json::json!({
                "email": email,
                "password": password,
            }))
            .send()
            .await?;
        if !res.status().is_success() {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            bail!("login failed ({status}): {body}");
        }
        let body: AuthBody = res.json().await?;
        Ok(body.access_token)
    }

    pub async fn demo_credit(&self) -> Result<()> {
        let res = self
            .http
            .post(format!("{}/api/v1/balances/demo-credit", self.base))
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&serde_json::json!({}))
            .send()
            .await?;
        if !res.status().is_success() {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            bail!("demo-credit failed ({status}): {body}");
        }
        Ok(())
    }

    pub async fn list_markets(&self) -> Result<Vec<Market>> {
        let res = self
            .http
            .get(format!("{}/api/v1/markets?limit=100&skip=0", self.base))
            .send()
            .await?;
        if !res.status().is_success() {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            bail!("list markets failed ({status}): {body}");
        }
        Ok(res.json().await?)
    }

    pub async fn list_assets(&self) -> Result<Vec<Asset>> {
        let res = self
            .http
            .get(format!("{}/api/v1/assets", self.base))
            .send()
            .await?;
        if !res.status().is_success() {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            bail!("list assets failed ({status}): {body}");
        }
        Ok(res.json().await?)
    }

    pub async fn create_order(
        &self,
        market_symbol: &str,
        order_type: OrderType,
        price: i64,
        quantity: i64,
    ) -> Result<()> {
        let side = match order_type {
            OrderType::Buy => "buy",
            OrderType::Sell => "sell",
        };
        let res = self
            .http
            .post(format!("{}/api/v1/orders/create", self.base))
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&serde_json::json!({
                "market_symbol": market_symbol,
                "order_type": side,
                "price": price,
                "quantity": quantity,
            }))
            .send()
            .await?;
        if !res.status().is_success() {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            bail!("create order failed ({status}): {body}");
        }
        Ok(())
    }

    pub async fn cancel_order(&self, order_id: &str) -> Result<()> {
        let res = self
            .http
            .post(format!("{}/api/v1/orders/cancel", self.base))
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&serde_json::json!({ "order_id": order_id }))
            .send()
            .await?;
        if !res.status().is_success() {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            bail!("cancel order failed ({status}): {body}");
        }
        Ok(())
    }

    pub async fn open_orders(&self) -> Result<Vec<OpenOrderView>> {
        let res = self
            .http
            .post(format!("{}/api/v1/orders/open", self.base))
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?;
        if !res.status().is_success() {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            bail!("open orders failed ({status}): {body}");
        }
        Ok(res.json().await?)
    }
}
