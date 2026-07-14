use axum::extract::State;
use sqlx::PgPool;

pub async fn health_check(State(_pool): State<PgPool>) -> String {
    String::from("Server is healthy")
}
