use crate::utilities::app_error::AppError;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use entity::products::{self, Entity as Products};
use sea_orm::{
    prelude::{DateTimeWithTimeZone, Decimal},
    ActiveModelTrait, DatabaseConnection, EntityTrait, Set,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProductResponse {
   pub id: i32,
   pub name: String,
   pub description: Option<String>,
   pub price: Decimal,
   pub stock: Option<i32>,
   pub category_id: Option<i32>,
   pub image_url: Option<String>,
   pub created_at: Option<DateTimeWithTimeZone>,
   pub updated_at: Option<DateTimeWithTimeZone>,
}

#[derive(Deserialize)]
pub struct CreateProduct {
    pub name: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub stock: Option<i32>,
    pub category_id: Option<i32>,
    pub image_url: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateProduct {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<Decimal>,
    pub stock: Option<i32>,
    pub category_id: Option<i32>,
    pub image_url: Option<String>,
}

pub async fn create_product(
    State(db): State<DatabaseConnection>,
    Json(input): Json<CreateProduct>,
) -> Result<Json<ProductResponse>, AppError> {
    let new_product = products::ActiveModel {
        name: Set(input.name),
        description: Set(input.description),
        price: Set(input.price),
        stock: Set(input.stock),
        category_id: Set(input.category_id),
        image_url: Set(input.image_url),
        ..Default::default()
    };

    let product = new_product.insert(&db).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again.",
        )
    })?;

    Ok(Json(ProductResponse {
        id: product.id,
        name: product.name,
        description: product.description,
        price: product.price,
        stock: product.stock,
        category_id: product.category_id,
        image_url: product.image_url,
        created_at: product.created_at,
        updated_at: product.updated_at,
    }))
}

pub async fn get_product_by_id(
    State(db): State<DatabaseConnection>,
    Path(product_id): Path<i32>,
) -> Result<Json<ProductResponse>, AppError> {
    let product = Products::find_by_id(product_id)
        .one(&db)
        .await
        .map_err(|_| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again.",
            )
        })?;

    match product {
        Some(product) => Ok(Json(ProductResponse {
            id: product.id,
            name: product.name,
            description: product.description,
            price: product.price,
            stock: product.stock,
            category_id: product.category_id,
            image_url: product.image_url,
            created_at: product.created_at,
            updated_at: product.updated_at,
        })),
        None => Err(AppError::new(StatusCode::NOT_FOUND, "Product not found")),
    }
}

pub async fn get_all_products(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<ProductResponse>>, AppError> {
    let products = Products::find().all(&db).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again.",
        )
    })?;

    let product_responses: Vec<ProductResponse> = products
        .into_iter()
        .map(|product| ProductResponse {
            id: product.id,
            name: product.name,
            description: product.description,
            price: product.price,
            stock: product.stock,
            category_id: product.category_id,
            image_url: product.image_url,
            created_at: product.created_at,
            updated_at: product.updated_at,
        })
        .collect();

    Ok(Json(product_responses))
}

pub async fn update_product(
    State(db): State<DatabaseConnection>,
    Path(product_id): Path<i32>,
    Json(input): Json<UpdateProduct>,
) -> Result<Json<ProductResponse>, AppError> {
    let mut product: products::ActiveModel = Products::find_by_id(product_id)
        .one(&db)
        .await
        .map_err(|_| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again.",
            )
        })?
        .ok_or_else(|| AppError::new(StatusCode::NOT_FOUND, "Product not found"))?
        .into();

    if let Some(name) = input.name {
        product.name = Set(name); // Wrap in Some()
    }
    if let Some(description) = input.description {
        product.description = Set(Some(description)); // Already Option<String>
    }
    if let Some(price) = input.price {
        product.price = Set(price); // Wrap in Some()
    }
    if let Some(stock) = input.stock {
        product.stock = Set(Some(stock)); // Already Option<i32>
    }
    if let Some(category_id) = input.category_id {
        product.category_id = Set(Some(category_id)); // Already Option<i32>
    }
    if let Some(image_url) = input.image_url {
        product.image_url = Set(Some(image_url)); // Already Option<String>
    }

    let product = product.update(&db).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again.",
        )
    })?;

    Ok(Json(ProductResponse {
        id: product.id,
        name: product.name, // Unwrap since it's now Option<String>
        description: product.description,
        price: product.price, // Unwrap since it's now Option<Decimal>
        stock: product.stock,
        category_id: product.category_id,
        image_url: product.image_url,
        created_at: product.created_at,
        updated_at: product.updated_at,
    }))
}

pub async fn delete_product(
    State(db): State<DatabaseConnection>,
    Path(product_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    Products::delete_by_id(product_id)
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
        format!("Product with id {} deleted", product_id),
    ))
}
