use axum::{routing::get, Router};
mod auth_route;
mod cart_item_route;
mod cart_route;
mod category_route;
mod product_route;
mod users_route;
mod oder_items_route;
mod oder_route;

use crate::app_state::AppState;

pub fn create_v1_routes(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(|| async { "Hello from API version 1" }))
        .merge(users_route::user_route(app_state.clone())) // Pass AppState to user_route
        .merge(auth_route::auth_route())
        .merge(category_route::category_route(app_state.clone()))
        .merge(product_route::product_route(app_state.clone()))
        .merge(cart_route::cart_route(app_state.clone()))
        .merge(cart_item_route::cart_item_route(app_state.clone()))
        .merge(oder_items_route::order_item_route(app_state.clone()))
        .merge(oder_route::order_route(app_state.clone()))
        .with_state(app_state) // Pass the AppState to the whole v1 routes
}
