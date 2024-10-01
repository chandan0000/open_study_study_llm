use crate::{
    api_middleware::admin_midleware::require_admin,
    app_state::AppState,
    handler::categories_handler::{
        create_category, delete_category, get_all_categories, get_category_by_id, update_category,
    },
};
use axum::{
    middleware,
    routing::{get, post},
    Router,
};

pub fn category_route(app_state: AppState) -> Router<AppState> {
    // let auth_middleware = ServiceBuilder::new().layer(axum::middleware::from_fn(require_authentication));

    Router::new()
        .route("/categories", post(create_category))
        .route(
            "/categories/:id",
            get(get_category_by_id)
                .put(update_category)
                .delete(delete_category),
        )
        .layer(middleware::from_fn_with_state(
            app_state.clone(), // Ensure the AppState is passed to the middleware
            require_admin,
        ))
        .route("/categories", get(get_all_categories))
        .route("/categories_by_id/:id", get(get_category_by_id))
}
