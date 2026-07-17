use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

use crate::balances::Balance;
use crate::ledger_entries::LedgerEntry;

#[derive(Serialize)]
pub struct CreditBalanceResponse {
    #[serde(skip)]
    status_code: StatusCode,
    pub balance: Balance,
    pub ledger_entry: LedgerEntry,
}

impl CreditBalanceResponse {
    pub fn created(balance: Balance, ledger_entry: LedgerEntry) -> Self {
        Self {
            status_code: StatusCode::CREATED,
            balance,
            ledger_entry,
        }
    }
}

impl IntoResponse for CreditBalanceResponse {
    fn into_response(self) -> Response {
        let status = self.status_code;
        (status, Json(self)).into_response()
    }
}

#[derive(Debug, Deserialize)]
pub struct CreditBalanceBody {
    pub balance: Balance,
    pub ledger_entry: LedgerEntry,
}
