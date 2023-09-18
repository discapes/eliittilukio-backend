use core::panic;

use axum::response::IntoResponse;
use axum::{
    http::StatusCode,
    response::Response,
    routing::{get, post},
    Json, Router,
};
use dotenv::dotenv;
use lettre::{message::header::ContentType, Message, Transport};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::Digest;

mod database;
use database::db;
mod mail;
mod user;

enum AppError {
    Internal(anyhow::Error),
    Request(anyhow::Error),
}

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Internal(err) => {
                tracing::error!("{}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Something went wrong."),
                )
                    .into_response()
            }
            AppError::Request(err) => {
                tracing::info!("{}", err);
                (StatusCode::BAD_REQUEST, format!("Bad request: {err}")).into_response()
            }
        }
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self::Internal(err.into())
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();

    tracing_subscriber::fmt::init();
    tracing::info!("Hello world!");

    let app = Router::new()
        .route("/", get(root))
        .route("/create_user", post(create_user))
        .route("/update_score", post(update_score))
        .route("/list_users", get(list_users))
        .route("/sleep", get(sleep))
        .route("/send_mail", get(send_mail));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn send_mail() -> () {
    let email = Message::builder()
        .from("Oispaeliitti <noreply@eliittilukio.fi>".parse().unwrap())
        .to("asdf <asdf>".parse().unwrap())
        .subject("Test email")
        .header(ContentType::TEXT_PLAIN)
        .body("Hi this a test mail".to_string())
        .unwrap();
    match mail::mailer().await.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}

async fn list_users() -> Result<Json<Vec<serde_json::Value>>, AppError> {
    Ok(Json(
        sqlx::query!("SELECT username,score FROM users ORDER BY score DESC LIMIT 10")
            .fetch_all(db().await)
            .await?
            .iter()
            .map(|rec| {
                json!({
                    "username": rec.username,
                    "score": rec.score
                })
            })
            .collect(),
    ))
}

// we can verify connection in async by calling this twice
async fn sleep() -> Result<(), AppError> {
    sqlx::query("SELECT SLEEP(1)")
        .fetch_one(db().await)
        .await
        .map(|_| ())
        .map_err(AppError::from)
}

async fn create_user(Json(payload): Json<CreateUser>) -> Result<Json<GenericResponse>, AppError> {
    sqlx::query("INSERT INTO users SET username=?, email=?, hash=?")
        .bind(payload.username)
        .bind(payload.email)
        .bind(hash_string(payload.password))
        .execute(db().await)
        .await?;

    Ok(Json(GenericResponse {
        message: "User created",
    }))
}

fn hash_string<T: AsRef<[u8]>>(input: T) -> Vec<u8> {
    let mut hasher = sha2::Sha256::new();
    hasher.update(input);
    let hash = hasher.finalize().to_vec();
    hash
}

async fn update_score(Json(payload): Json<UpdateScore>) -> Result<Json<GenericResponse>, AppError> {
    if sqlx::query(
        "UPDATE users SET score=?
    WHERE username=? AND hash=? AND score<=?",
    )
    .bind(payload.newscore)
    .bind(payload.username)
    .bind(hash_string(payload.password))
    .bind(payload.newscore)
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
struct CreateUser {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct UpdateScore {
    username: String,
    password: String,
    newscore: u64,
}

#[derive(Serialize)]
struct GenericResponse {
    message: &'static str,
}
