use axum::{middleware, routing::get, Router};

use crate::{
    app_state::AppState,
    handler::users::{delete_user, get_all_users, get_user_by_id, update_user},
    api_middleware::auth_middleware::require_authentication,
};

pub fn user_route(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/users/:id", // Proper route with leading slash
            get(get_user_by_id).put(update_user).delete(delete_user),
        )
        .route("/users/", get(get_all_users)) // Another proper route
        .layer(middleware::from_fn_with_state(
            app_state.clone(), // Ensure the AppState is passed to the middleware
            require_authentication,
        ))
        .with_state(app_state) // Ensure the state is propagated
}
