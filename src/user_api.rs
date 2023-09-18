use crate::{
    app_error::AppError,
    database::db,
    util::{claims_from_cookie, encode_claims, GenericResponse, ISO_FORMAT, JWT_TTL},
};
use anyhow::anyhow;
use axum::{
    headers::Cookie,
    http::{header, HeaderMap},
    Json, TypedHeader,
};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct UpdateScorePayload {
    newscore: u64,
}

// typedheader must be before body
pub async fn update_score(
    TypedHeader(cookie): TypedHeader<Cookie>,
    Json(payload): Json<UpdateScorePayload>,
) -> Result<Json<GenericResponse>, AppError> {
    let claims = claims_from_cookie(cookie).map_err(AppError::Request)?;

    if sqlx::query!(
        "UPDATE users SET score=? WHERE username=? AND score<=?",
        payload.newscore,
        claims.sub,
        payload.newscore
    )
    .execute(db().await)
    .await?
    .rows_affected()
        == 1
    {
        Ok(Json(GenericResponse {
            message: "Score updated",
        }))
    } else {
        Err(AppError::Request(anyhow::anyhow!(
            "Failed to update score: is it really your highscore?"
        )))
    }
}

#[derive(Deserialize)]
pub struct CreateUserPayload {
    username: String,
    email: String,
    password: String,
}

pub async fn create_user(
    Json(payload): Json<CreateUserPayload>,
) -> Result<(HeaderMap, Json<GenericResponse>), AppError> {
    use argon2::{
        password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
        Argon2,
    };
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|_| anyhow::anyhow!("Failed to create hash"))?
        .to_string();

    sqlx::query!(
        "INSERT INTO users SET username=?, email=?, hash=?",
        payload.username,
        payload.email,
        password_hash,
    )
    .execute(db().await)
    .await
    .map_err(|err| match err {
        sqlx::Error::Database(db_err) => {
            if db_err.is_unique_violation() {
                AppError::Request(anyhow!("Failed to create user: {}", db_err.message()))
            } else {
                AppError::Internal(anyhow!(db_err))
            }
        }
        _ => AppError::Internal(anyhow!(err)),
    })?;

    let mut headers = HeaderMap::new();
    headers.insert(
        header::SET_COOKIE,
        format!(
            "jwt={}",
            encode_claims(payload.username, payload.email, JWT_TTL)
        )
        .parse()
        .unwrap(),
    );

    Ok((
        headers,
        Json(GenericResponse {
            message: "Created user",
        }),
    ))
}

pub async fn list_users() -> Result<Json<Vec<serde_json::Value>>, AppError> {
    let list = sqlx::query!("SELECT username,score FROM users ORDER BY score DESC LIMIT 10")
        .fetch_all(db().await)
        .await?
        .iter()
        .map(|rec| {
            json!({
                "username": rec.username,
                "score": rec.score
            })
        })
        .collect();
    Ok(Json(list))
}

pub async fn me(
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> Result<Json<serde_json::Value>, AppError> {
    let claims = claims_from_cookie(cookie).map_err(AppError::Request)?;

    let rec = sqlx::query!("SELECT * FROM users WHERE username=?", claims.sub)
        .fetch_one(db().await)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::Request(anyhow::anyhow!("Invalid credentials!")),
            ot => ot.into(),
        })?;

    Ok(Json(json!({
        "username": rec.username,
        "email": rec.email,
        "score": rec.score,
        "banned": rec.banned,
        "created": rec.created.format(ISO_FORMAT).to_string(),
        "modified": rec.modified.format(ISO_FORMAT).to_string()
    })))
}

#[derive(Deserialize)]
pub struct LoginPayload {
    username: String,
    password: String,
}

pub async fn login(
    Json(payload): Json<LoginPayload>,
) -> Result<(HeaderMap, Json<GenericResponse>), AppError> {
    use argon2::{
        password_hash::{PasswordHash, PasswordVerifier},
        Argon2,
    };

    let rec = sqlx::query!(
        "SELECT hash,username,email FROM users WHERE username=?",
        payload.username,
    )
    .fetch_one(db().await)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => AppError::Request(anyhow::anyhow!("User doesn't exist!")),
        ot => ot.into(),
    })?;

    let parsed_hash =
        PasswordHash::new(&rec.hash).map_err(|_| anyhow::anyhow!("Failed to parse hash"))?;
    Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .map_err(|_| anyhow::anyhow!("Invalid password"))?;

    let mut headers = HeaderMap::new();
    headers.insert(
        header::SET_COOKIE,
        format!("jwt={}", encode_claims(rec.username, rec.email, JWT_TTL))
            .parse()
            .unwrap(),
    );

    Ok((
        headers,
        Json(GenericResponse {
            message: "Logged in",
        }),
    ))
}
