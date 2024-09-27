use actix_web::{http::header::ContentType, web, HttpResponse};

pub async fn login_form(query: web::Query<QueryParams>) -> HttpResponse {
    let _error = query.0.error;
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("login.html"))
}

#[derive(serde::Deserialize)]
pub struct QueryParams {
    error: Option<String>,
}
