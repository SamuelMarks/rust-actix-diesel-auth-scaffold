use std::pin::Pin;
use std::task::{Context, Poll};

use actix_http::body::{BodySize, MessageBody};
use actix_web::web::Bytes;
use actix_web::{get, Responder};

pub const SECRET_TEXT: &'static str = "secret endpoint";

#[derive(utoipa::ToSchema, utoipa::ToResponse, serde::Serialize, serde::Deserialize)]
#[response(
    description = "Override description for response",
    content_type = "application/text"
)]
#[response(
    example = json!(SECRET_TEXT)
)]
struct SecretText(&'static str);

impl Default for SecretText {
    fn default() -> Self {
        Self { 0: SECRET_TEXT }
    }
}

impl MessageBody for SecretText {
    type Error = std::convert::Infallible;
    fn size(&self) -> BodySize {
        BodySize::Sized(self.0.len() as u64)
    }

    fn poll_next(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        if self.0.is_empty() {
            Poll::Ready(None)
        } else {
            Poll::Ready(Some(Ok(Bytes::from_static(self.0.as_bytes()))))
        }
    }
}

impl SecretText {
    const fn const_default() -> Self {
        Self { 0: SECRET_TEXT }
    }
}

impl Into<String> for SecretText {
    fn into(self) -> String {
        self.0.to_string()
    }
}

// const DEFAULT_SECRET_TEXT: SecretText = SecretText::const_default();

/// Shows secret to authenticated user (uses provided Bearer token from Header)
#[utoipa::path(
    responses(
        (status = 200, description = "secret endpoint", body=SecretText, example=SecretText::const_default)
    ),
    security(("password"=[]))
)]
#[get("/secret")]
pub async fn secret() -> impl Responder {
    SECRET_TEXT
}
