use crate::{
    api_middleware::admin_midleware::require_admin,
    app_state::{self, AppState},
    handler::product_handler::{
        create_product, delete_product, get_all_products, get_product_by_id, update_product,
    },
};
use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};

pub fn product_route(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/products", post(create_product).get(get_all_products))
        .route(
            "/products/:id",
            get(get_product_by_id)
                .put(update_product)
                .delete(delete_product),
        )
        .layer(middleware::from_fn_with_state(
            app_state.clone(), // Ensure the AppState is passed to the middleware
            require_admin,
        ))
}
