use crate::{
    app_state::AppState,
    mail::mails::{send_forgot_password_email, send_verification_email, send_welcome_email},
    utilities::{
        app_error::AppError,
        hash::{hash_password, verify_password},
        jwt::create_token,
    },
};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::{Duration, Utc};
use dotenvy_macro::dotenv;
use entity::users::{self, Entity as Users};
use entity::{
    password_reset_tokens::{self, Entity as PasswordResetTokens},
    token_verifcation::{self, Entity as TokenVerifcation},
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct CustomResponse {
    status: String,
    message: String,
}

#[derive(serde::Deserialize)]
pub struct UserRegistationData {
    full_name: String,
    email: String,
    password: String,
}

#[derive(serde::Deserialize)]
pub struct VerifyToken {
    pub token: String,
}

#[derive(serde::Serialize)]
struct UserResponse {
    full_name: String,
    email: String,
    x_auth_token: String,
}

// Registration Handler
pub async fn sign_up(
    State(app_state): State<AppState>,
    Json(data): Json<UserRegistationData>,
) -> Result<impl IntoResponse, AppError> {
    let db: &DatabaseConnection = &app_state.database;

    // Hash the password and create a new user
    let user_active = users::ActiveModel {
        fullname: Set(data.full_name),
        email_id: Set(data.email),
        password: Set(hash_password(&data.password)?),
        ..Default::default()
    };

    let user = user_active.save(db).await.map_err(|error| {
        let error_message = error.to_string();
        if error_message.contains("duplicate key value violates unique constraint") {
            AppError::new(
                StatusCode::BAD_REQUEST,
                "Email already taken, try again with a different one.",
            )
        } else {
            eprintln!("Error creating user: {:?}", error_message);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again.",
            )
        }
    })?;

    // Create a verification token
    let verification_token = uuid::Uuid::new_v4().to_string();
    let expires_at = Utc::now() + Duration::hours(24);

    // Save the verification token in the database
    token_verifcation::ActiveModel {
        user_id: Set(user.id.unwrap()),
        token: Set(verification_token.clone()),
        expires_at: Set(expires_at.naive_local()),
        ..Default::default()
    }
    .save(db)
    .await
    .map_err(|error| {
        eprintln!("Error saving verification token: {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again.",
        )
    })?;

    // Send verification email
    let send_email = send_verification_email(
        &user.email_id.clone().unwrap(),
        &user.fullname.clone().unwrap(),
        &verification_token,
    )
    .await;

    if send_email.is_err() {
        eprintln!("Error sending verification email: {:?}", send_email.err());
        return Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again.",
        ));
    }

    Ok((
        StatusCode::CREATED,
        Json(CustomResponse {
            status: "success".to_string(),
            message: "Registration successful! Check your email to verify your account."
                .to_string(),
        }),
    ))
}
pub async fn verify_email(
    State(app_state): State<AppState>,
    Query(data): Query<VerifyToken>,
) -> Result<impl IntoResponse, AppError> {
    let db: &DatabaseConnection = &app_state.database;

    // Find the token in the database
    let token = TokenVerifcation::find()
        .filter(token_verifcation::Column::Token.eq(data.token.clone()))
        .one(db)
        .await
        .map_err(|error| {
            eprintln!("Error finding token: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, try again.",
            )
        })?;

    if let Some(token) = token {
        // Check if the token has expired
        if token.expires_at < Utc::now().naive_local() {
            return Err(AppError::new(
                StatusCode::BAD_REQUEST,
                "Token has expired, please request a new one.",
            ));
        }

        // Find the associated user
        let user = Users::find()
            .filter(users::Column::Id.eq(token.user_id))
            .one(db)
            .await
            .map_err(|error| {
                eprintln!("Error finding user: {:?}", error);
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong, try again.",
                )
            })?;

        if let Some(user) = user {
            // Check if the user is already verified
            if user.is_verdified.unwrap_or(false) {
                return Ok((
                    StatusCode::OK,
                    Json(CustomResponse {
                        status: "info".to_string(),
                        message: "You are already verified. Please log in.".to_string(),
                    }),
                ));
            }

            // Update user verification status
            let mut active_user = user.into_active_model();
            active_user.is_verdified = Set(Some(true));

            // Extract email_id and fullname before updating active_user
            let email_id = active_user.email_id.clone().unwrap();
            let fullname = active_user.fullname.clone().unwrap();

            active_user.update(db).await.map_err(|error| {
                eprintln!("Error updating user: {:?}", error);
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong, try again.",
                )
            })?;

            // Now use the extracted values to send the welcome email
            send_welcome_email(&email_id, &fullname).await;

            Ok((
                StatusCode::OK,
                Json(CustomResponse {
                    status: "success".to_string(),
                    message: "Email verified successfully!".to_string(),
                }),
            ))
        } else {
            Err(AppError::new(
                StatusCode::BAD_REQUEST,
                "Invalid user, try again.",
            ))
        }
    } else {
        Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "Invalid token, try again.",
        ))
    }
}

// Login handler
pub async fn login(
    State(app_state): State<AppState>,
    Json(data): Json<UserRegistationData>,
) -> Result<impl IntoResponse, AppError> {
    let db: &DatabaseConnection = &app_state.database;

    // Find the user in the database
    let user = Users::find()
        .filter(users::Column::EmailId.eq(data.email.clone()))
        .one(db)
        .await
        .map_err(|error| {
            eprintln!("Error finding user: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, try again.",
            )
        })?;

    if let Some(user) = user {
        // Check if the user is verified
        if !user.is_verdified.unwrap() {
            // Create a verification token
            let verification_token = uuid::Uuid::new_v4().to_string();
            let expires_at = Utc::now() + Duration::hours(24);

            // Save the verification token in the database
            token_verifcation::ActiveModel {
                user_id: Set(user.id),
                token: Set(verification_token.clone()),
                expires_at: Set(expires_at.naive_local()),
                ..Default::default()
            }
            .save(db)
            .await
            .map_err(|error| {
                eprintln!("Error saving verification token: {:?}", error);
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong, please try again.",
                )
            })?;

            // Send verification email
            let send_email = send_verification_email(
                &user.email_id.clone(),
                &user.fullname.clone(),
                &verification_token,
            )
            .await;

            if send_email.is_err() {
                eprintln!("Error sending verification email: {:?}", send_email.err());
                return Err(AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong, please try again.",
                ));
            }

            return Err(AppError::new(
                StatusCode::BAD_REQUEST,
                "Account not verified, check your email to verify.",
            ));
        }

        // Check if the password is correct
        if verify_password(&data.password, &user.password.clone()).unwrap_or(false) {
            return Err(AppError::new(
                StatusCode::BAD_REQUEST,
                "Invalid password, try again.",
            ));
        }

        // Create token
        let token = create_token(user.id)?;

        Ok((
            StatusCode::OK,
            Json(UserResponse {
                full_name: user.fullname.clone(),
                email: user.email_id.clone(),
                x_auth_token: token,
            }),
        ))
    } else {
        Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "Invalid user, try again.",
        ))
    }
}

// create forget password handler check user verify or not if verify then send email with reset password link

#[derive(Deserialize)]
pub struct ForgetPasswordData {
    pub email: String,
}

pub async fn forget_password(
    State(app_state): State<AppState>,
    Json(data): Json<ForgetPasswordData>,
) -> Result<impl IntoResponse, AppError> {
    let db: &DatabaseConnection = &app_state.database;

    // Check if the user exists and is verified
    let user = Users::find()
        .filter(users::Column::EmailId.eq(data.email.clone()))
        .one(db)
        .await
        .map_err(|error| {
            eprintln!("Error finding user: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again.",
            )
        })?;

    if let Some(user) = user {
        // Check if the user is verified
        if user.is_verdified.unwrap_or(false) {
            // Generate a reset password token
            let reset_token = uuid::Uuid::new_v4().to_string();
            let expires_at = Utc::now() + chrono::Duration::hours(1); // Token valid for 1 hour

            // Save the reset token in the database
            let new_token = password_reset_tokens::ActiveModel {
                user_id: Set(user.id),
                token: Set(reset_token.clone()),
                expires_at: Set(expires_at.naive_utc()),
                ..Default::default()
            };

            new_token.insert(db).await.map_err(|error| {
                eprintln!("Error saving reset token: {:?}", error);
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong, please try again.",
                )
            })?;

            // Create the reset password link (you can use your frontend URL here)
            let reset_link = format!(
                "{}/reset-password?token={}",
                dotenv!("SERVER_HOST_URL"),
                reset_token
            );

            // Send email with reset password link
            send_forgot_password_email(&user.email_id, &reset_link, &user.fullname);

            Ok((
                StatusCode::OK,
                Json(CustomResponse {
                    status: "success".to_string(),
                    message: "Reset password link has been sent to your email.".to_string(),
                }),
            ))
        } else {
            Err(AppError::new(
                StatusCode::BAD_REQUEST,
                "User is not verified.",
            ))
        }
    } else {
        Err(AppError::new(StatusCode::BAD_REQUEST, "User not found."))
    }
}

// reset password handler and check user verify or not then password reset successfully
#[derive(Deserialize)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub new_password: String,
}

pub async fn reset_password(
    State(app_state): State<AppState>,
    Json(data): Json<ResetPasswordRequest>,
) -> Result<impl IntoResponse, AppError> {
    let db: &DatabaseConnection = &app_state.database;

    // Step 1: Find the token in the database
    let token_entry = PasswordResetTokens::find()
        .filter(password_reset_tokens::Column::Token.eq(data.token))
        .one(db)
        .await
        .map_err(|error| {
            eprintln!("Error finding reset token: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, try again.",
            )
        })?;

    if let Some(token_entry) = token_entry {
        // Step 2: Check if the token has expired
        if token_entry.expires_at < Utc::now().naive_utc() {
            return Err(AppError::new(StatusCode::BAD_REQUEST, "Token has expired."));
        }

        // Step 3: Find the user associated with the token
        let user = Users::find_by_id(token_entry.user_id)
            .one(db)
            .await
            .map_err(|error| {
                eprintln!("Error finding user: {:?}", error);
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong, try again.",
                )
            })?;

        if let Some(user) = user {
            // Step 4: Hash the new password
            let hashed_password = hash_password(&data.new_password)?;

            // Step 5: Update the user's password
            let mut active_user = user.into_active_model();
            active_user.password = Set(hashed_password);

            active_user.update(db).await.map_err(|error| {
                eprintln!("Error updating password: {:?}", error);
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong, try again.",
                )
            })?;

            Ok((
                StatusCode::OK,
                Json(CustomResponse {
                    status: "success".to_string(),
                    message: "Password reset successfully.".to_string(),
                }),
            ))
        } else {
            Err(AppError::new(StatusCode::BAD_REQUEST, "User not found."))
        }
    } else {
        Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "Invalid or expired token.",
        ))
    }
}
