use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use sea_orm::{prelude::DateTimeWithTimeZone, ActiveModelTrait, DatabaseConnection, EntityTrait, PaginatorTrait, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use entity::reviews::{self, Entity as Reviews, Model as ReviewModel};
use crate::utilities::app_error::AppError;


#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Deserialize)]
pub struct CreateReview {
  pub  product_id: Option<i32>,
  pub  user_id: Option<i32>,
  pub  rating: Option<i32>,
  pub  review_text: Option<String>,
  pub  created_at: Option<DateTimeWithTimeZone>,
  pub  updated_at: Option<DateTimeWithTimeZone>,
}


#[derive(Serialize, Deserialize)]
pub struct ReviewResponse {
    pub id: i32,
    pub product_id: Option<i32>,
    pub user_id: Option<i32>,
    pub rating: Option<i32>,
    pub review_text: Option<String>,
    pub created_at: Option<DateTimeWithTimeZone>,
    pub updated_at: Option<DateTimeWithTimeZone>,
}



pub async fn create_review(
    State(db): State<DatabaseConnection>,
    Json(input): Json<CreateReview>,
) -> Result<Json<ReviewResponse>, AppError> {
    let new_review = reviews::ActiveModel {
        product_id: Set(input.product_id),
        user_id: Set(input.user_id),
        rating: Set(input.rating),
        review_text: Set(input.review_text),
        created_at: Set(input.created_at),
        updated_at: Set(input.updated_at),
        ..Default::default()
    };

    let review = new_review.insert(&db).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again.",
        )
    })?;

    Ok(Json(ReviewResponse {
        id: review.id,
        product_id: review.product_id,
        user_id: review.user_id,
        rating: review.rating,
        review_text: review.review_text,
        created_at: review.created_at,
        updated_at: review.updated_at,
    }))
}

pub async fn get_review_by_id(
    Extension(review): Extension<ReviewModel>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ReviewResponse>, AppError> {
    let review = Reviews::find_by_id(review.id).one(&db).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again.",
        )
    })?;

    match review {
        Some(review) => Ok(Json(ReviewResponse {
            id: review.id,
            product_id: review.product_id,
            user_id: review.user_id,
            rating: review.rating,
            review_text: review.review_text,
            created_at: review.created_at,
            updated_at: review.updated_at,
        })),
        None => Err(AppError::new(StatusCode::NOT_FOUND, "Review not found")),
    }
}

pub async fn get_all_reviews(
    State(db): State<DatabaseConnection>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<Vec<ReviewResponse>>, AppError> {
    let page = pagination.page.unwrap_or(1);
    let page_size = pagination.page_size.unwrap_or(10);

    let paginator = Reviews::find().paginate(&db, page_size);

    let total_pages = paginator.num_pages().await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again.",
        )
    })?;

    if page > total_pages {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "Page number out of range",
        ));
    }

    let reviews = paginator.fetch_page(page - 1).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again.",
        )
    })?;

    let review_responses: Vec<ReviewResponse> = reviews
        .into_iter()
        .map(|review| ReviewResponse {
            id: review.id,
            product_id: review.product_id,
            user_id: review.user_id,
            rating: review.rating,
            review_text: review.review_text,
            created_at: review.created_at,
            updated_at: review.updated_at,
        })
        .collect();

    Ok(Json(review_responses))
}

#[derive(Deserialize)]
pub struct UpdateReview {
    pub product_id: Option<i32>,
    pub user_id: Option<i32>,
    pub rating: Option<i32>,
    pub review_text: Option<String>,
}

pub async fn update_review(
    Extension(review): Extension<ReviewModel>,
    State(db): State<DatabaseConnection>,
    Json(input): Json<UpdateReview>,
) -> Result<Json<ReviewResponse>, AppError> {
    let mut review_model: reviews::ActiveModel = Reviews::find_by_id(review.id)
        .one(&db)
        .await
        .map_err(|_| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again.",
            )
        })?
        .ok_or_else(|| AppError::new(StatusCode::NOT_FOUND, "Review not found"))?
        .into();

    if let Some(product_id) = input.product_id {
        review_model.product_id = Set(Some(product_id));
    }
    if let Some(user_id) = input.user_id {
        review_model.user_id = Set(Some(user_id));
    }
    if let Some(rating) = input.rating {
        review_model.rating = Set(Some(rating));
    }
    if let Some(review_text) = input.review_text {
        review_model.review_text = Set(Some(review_text));
    }

    let review = review_model.update(&db).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again.",
        )
    })?;

    Ok(Json(ReviewResponse {
        id: review.id,
        product_id: review.product_id,
        user_id: review.user_id,
        rating: review.rating,
        review_text: review.review_text,
        created_at: review.created_at,
        updated_at: review.updated_at,
    }))
}

pub async fn delete_review(
    Extension(review): Extension<ReviewModel>,
    State(db): State<DatabaseConnection>,
) -> Result<impl IntoResponse, AppError> {
    Reviews::delete_by_id(review.id).exec(&db).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again.",
        )
    })?;

    Ok((StatusCode::OK, format!("Review with id {} deleted", review.id)))
}