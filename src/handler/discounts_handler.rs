use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sea_orm::{prelude::{DateTimeWithTimeZone, Decimal}, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use entity::discounts::{self, Entity as Discounts};
use crate::utilities::app_error::AppError;

#[derive(Serialize, Deserialize)]
pub struct DiscountResponse {
  pub  id: i32,
  pub  product_id: Option<i32>,
  pub  discount_percentage: Option<Decimal>,
  pub  valid_from: DateTimeWithTimeZone,
  pub  valid_to: DateTimeWithTimeZone,
}

#[derive(Deserialize)]
pub struct CreateDiscount {
   pub product_id: Option<i32>,
   pub discount_percentage: Option<Decimal>,
   pub valid_from: DateTimeWithTimeZone,
   pub valid_to: DateTimeWithTimeZone,
}

#[derive(Deserialize)]
pub struct UpdateDiscount {
   pub product_id: Option<i32>,
   pub discount_percentage: Option<Decimal>,
   pub valid_from: Option<DateTimeWithTimeZone>,
   pub valid_to: Option<DateTimeWithTimeZone>,
}

pub async fn create_discount(
    State(db): State<DatabaseConnection>,
    Json(input): Json<CreateDiscount>,
) -> Result<Json<DiscountResponse>, AppError> {
    let new_discount = discounts::ActiveModel {
        product_id: Set(input.product_id),
        discount_percentage: Set(input.discount_percentage),
        valid_from: Set(input.valid_from),
        valid_to: Set(input.valid_to),
        ..Default::default()
    };

    let discount = new_discount.insert(&db).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again.",
        )
    })?;

    Ok(Json(DiscountResponse {
        id: discount.id,
        product_id: discount.product_id,
        discount_percentage: discount.discount_percentage,
        valid_from: discount.valid_from,
        valid_to: discount.valid_to,
    }))
}

pub async fn get_discount_by_id(
    State(db): State<DatabaseConnection>,
    Path(discount_id): Path<i32>,
) -> Result<Json<DiscountResponse>, AppError> {
    let discount = Discounts::find_by_id(discount_id)
        .one(&db)
        .await
        .map_err(|_| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again.",
            )
        })?;

    match discount {
        Some(discount) => Ok(Json(DiscountResponse {
            id: discount.id,
            product_id: discount.product_id,
            discount_percentage: discount.discount_percentage,
            valid_from: discount.valid_from,
            valid_to: discount.valid_to,
        })),
        None => Err(AppError::new(StatusCode::NOT_FOUND, "Discount not found")),
    }
}

pub async fn get_all_discounts(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<DiscountResponse>>, AppError> {
    let discounts = Discounts::find()
        .all(&db)
        .await
        .map_err(|_| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again.",
            )
        })?;

    let discount_responses: Vec<DiscountResponse> = discounts
        .into_iter()
        .map(|discount| DiscountResponse {
            id: discount.id,
            product_id: discount.product_id,
            discount_percentage: discount.discount_percentage,
            valid_from: discount.valid_from,
            valid_to: discount.valid_to,
        })
        .collect();

    Ok(Json(discount_responses))
}

pub async fn update_discount(
    State(db): State<DatabaseConnection>,
    Path(discount_id): Path<i32>,
    Json(input): Json<UpdateDiscount>,
) -> Result<Json<DiscountResponse>, AppError> {
    let mut discount: discounts::ActiveModel = Discounts::find_by_id(discount_id)
        .one(&db)
        .await
        .map_err(|_| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again.",
            )
        })?
        .ok_or_else(|| AppError::new(StatusCode::NOT_FOUND, "Discount not found"))?
        .into();

    if let Some(product_id) = input.product_id {
        discount.product_id = Set(Some(product_id));
    }
    if let Some(discount_percentage) = input.discount_percentage {
        discount.discount_percentage = Set(Some(discount_percentage));
    }
    if let Some(valid_from) = input.valid_from {
        discount.valid_from = Set(valid_from);
    }
    if let Some(valid_to) = input.valid_to {
        discount.valid_to = Set(valid_to);
    }

    let discount = discount.update(&db).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again.",
        )
    })?;

    Ok(Json(DiscountResponse {
        id: discount.id,
        product_id: discount.product_id,
        discount_percentage: discount.discount_percentage,
        valid_from: discount.valid_from,
        valid_to: discount.valid_to,
    }))
}

pub async fn delete_discount(
    State(db): State<DatabaseConnection>,
    Path(discount_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    Discounts::delete_by_id(discount_id).exec(&db).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again.",
        )
    })?;

    Ok((StatusCode::OK, format!("Discount with id {} deleted", discount_id)))
}