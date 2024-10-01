use crate::{
    api_middleware::auth_middleware::require_authentication,
    app_state::AppState,
    handler::reviews_handler::{
        create_review, delete_review, get_all_reviews, get_review_by_id, update_review,
    },
};
use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};

pub fn review_route(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/reviews", post(create_review).get(get_all_reviews))
        .route(
            "/reviews/:id",
            get(get_review_by_id)
                .put(update_review)
                .delete(delete_review),
        )
        .layer(middleware::from_fn_with_state(
            app_state.clone(), // Ensure the AppState is passed to the middleware
            require_authentication,
        ))
}
