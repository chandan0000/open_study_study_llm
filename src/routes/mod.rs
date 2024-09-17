mod hello_server;
 
use axum::{
    extract::FromRef,
    middleware::{self, from_fn_with_state},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use sea_orm::DatabaseConnection;

use hello_server::hello_server;

async fn protected_route() -> impl IntoResponse {
    "This is a protected route".into_response()
}

#[derive(Clone, FromRef)]
pub struct AppState {
    pub database: DatabaseConnection,
}

pub fn create_routes(database: DatabaseConnection) -> Router {
    let app_state = AppState { database };
    Router::new()
      
        .route("/", get(hello_server))
     
        .with_state(app_state)
}