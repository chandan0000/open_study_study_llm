use crate::{
    api_middleware::auth_middleware::require_authentication, app_state::{self, AppState}, handler::cart_item_handler::{create_cart_item, delete_cart_item, get_all_cart_items, get_cart_item_by_id, update_cart_item}
};
use axum::{middleware, routing::{delete, get, post, put}, Router};
 
pub fn cart_item_route(app_state:AppState) -> Router<AppState> {
 

    Router::new()
        .route("/cart_items", post(create_cart_item).get(get_all_cart_items))
        .route(
            "/cart_items/:id",
            get(get_cart_item_by_id)
                .put(update_cart_item)
                .delete(delete_cart_item)
         )
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            require_authentication,))
}