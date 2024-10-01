use crate::{
    app_state::AppState,
    handler::order_handler::{create_order, delete_order, get_all_orders, get_order_by_id, update_order},
    api_middleware::auth_middleware::require_authentication,
};
use axum::{

    middleware,
    routing::{get, post},
    Router,
};

pub fn order_route(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/orders", post(create_order).get(get_all_orders))
        .route(
            "/orders/:id",
            get(get_order_by_id)
                .put(update_order)
                .delete(delete_order),
        )
        .layer(middleware::from_fn_with_state(
            app_state.clone(), // Ensure the AppState is passed to the middleware
            require_authentication,
        ))
}