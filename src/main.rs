use std::net::TcpListener;
use std::time::Duration;

use secrecy::ExposeSecret;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

use zero2prod::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("{}:{}", configuration.application.host, configuration.application.port);
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(Duration::from_secs(2))
        .connect_lazy(&configuration.database.connection_string().expose_secret())
        .unwrap();

    run(TcpListener::bind(address)?, connection_pool)?.await
}
