use anyhow::Result;
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use dotenv::dotenv;
use lettre::{message::header::ContentType, Message, Transport};
use serde::{Deserialize, Serialize};

mod database;
use database::db;
mod mail;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt::init();
    tracing::info!("Hello world!");

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .route("/users", get(list_users))
        .route("/sleep", get(sleep))
        .route("/mail", get(sendmail));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn sendmail() -> () {
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

async fn list_users() -> String {
    let row: (String,) = sqlx::query_as("SELECT * FROM users")
        .fetch_one(db().await)
        .await
        .unwrap();
    row.0
}

// we can verify connection in async by calling this twice
async fn sleep() -> &'static str {
    let row = sqlx::query("SELECT SLEEP(1)")
        .fetch_one(db().await)
        .await
        .unwrap();
    tracing::info!("{:#?}", row);
    "slept 5"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
