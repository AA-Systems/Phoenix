use axum::{Router, middleware, routing::post};

use crate::{
    app_state::AppState,
    handlers::balances::{
        credit_balance::credit_balance, demo_credit::demo_credit,
        get_balance_by_user_id::get_balance_by_user_id, list_ledger::list_ledger_entries,
    },
    middlewares::{admin_auth::admin_auth, jwt_middleware::jwt_middleware},
};

pub fn balances_router(app_state: AppState) -> Router<AppState> {
    let user_routes = Router::new()
        .route("/get", post(get_balance_by_user_id))
        .route("/ledger", post(list_ledger_entries))
        .route("/demo-credit", post(demo_credit))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            jwt_middleware,
        ));

    let admin_routes = Router::new()
        .route("/admin/credit", post(credit_balance))
        .layer(middleware::from_fn_with_state(app_state, admin_auth));

    Router::new().merge(user_routes).merge(admin_routes)
}
