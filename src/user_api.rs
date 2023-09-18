use crate::{
    app_error::AppError,
    database::db,
    util::{hash_string, GenericResponse, ISO_FORMAT},
};
use axum::Json;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct UpdateScorePayload {
    username: String,
    password: String,
    newscore: u64,
}

pub async fn update_score(
    Json(payload): Json<UpdateScorePayload>,
) -> Result<Json<GenericResponse>, AppError> {
    if sqlx::query!(
        "UPDATE users SET score=? WHERE username=? AND hash=? AND score<=?",
        payload.newscore,
        payload.username,
        hash_string(payload.password),
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
            "Failed to update score: invalid credentials or not highscore"
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
) -> Result<Json<GenericResponse>, AppError> {
    sqlx::query!(
        "INSERT INTO users SET username=?, email=?, hash=?",
        payload.username,
        payload.email,
        hash_string(payload.password)
    )
    .execute(db().await)
    .await?;

    Ok(Json(GenericResponse {
        message: "User created",
    }))
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

#[derive(Deserialize)]
pub struct MePayload {
    username: String,
    password: String,
}

pub async fn me(Json(payload): Json<MePayload>) -> Result<Json<serde_json::Value>, AppError> {
    let rec = sqlx::query!(
        "SELECT * FROM users WHERE username=? AND hash=?",
        payload.username,
        hash_string(payload.password)
    )
    .fetch_one(db().await)
    .await?;

    Ok(Json(json!({
        "username": rec.username,
        "email": rec.email,
        "score": rec.score,
        "banned": rec.banned,
        "created": rec.created.format(ISO_FORMAT).to_string(),
        "modified": rec.modified.format(ISO_FORMAT).to_string()
    })))
}
