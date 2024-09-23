use axum::{routing::get, Router};
mod auth_route;

use crate::app_state::AppState;

pub fn create_v1_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(|| async { "Hello from API version 1" }))
        .merge(auth_route::auth_route())
}
