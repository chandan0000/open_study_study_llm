use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sea_orm::{prelude::Decimal, ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use entity::order_items::{self, Entity as OrderItems};
use crate::utilities::app_error::AppError;

#[derive(Serialize, Deserialize)]
pub struct OrderItemResponse {
  pub   id: i32,
  pub   order_id: Option<i32>,
  pub   product_id: Option<i32>,
  pub   quantity: i32,
  pub   price_at_time: Decimal,
}

#[derive(Deserialize)]
pub struct CreateOrderItem {
  pub  order_id: Option<i32>,
  pub  product_id: Option<i32>,
  pub  quantity: i32,
  pub  price_at_time: Decimal,
}

#[derive(Deserialize)]
pub struct UpdateOrderItem {
   pub order_id: Option<i32>,
   pub product_id: Option<i32>,
   pub quantity: Option<i32>,
   pub price_at_time: Option<Decimal>,
}

pub async fn create_order_item(
    State(db): State<DatabaseConnection>,
    Json(input): Json<CreateOrderItem>,
) -> Result<Json<OrderItemResponse>, AppError> {
    let new_order_item = order_items::ActiveModel {
        order_id: Set(input.order_id),
        product_id: Set(input.product_id),
        quantity: Set(input.quantity),
        price_at_time: Set(input.price_at_time),
        ..Default::default()
    };

    let order_item = new_order_item.insert(&db).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again.",
        )
    })?;

    Ok(Json(OrderItemResponse {
        id: order_item.id,
        order_id: order_item.order_id,
        product_id: order_item.product_id,
        quantity: order_item.quantity,
        price_at_time: order_item.price_at_time,
    }))
}

pub async fn get_order_item_by_id(
    State(db): State<DatabaseConnection>,
    Path(order_item_id): Path<i32>,
) -> Result<Json<OrderItemResponse>, AppError> {
    let order_item = OrderItems::find_by_id(order_item_id)
        .one(&db)
        .await
        .map_err(|_| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again.",
            )
        })?;

    match order_item {
        Some(order_item) => Ok(Json(OrderItemResponse {
            id: order_item.id,
            order_id: order_item.order_id,
            product_id: order_item.product_id,
            quantity: order_item.quantity,
            price_at_time: order_item.price_at_time,
        })),
        None => Err(AppError::new(StatusCode::NOT_FOUND, "Order item not found")),
    }
}

pub async fn get_all_order_items(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<OrderItemResponse>>, AppError> {
    let order_items = OrderItems::find()
        .all(&db)
        .await
        .map_err(|_| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again.",
            )
        })?;

    let order_item_responses: Vec<OrderItemResponse> = order_items
        .into_iter()
        .map(|order_item| OrderItemResponse {
            id: order_item.id,
            order_id: order_item.order_id,
            product_id: order_item.product_id,
            quantity: order_item.quantity,
            price_at_time: order_item.price_at_time,
        })
        .collect::<Vec<OrderItemResponse>>();

    Ok(Json(order_item_responses))
}

pub async fn update_order_item(
    State(db): State<DatabaseConnection>,
    Path(order_item_id): Path<i32>,
    Json(input): Json<UpdateOrderItem>,
) -> Result<Json<OrderItemResponse>, AppError> {
    let mut order_item: order_items::ActiveModel = OrderItems::find_by_id(order_item_id)
        .one(&db)
        .await
        .map_err(|_| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again.",
            )
        })?
        .ok_or_else(|| AppError::new(StatusCode::NOT_FOUND, "Order item not found"))?
        .into();

    if let Some(order_id) = input.order_id {
        order_item.order_id = Set(Some(order_id));
    }
    if let Some(product_id) = input.product_id {
        order_item.product_id = Set(Some(product_id));
    }
    if let Some(quantity) = input.quantity {
        order_item.quantity = Set(quantity);
    }
    if let Some(price_at_time) = input.price_at_time {
        order_item.price_at_time = Set(price_at_time);
    }

    let order_item = order_item.update(&db).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again.",
        )
    })?;

    Ok(Json(OrderItemResponse {
        id: order_item.id,
        order_id: order_item.order_id,
        product_id: order_item.product_id,
        quantity: order_item.quantity,
        price_at_time: order_item.price_at_time,
    }))
}

pub async fn delete_order_item(
    State(db): State<DatabaseConnection>,
    Path(order_item_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    OrderItems::delete_by_id(order_item_id).exec(&db).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again.",
        )
    })?;

    Ok((StatusCode::OK, format!("Order item with id {} deleted", order_item_id)))
}