use crate::{
    api_middleware::auth_middleware::require_authentication,
    app_state::{AppState},
    handler::cart_handler::{create_cart, delete_cart, get_all_carts, get_cart_by_id, update_cart},
};
use axum::{
    middleware,
    routing::{get, post},
    Router,
};

pub fn cart_route(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/cart", post(create_cart).get(get_all_carts))
        .route(
            "/cart/:id",
            get(get_cart_by_id).put(update_cart).delete(delete_cart), // Apply authentication middleware
        )
        .layer(middleware::from_fn_with_state(
            app_state.clone(), // Ensure the AppState is passed to the middleware
            require_authentication,
        ))
}
