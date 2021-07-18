use reqwest::Url;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

use crate::helpers::spawn_app;

#[actix_rt::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    Mock::given(path("/email")).and(method("POST")).respond_with(ResponseTemplate::new(200)).mount(&app.email_server).await;

    let res = app.post_subscriptions(body.into()).await;
    assert_eq!(200, res.status().as_u16());
}

#[actix_rt::test]
async fn subscribe_persists_the_new_subscriber() {
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    app.post_subscriptions(body.into()).await;

    let saved = sqlx::query!("SELECT email, name, status FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
    assert_eq!(saved.status, "pending_confirmation");
}

#[actix_rt::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let res = app.post_subscriptions(invalid_body.into()).await;
        assert_eq!(
            400,
            res.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        )
    }
}

#[actix_rt::test]
async fn subscribe_returns_a_400_when_fields_are_present_but_invalid() {
    let app = spawn_app().await;
    let test_cases = vec![
        ("name=le%20guin&email=", "empty email"),
        ("name=&email=ursula_le_guin%40gmail.com", "empty name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let res = app.post_subscriptions(invalid_body.into()).await;
        assert_eq!(
            400,
            res.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        )
    }
}

#[actix_rt::test]
async fn subscribe_sends_a_confirmation_email_for_valida_data() {
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    Mock::given(path("/email")).and(method("POST")).respond_with(ResponseTemplate::new(200)).mount(&app.email_server).await;

    app.post_subscriptions(body.into()).await;

    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_confirmation_links(email_request);
    assert_eq!(confirmation_links.html, confirmation_links.plain_text);
}

#[actix_rt::test]
async fn confirmations_without_token_are_rejected_with_a_400() {
    let app = spawn_app().await;
    let res = reqwest::get(&format!("{}/subscriptions/confirm", app.address)).await.unwrap();
    assert_eq!(res.status().as_u16(), 400);
}

#[actix_rt::test]
async fn the_link_returned_by_subscribe_returns_a_200_if_called() {
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    Mock::given(path("/email")).and(method("POST")).respond_with(ResponseTemplate::new(200)).mount(&app.email_server).await;
    app.post_subscriptions(body.into()).await;

    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_confirmation_links(email_request);
    let res = reqwest::get(confirmation_links.html).await.unwrap();
    assert_eq!(res.status().as_u16(), 200);
}
