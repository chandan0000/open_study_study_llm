use axum::{extract::State, routing::get, Router};
use sea_orm::DatabaseConnection;
mod auth_route;
mod users_route;

use crate::app_state::AppState;

pub fn create_v1_routes(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(|| async { "Hello from API version 1" }))
        .merge(users_route::user_route(app_state.clone())) // Pass AppState to user_route
        .merge(auth_route::auth_route())
        .with_state(app_state) // Pass the AppState to the whole v1 routes
}
