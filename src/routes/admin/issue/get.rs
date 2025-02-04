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
    let idempotency_key = uuid::Uuid::new_v4();
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"{html}
            newsletter issue form
            <form action="/admin/newsletters" method="post">
                <input type="text" name="title" value="">
                <input hidden type="text" name="idempotency_key" value="{idempotency_key}">
                <button type="submit">Publish</button>
            </form>
            "#,
        ))
}
