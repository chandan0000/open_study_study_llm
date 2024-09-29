use crate::{
    app_state::AppState,
    utilities::{app_error::AppError, jwt::validate_token},
};
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use axum_extra::headers::{authorization::Bearer, Authorization, HeaderMapExt};

use entity::users::Entity as Users;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub async fn require_authentication(
    State(db): State<DatabaseConnection>,

    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Extract the token from the headers
    let header_token = request
        .headers()
        .get("x-auth-token")
        .and_then(|token| token.to_str().ok())
        .ok_or_else(|| AppError::new(StatusCode::UNAUTHORIZED, "Not authenticated!"))?;

    // Validate the token (your validation logic here)
    let user_id = validate_token(header_token)?;

    // Check if the user exists in the database
    let user = Users::find_by_id(user_id).one(&db).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error finding user by id",
        )
    })?;

    if let Some(user) = user {
        // Insert the user into the request's extensions so it can be accessed later
        request.extensions_mut().insert(user);
        Ok(next.run(request).await)
    } else {
        Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "Unauthorized access!",
        ))
    }
}
