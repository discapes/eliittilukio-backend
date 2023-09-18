use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum AppError {
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
