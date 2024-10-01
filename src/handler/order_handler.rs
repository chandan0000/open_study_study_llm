use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sea_orm::{prelude::{DateTimeWithTimeZone, Decimal}, ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use entity::orders::{self, Entity as Orders};
use crate::utilities::app_error::AppError;

#[derive(Serialize, Deserialize)]
pub struct OrderResponse {
  pub  id: i32,
  pub  user_id: Option<i32>,
  pub  status: Option<String>,
  pub  total_price: Decimal,
  pub  created_at: Option<DateTimeWithTimeZone>,
  pub  updated_at: Option<DateTimeWithTimeZone>,
}

#[derive(Deserialize)]
pub struct CreateOrder {
   pub user_id: Option<i32>,
   pub status: Option<String>,
   pub total_price: Decimal,
}

#[derive(Deserialize)]
pub struct UpdateOrder {
   pub user_id: Option<i32>,
   pub status: Option<String>,
   pub total_price: Option<Decimal>,
}

pub  async fn create_order(
    State(db): State<DatabaseConnection>,
    Json(input): Json<CreateOrder>,
) -> Result<Json<OrderResponse>, AppError> {
    let new_order = orders::ActiveModel {
        user_id: Set(input.user_id),
        status: Set(input.status),
        total_price: Set(input.total_price),
        ..Default::default()
    };

    let order = new_order.insert(&db).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again.",
        )
    })?;

    Ok(Json(OrderResponse {
        id: order.id,
        user_id: order.user_id,
        status: order.status,
        total_price: order.total_price,
        created_at: order.created_at,
        updated_at: order.updated_at,
    }))
}

pub async fn get_order_by_id(
    State(db): State<DatabaseConnection>,
    Path(order_id): Path<i32>,
) -> Result<Json<OrderResponse>, AppError> {
    let order = Orders::find_by_id(order_id)
        .one(&db)
        .await
        .map_err(|_| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again.",
            )
        })?;

    match order {
        Some(order) => Ok(Json(OrderResponse {
            id: order.id,
            user_id: order.user_id,
            status: order.status,
            total_price: order.total_price,
            created_at: order.created_at,
            updated_at: order.updated_at,
        })),
        None => Err(AppError::new(StatusCode::NOT_FOUND, "Order not found")),
    }
}

pub async fn get_all_orders(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<OrderResponse>>, AppError> {
    let orders = Orders::find()
        .all(&db)
        .await
        .map_err(|_| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again.",
            )
        })?;

    let order_responses: Vec<OrderResponse> = orders
        .into_iter()
        .map(|order| OrderResponse {
            id: order.id,
            user_id: order.user_id,
            status: order.status,
            total_price: order.total_price,
            created_at: order.created_at,
            updated_at: order.updated_at,
        })
        .collect();

    Ok(Json(order_responses))
}

pub async fn update_order(
    State(db): State<DatabaseConnection>,
    Path(order_id): Path<i32>,
    Json(input): Json<UpdateOrder>,
) -> Result<Json<OrderResponse>, AppError> {
    let mut order: orders::ActiveModel = Orders::find_by_id(order_id)
        .one(&db)
        .await
        .map_err(|_| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again.",
            )
        })?
        .ok_or_else(|| AppError::new(StatusCode::NOT_FOUND, "Order not found"))?
        .into();

    if let Some(user_id) = input.user_id {
        order.user_id = Set(Some(user_id));
    }
    if let Some(status) = input.status {
        order.status = Set(Some(status));
    }
    if let Some(total_price) = input.total_price {
        order.total_price = Set(total_price);
    }

    let order = order.update(&db).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again.",
        )
    })?;

    Ok(Json(OrderResponse {
        id: order.id,
        user_id: order.user_id,
        status: order.status,
        total_price: order.total_price,
        created_at: order.created_at,
        updated_at: order.updated_at,
    }))
}

pub async fn delete_order(
    State(db): State<DatabaseConnection>,
    Path(order_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    Orders::delete_by_id(order_id).exec(&db).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again.",
        )
    })?;

    Ok((StatusCode::OK, format!("Order with id {} deleted", order_id)))
}