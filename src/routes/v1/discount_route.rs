use crate::{
    app_state::AppState,
    handler::discounts_handler::{create_discount, delete_discount, get_all_discounts, get_discount_by_id, update_discount},
    api_middleware::auth_middleware::require_authentication,
};
use axum::{
    middleware,
    routing::{get, post, },
    Router,
};

pub fn discount_route(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/discounts", post(create_discount).get(get_all_discounts))
        .route(
            "/discounts/:id",
            get(get_discount_by_id)
                .put(update_discount)
                .delete(delete_discount),
        )
        .layer(middleware::from_fn_with_state(
            app_state.clone(), // Ensure the AppState is passed to the middleware
            require_authentication,
        ))
}