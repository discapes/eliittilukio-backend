use axum::{
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

mod app_error;
mod database;
mod mail;
mod user_api;
mod util;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    tracing::info!("Hello world!");

    let app = Router::new()
        .route("/create_user", post(user_api::create_user))
        .route("/login", post(user_api::login))
        .route("/update_score", post(user_api::update_score))
        .route("/send_feedback", post(user_api::send_feedback))
        .route("/list_users", get(user_api::list_users))
        .route("/average_score", get(user_api::average_score))
        .route("/me", get(user_api::me))
        .route("/logout", post(user_api::logout))
        .layer(
            ServiceBuilder::new().layer(TraceLayer::new_for_http()), //  .layer(CorsLayer::new().allow_origin("localhost".parse::<HeaderValue>().unwrap())),
        );

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
