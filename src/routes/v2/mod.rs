use axum::{Router, routing::get};

use crate::app_state::AppState;
 
pub fn create_v2_routes() -> Router<AppState> {
    Router::new()
        .route("/hello", get(||async{ "Hello from API version 2"}))
}
