use axum::extract::State;

use crate::app_state::AppState;

pub async fn health_check(State(_app_state): State<AppState>) -> String {
    String::from("Server is healthy")
}
