use std::net::TcpListener;

use sqlx::postgres::PgPoolOptions;

use zero2prod::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};
use zero2prod::email_client::EmailClient;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let conf = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_with(conf.database.with_db())
        .await
        .expect("Failed to connect to Postgres");
    let sender_email = conf.email_client.sender().expect("Invalid sender email address.");
    let email_client = EmailClient::new(conf.email_client.base_url, sender_email, conf.email_client.authorization_token);
    let address = format!("{}:{}", conf.application.host, conf.application.port);
    let listener = TcpListener::bind(address).expect("Failed to bind 3000 port");
    run(listener, connection_pool, email_client)?.await
}
