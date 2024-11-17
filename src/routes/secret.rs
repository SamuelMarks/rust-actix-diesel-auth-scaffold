use actix_web::{get, HttpResponse, Responder};

/// Shows secret to authenticated user (uses provided Bearer token from Header)
#[utoipa::path(
    responses(
        (status = 200, description = "secret endpoint")
    ),
    security(("password"=[]))
)]
#[get("/secret")]
pub async fn secret() -> impl Responder {
    HttpResponse::Ok().body("secret endpoint")
}
