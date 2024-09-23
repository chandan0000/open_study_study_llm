use crate::handler::auth::*;
use axum::routing::get;
use axum::{routing::post, Router};

use crate::app_state::AppState;

pub fn auth_route() -> Router<AppState> {
    Router::new()
        .route("/register", post(sign_up))
        .route("/login", post(login))
        .route("/verify", get(verify_email))
        .route("/forgot-password", post(forget_password))
        .route("/reset-password", post(reset_password))
}

