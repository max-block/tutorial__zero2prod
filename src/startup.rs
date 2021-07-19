use std::net::TcpListener;

use actix_web::{App, dev::Server, HttpServer, web};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tracing_actix_web::TracingLogger;

use crate::configuration::{DatabaseSettings, Settings};
use crate::email_client::EmailClient;
use crate::routes::{confirm, health_check, subscribe};

pub struct Application {
    port: u16,
    server: Server,
}

pub struct ApplicationBaseUrl(pub String);

impl Application {
    pub async fn build(conf: Settings) -> Result<Self, std::io::Error> {
        let db_pool = get_connection_pool(&conf.database).await.expect("Failed to connect to Postgres");
        let sender_email = conf.email_client.sender().expect("Invalid sender email address.");
        let email_client = EmailClient::new(conf.email_client.base_url, sender_email, conf.email_client.authorization_token);
        let address = format!("{}:{}", conf.application.host, conf.application.port);
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, db_pool, email_client, conf.application.base_url)?;
        Ok(Application { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub async fn get_connection_pool(conf: &DatabaseSettings) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new().connect_timeout(std::time::Duration::from_secs(2)).connect_with(conf.with_db()).await
}

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
    base_url: String,
) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let email_client = web::Data::new(email_client);
    let base_url = web::Data::new(base_url);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .route("/subscriptions/confirm", web::get().to(confirm))
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
            .app_data(base_url.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
