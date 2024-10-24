use actix_web::{get, HttpResponse, Responder};

#[get("/secret")]
pub async fn secret() -> impl Responder {
    HttpResponse::Ok().body("secret endpoint")
}
