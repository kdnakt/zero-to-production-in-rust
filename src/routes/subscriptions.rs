use actix_web::{web, HttpResponse};

#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(_firn: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
