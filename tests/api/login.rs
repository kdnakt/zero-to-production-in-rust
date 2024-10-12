use std::collections::HashSet;

use wiremock::http::HeaderValue;

use crate::helpers::{assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn an_error_flash_message_is_set_on_failure() {
    let app = spawn_app().await;

    let login_body = serde_json::json!({
        "username": "random-username",
        "password": "random-password"
    });
    let res = app.post_login(&login_body).await;

    assert_is_redirect_to(&res, "/login");

    let cookies: HashSet<_> = res.headers().get_all("Set-Cookie").into_iter().collect();
    assert!(cookies.contains(&HeaderValue::from_str("_flash=Authentication failed").unwrap()));
}
