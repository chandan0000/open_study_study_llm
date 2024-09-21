mod v1;
mod v2;
use axum::{extract::FromRef, http::StatusCode, Router};
use sea_orm::DatabaseConnection;

use crate::app_state::AppState;



pub fn create_routes(database: DatabaseConnection) -> Router {
    let app_state = AppState { database };
    Router::new()
        .nest("/v1", v1::create_v1_routes())
        .nest("/v2", v2::create_v2_routes())
        .fallback(fallback)
        .with_state(app_state)
}

async fn fallback(uri: axum::http::Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route found for {}", uri))
}
