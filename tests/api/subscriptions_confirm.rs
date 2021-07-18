use wiremock::{Mock, ResponseTemplate};
use wiremock::matchers::{method, path};

use crate::helpers::spawn_app;

#[actix_rt::test]
async fn clicking_on_the_confirmation_link_confirms_a_subscriber() {
    // Prepare
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    Mock::given(path("/email")).and(method("POST")).respond_with(ResponseTemplate::new(200)).mount(&app.email_server).await;
    app.post_subscriptions(body.into()).await;
    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_confirmation_links(email_request);

    // Act
    reqwest::get(confirmation_links.html).await.unwrap().error_for_status().unwrap();

    // Assert
    let saved = sqlx::query!("SELECT email, name, status FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
    assert_eq!(saved.status, "confirmed");
}
