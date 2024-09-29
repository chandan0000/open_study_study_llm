use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, PaginatorTrait,
    QueryFilter, Set,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use entity::users;
use entity::users::Entity as Users;

use crate::utilities::app_error::AppError;

#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    pub id: i32,
    pub uuid: Uuid,
    pub fullname: String,
    pub email_id: String,
    pub profile_pic: Option<String>,
    pub github_link: Option<String>,
    pub linkdin_link: Option<String>,
    pub is_verdified: Option<bool>,
}

pub async fn get_user_by_id(
    State(db): State<DatabaseConnection>,
    Path(user_id): Path<i32>,
) -> Result<Json<UserResponse>, AppError> {
    let user = Users::find_by_id(user_id).one(&db).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again.",
        )
    })?;

    match user {
        Some(user) => Ok(Json(UserResponse {
            id: user.id,
            uuid: user.uuid,
            fullname: user.fullname,
            email_id: user.email_id,
            profile_pic: user.profile_pic,
            github_link: user.github_link,
            linkdin_link: user.linkdin_link,
            is_verdified: user.is_verdified,
        })),
        None => Err(AppError::new(StatusCode::NOT_FOUND, "User not found")),
    }
}

// async fn get_all_users(
//     State(db): State<DatabaseConnection>,
// ) -> Result<Json<Vec<UserResponse>>, AppError> {
//     let users = Users::find().all(&db).await.map_err(|_| {
//         AppError::new(
//             StatusCode::INTERNAL_SERVER_ERROR,
//             "Something went wrong, please try again.",
//         )
//     })?;

//     let user_responses: Vec<UserResponse> = users
//         .into_iter()
//         .map(|user| UserResponse {
//             id: user.id,
//             uuid: user.uuid,
//             fullname: user.fullname,
//             email_id: user.email_id,
//             profile_pic: user.profile_pic,
//             github_link: user.github_link,
//             linkdin_link: user.linkdin_link,
//             is_verdified: user.is_verdified,
//         })
//         .collect();

//     Ok(Json(user_responses))
// }

pub async fn get_all_users(
    State(db): State<DatabaseConnection>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<Vec<UserResponse>>, AppError> {
    let page = pagination.page.unwrap_or(1);
    let page_size = pagination.page_size.unwrap_or(10);

    let paginator = Users::find().paginate(&db, page_size);

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

    let users = paginator.fetch_page(page - 1).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again.",
        )
    })?;

    let user_responses: Vec<UserResponse> = users
        .into_iter()
        .map(|user| UserResponse {
            id: user.id,
            uuid: user.uuid,
            fullname: user.fullname,
            email_id: user.email_id,
            profile_pic: user.profile_pic,
            github_link: user.github_link,
            linkdin_link: user.linkdin_link,
            is_verdified: user.is_verdified,
        })
        .collect();

    Ok(Json(user_responses))
}

#[derive(Deserialize)]
pub struct UpdateUser {
    pub fullname: Option<String>,
    pub email_id: Option<String>,
    pub profile_pic: Option<String>,
    pub github_link: Option<String>,
    pub linkdin_link: Option<String>,
    pub is_verdified: Option<bool>,
}

pub async fn update_user(
    State(db): State<DatabaseConnection>,
    Path(user_id): Path<i32>,
    Json(user): Json<UpdateUser>,
) -> Result<Json<UserResponse>, AppError> {
    let mut user_model: users::ActiveModel = Users::find_by_id(user_id)
        .one(&db)
        .await
        .map_err(|_| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again.",
            )
        })?
        .ok_or_else(|| AppError::new(StatusCode::NOT_FOUND, "User not found"))?
        .into();

    if let Some(fullname) = &user.fullname {
        user_model.fullname = Set(fullname.clone());
    }
    if let Some(email_id) = &user.email_id {
        user_model.email_id = Set(email_id.clone());
    }
    if let Some(profile_pic) = &user.profile_pic {
        user_model.profile_pic = Set(Some(profile_pic.clone()));
    }
    if let Some(github_link) = &user.github_link {
        user_model.github_link = Set(Some(github_link.clone()));
    }
    if let Some(linkdin_link) = &user.linkdin_link {
        user_model.linkdin_link = Set(Some(linkdin_link.clone()));
    }
    if let Some(is_verdified) = user.is_verdified {
        user_model.is_verdified = Set(Some(is_verdified));
    }

    let user = user_model.update(&db).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again.",
        )
    })?;

    Ok(Json(UserResponse {
        id: user.id,
        uuid: user.uuid,
        fullname: user.fullname,
        email_id: user.email_id,
        profile_pic: user.profile_pic,
        github_link: user.github_link,
        linkdin_link: user.linkdin_link,
        is_verdified: user.is_verdified,
    }))
}

pub async fn delete_user(
    State(db): State<DatabaseConnection>,
    Path(user_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    Users::delete_by_id(user_id).exec(&db).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again.",
        )
    })?;

    Ok((StatusCode::OK, format!("User with id {} deleted", user_id)))
}
