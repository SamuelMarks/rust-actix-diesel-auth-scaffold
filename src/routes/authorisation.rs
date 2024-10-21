use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct AuthRequest {
    response_type: String,
    client_id: String,
    redirect_uri: String,
    state: Option<String>,
}

#[get("/authorize")]
async fn authorise(_query: web::Query<AuthRequest>) -> impl Responder {
    // Validate client_id and redirect_uri
    // Generate authorization code
    // Redirect back to the client with the authorization code
    HttpResponse::Ok().body("Authorization endpoint")
}
