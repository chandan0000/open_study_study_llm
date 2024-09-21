use axum::{routing::get, Router};

use crate::app_state::AppState;

pub fn create_v1_routes() -> Router<AppState> {
    Router::new().route("/", get(|| async { "Hello from API version 1" }))
}
