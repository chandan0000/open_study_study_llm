use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sea_orm::{prelude::DateTimeWithTimeZone, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use entity::cart::{self, Entity as Cart};
use crate::utilities::app_error::AppError;

#[derive(Serialize, Deserialize)]
pub struct CartResponse {
   pub id: i32,
   pub user_id: Option<i32>,
   pub created_at: Option<DateTimeWithTimeZone>,
}

#[derive(Deserialize)]
pub struct CreateCart {
  pub  user_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct UpdateCart {
   pub  user_id: Option<i32>,
}

pub async fn create_cart(
    State(db): State<DatabaseConnection>,
    Json(input): Json<CreateCart>,
) -> Result<Json<CartResponse>, AppError> {
    let new_cart = cart::ActiveModel {
        user_id: Set(input.user_id),
        ..Default::default()
    };

    let cart = new_cart.insert(&db).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again.",
        )
    })?;

    Ok(Json(CartResponse {
        id: cart.id,
        user_id: cart.user_id,
        created_at: cart.created_at,
    }))
}

pub async fn get_cart_by_id(
    State(db): State<DatabaseConnection>,
    Path(cart_id): Path<i32>,
) -> Result<Json<CartResponse>, AppError> {
    let cart = Cart::find_by_id(cart_id)
        .one(&db)
        .await
        .map_err(|_| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again.",
            )
        })?;

    match cart {
        Some(cart) => Ok(Json(CartResponse {
            id: cart.id,
            user_id: cart.user_id,
            created_at: cart.created_at,
        })),
        None => Err(AppError::new(StatusCode::NOT_FOUND, "Cart not found")),
    }
}

pub async fn get_all_carts(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<CartResponse>>, AppError> {
    let carts = Cart::find()
        .all(&db)
        .await
        .map_err(|_| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again.",
            )
        })?;

    let cart_responses: Vec<CartResponse> = carts
        .into_iter()
        .map(|cart| CartResponse {
            id: cart.id,
            user_id: cart.user_id,
            created_at: cart.created_at,
        })
        .collect();

    Ok(Json(cart_responses))
}

pub async fn update_cart(
    State(db): State<DatabaseConnection>,
    Path(cart_id): Path<i32>,
    Json(input): Json<UpdateCart>,
) -> Result<Json<CartResponse>, AppError> {
    let mut cart: cart::ActiveModel = Cart::find_by_id(cart_id)
        .one(&db)
        .await
        .map_err(|_| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again.",
            )
        })?
        .ok_or_else(|| AppError::new(StatusCode::NOT_FOUND, "Cart not found"))?
        .into();

    if let Some(user_id) = input.user_id {
        cart.user_id = Set(Some(user_id));
    }

    let cart = cart.update(&db).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again.",
        )
    })?;

    Ok(Json(CartResponse {
        id: cart.id,
        user_id: cart.user_id,
        created_at: cart.created_at,
    }))
}

pub async fn delete_cart(
    State(db): State<DatabaseConnection>,
    Path(cart_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    Cart::delete_by_id(cart_id).exec(&db).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again.",
        )
    })?;

    Ok((StatusCode::OK, format!("Cart with id {} deleted", cart_id)))
}