use sqlx::{postgres::PgPoolOptions};
use std::net::TcpListener;
use zero2prod::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subsriber, init_subscriber},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subsriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let conf = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect(&conf.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    let address = format!("{}:{}", conf.application.host, conf.application.port);
    let listener = TcpListener::bind(address).expect("Failed to bind 3000 port");
    run(listener, connection_pool)?.await
}