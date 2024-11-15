use actix_web::{get, HttpResponse, Responder};

/// Logout a user (uses provided Bearer token from Header)
#[utoipa::path(
    responses(
        (status = 200, description = "secret endpoint")
    )
)]
#[get("/secret")]
pub async fn secret() -> impl Responder {
    HttpResponse::Ok().body("secret endpoint")
}
