mod v1;
mod v2;
use axum::{http::StatusCode, Router};
use sea_orm::DatabaseConnection;

use crate::{api_middleware::auth_middleware::require_authentication, app_state::AppState};

pub fn create_routes(database: DatabaseConnection) -> Router {
    let app_state = AppState { database };
    
    Router::new()
        .nest("/v1", v1::create_v1_routes(app_state.clone())) // Pass app_state to v1 routes
        .nest("/v2", v2::create_v2_routes()) // Assuming v2 doesn't need AppState, if it does, pass it similarly
        .fallback(fallback)
       
        .with_state(app_state) // Set the AppState for the root router
}

async fn fallback(uri: axum::http::Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route found for {}", uri))
}
