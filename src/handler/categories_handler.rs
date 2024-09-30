use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sea_orm::{prelude::DateTimeWithTimeZone, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};

use entity::categories::{self, Entity as Categories};
use crate::utilities::app_error::AppError;

#[derive(Serialize, Deserialize)]
pub struct CategoryResponse {
  pub  id: i32,
  pub  name: String,
  pub  description: Option<String>,
  pub  created_at: Option<DateTimeWithTimeZone>,
  pub  updated_at: Option<DateTimeWithTimeZone>,
}

#[derive(Deserialize)]
pub struct CreateCategory {
   pub name: String,
   pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateCategory {
   pub name: Option<String>,
   pub description: Option<String>,
}

pub async fn create_category(
    State(db): State<DatabaseConnection>,
    Json(input): Json<CreateCategory>,
) -> Result<Json<CategoryResponse>, AppError> {
    let new_category = categories::ActiveModel {
        name: Set(input.name),
        description: Set(input.description),
        ..Default::default()
    };

    let category = new_category.insert(&db).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again.",
        )
    })?;

    Ok(Json(CategoryResponse {
        id: category.id,
        name: category.name,
        description: category.description,
        created_at: category.created_at,
        updated_at: category.updated_at,
    }))
}

pub async fn get_category_by_id(
    State(db): State<DatabaseConnection>,
    Path(category_id): Path<i32>,
) -> Result<Json<CategoryResponse>, AppError> {
    let category = Categories::find_by_id(category_id)
        .one(&db)
        .await
        .map_err(|_| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again.",
            )
        })?;

    match category {
        Some(category) => Ok(Json(CategoryResponse {
            id: category.id,
            name: category.name,
            description: category.description,
            created_at: category.created_at,
            updated_at: category.updated_at,
        })),
        None => Err(AppError::new(StatusCode::NOT_FOUND, "Category not found")),
    }
}

pub async fn get_all_categories(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<CategoryResponse>>, AppError> {
    let categories = Categories::find()
        .all(&db)
        .await
        .map_err(|_| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again.",
            )
        })?;

    let category_responses: Vec<CategoryResponse> = categories
        .into_iter()
        .map(|category| CategoryResponse {
            id: category.id,
            name: category.name,
            description: category.description,
            created_at: category.created_at,
            updated_at: category.updated_at,
        })
        .collect();

    Ok(Json(category_responses))
}

pub async fn update_category(
    State(db): State<DatabaseConnection>,
    Path(category_id): Path<i32>,
    Json(input): Json<UpdateCategory>,
) -> Result<Json<CategoryResponse>, AppError> {
    let mut category: categories::ActiveModel = Categories::find_by_id(category_id)
        .one(&db)
        .await
        .map_err(|_| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again.",
            )
        })?
        .ok_or_else(|| AppError::new(StatusCode::NOT_FOUND, "Category not found"))?
        .into();

    if let Some(name) = input.name {
        category.name = Set(name);
    }
    // if let Some(description) = input.description {
        category.description = Set(input.description);
    // }

    let category = category.update(&db).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again.",
        )
    })?;

    Ok(Json(CategoryResponse {
        id: category.id,
        name: category.name,
        description: category.description,
        created_at: category.created_at,
        updated_at: category.updated_at,
    }))
}

pub async fn delete_category(
    State(db): State<DatabaseConnection>,
    Path(category_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    Categories::delete_by_id(category_id).exec(&db).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again.",
        )
    })?;

    Ok((StatusCode::OK, format!("Category with id {} deleted", category_id)))
}