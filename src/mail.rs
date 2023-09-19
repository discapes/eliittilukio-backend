use lettre::message::header::ContentType;
use lettre::message::{Mailbox, MessageBuilder};
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::SmtpTransport;
use std::env;
use tokio::sync::OnceCell;

static MAILER_CELL: OnceCell<SmtpTransport> = OnceCell::const_new();

pub async fn mailer() -> &'static SmtpTransport {
    MAILER_CELL
        .get_or_init(|| async {
            SmtpTransport::relay("email-smtp.eu-north-1.amazonaws.com")
                .unwrap()
                .credentials(Credentials::new(
                    env::var("SMTP_USER").unwrap(),
                    env::var("SMTP_PASS").unwrap(),
                ))
                .port(465)
                .tls(Tls::Wrapper(
                    TlsParameters::new("email-smtp.eu-north-1.amazonaws.com".to_string()).unwrap(),
                ))
                .build()
        })
        .await
}

static MSG_BUILDER_CELL: OnceCell<MessageBuilder> = OnceCell::const_new();

pub async fn get_default_message_builder() -> &'static MessageBuilder {
    MSG_BUILDER_CELL
        .get_or_init(|| async {
            MessageBuilder::new()
                .from(Mailbox::new(
                    Some("Eliittilukio.fi".to_string()),
                    "noreply@eliittilukio.fi".parse().unwrap(),
                ))
                .header(ContentType::TEXT_PLAIN)
        })
        .await
}
