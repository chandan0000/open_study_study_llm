use crate::utilities::app_error::AppError;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use entity::cart_items::{self, Entity as CartItems};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CartItemResponse {
    pub id: i32,
    pub cart_id: Option<i32>,
    pub product_id: Option<i32>,
    pub quantity: Option<i32>,
}

#[derive(Deserialize)]
pub struct CreateCartItem {
    pub cart_id: Option<i32>,
    pub product_id: Option<i32>,
    pub quantity: Option<i32>,
}

#[derive(Deserialize)]
pub struct UpdateCartItem {
    pub cart_id: Option<i32>,
    pub product_id: Option<i32>,
    pub quantity: Option<i32>,
}

pub async fn create_cart_item(
    State(db): State<DatabaseConnection>,
    Json(input): Json<CreateCartItem>,
) -> Result<Json<CartItemResponse>, AppError> {
    let new_cart_item = cart_items::ActiveModel {
        cart_id: Set(input.cart_id),
        product_id: Set(input.product_id),
        quantity: Set(input.quantity),
        ..Default::default()
    };

    let cart_item = new_cart_item.insert(&db).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again.",
        )
    })?;

    Ok(Json(CartItemResponse {
        id: cart_item.id,
        cart_id: cart_item.cart_id,
        product_id: cart_item.product_id,
        quantity: cart_item.quantity,
    }))
}

pub async fn get_cart_item_by_id(
    State(db): State<DatabaseConnection>,
    Path(cart_item_id): Path<i32>,
) -> Result<Json<CartItemResponse>, AppError> {
    let cart_item = CartItems::find_by_id(cart_item_id)
        .one(&db)
        .await
        .map_err(|_| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again.",
            )
        })?;

    match cart_item {
        Some(cart_item) => Ok(Json(CartItemResponse {
            id: cart_item.id,
            cart_id: cart_item.cart_id,
            product_id: cart_item.product_id,
            quantity: cart_item.quantity,
        })),
        None => Err(AppError::new(StatusCode::NOT_FOUND, "Cart item not found")),
    }
}

pub async fn get_all_cart_items(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<CartItemResponse>>, AppError> {
    let cart_items = CartItems::find().all(&db).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again.",
        )
    })?;

    let cart_item_responses: Vec<CartItemResponse> = cart_items
        .into_iter()
        .map(|cart_item| CartItemResponse {
            id: cart_item.id,
            cart_id: cart_item.cart_id,
            product_id: cart_item.product_id,
            quantity: cart_item.quantity,
        })
        .collect();

    Ok(Json(cart_item_responses))
}

pub async fn update_cart_item(
    State(db): State<DatabaseConnection>,
    Path(cart_item_id): Path<i32>,
    Json(input): Json<UpdateCartItem>,
) -> Result<Json<CartItemResponse>, AppError> {
    let mut cart_item: cart_items::ActiveModel = CartItems::find_by_id(cart_item_id)
        .one(&db)
        .await
        .map_err(|_| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again.",
            )
        })?
        .ok_or_else(|| AppError::new(StatusCode::NOT_FOUND, "Cart item not found"))?
        .into();

    if let Some(cart_id) = input.cart_id {
        cart_item.cart_id = Set(Some(cart_id));
    }
    if let Some(product_id) = input.product_id {
        cart_item.product_id = Set(Some(product_id));
    }
    if let Some(quantity) = input.quantity {
        cart_item.quantity = Set(Some(quantity));
    }

    let cart_item = cart_item.update(&db).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again.",
        )
    })?;

    Ok(Json(CartItemResponse {
        id: cart_item.id,
        cart_id: cart_item.cart_id,
        product_id: cart_item.product_id,
        quantity: cart_item.quantity,
    }))
}

pub async fn delete_cart_item(
    State(db): State<DatabaseConnection>,
    Path(cart_item_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    CartItems::delete_by_id(cart_item_id)
        .exec(&db)
        .await
        .map_err(|_| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again.",
            )
        })?;

    Ok((
        StatusCode::OK,
        format!("Cart item with id {} deleted", cart_item_id),
    ))
}
