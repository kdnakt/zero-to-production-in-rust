use actix_web::{http::header::ContentType, HttpResponse};

pub async fn issue_newsletter_form() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!("newsletter issue form"))
}
