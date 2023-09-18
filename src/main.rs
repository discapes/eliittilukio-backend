use axum::{
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use lettre::{message::header::ContentType, Message, Transport};

mod app_error;
mod database;
mod mail;
mod user_api;
mod util;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();

    tracing_subscriber::fmt::init();
    tracing::info!("Hello world!");

    let app = Router::new()
        .route("/create_user", post(user_api::create_user))
        .route("/login", post(user_api::login))
        .route("/update_score", post(user_api::update_score))
        .route("/list_users", get(user_api::list_users))
        .route("/me", get(user_api::me))
        .route("/send_mail", get(send_mail));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
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
