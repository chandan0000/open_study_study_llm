use crate::{
    app_state::AppState,
    handler::order_items_handler::{
        create_order_item, delete_order_item, get_all_order_items, get_order_item_by_id,
        update_order_item,
    },
    api_middleware::auth_middleware::require_authentication,
};
use axum::{
    middleware,
    routing::{ get, post,},
    Router,
};
 
pub fn order_item_route(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/order_items",
            post(create_order_item).get(get_all_order_items),
        )
        .route(
            "/order_items/:id",
            get(get_order_item_by_id)
                .put(update_order_item)
                .delete(delete_order_item), // Apply authentication middleware
        )
        .layer(middleware::from_fn_with_state(
            app_state.clone(), // Ensure the AppState is passed to the middleware
            require_authentication,
        ))
}
