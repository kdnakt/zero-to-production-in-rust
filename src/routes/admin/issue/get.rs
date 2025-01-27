use actix_web::{http::header::ContentType, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn issue_newsletter_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let mut html = String::new();
    for m in flash_messages.iter() {
        writeln!(
            html,
            "<p><i>{}</i></p>",
            htmlescape::encode_minimal(m.content())
        )
        .unwrap();
    }
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!("{html}newsletter issue form"))
}
