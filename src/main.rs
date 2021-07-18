use zero2prod::startup::Application;
use zero2prod::{
    configuration::get_configuration,
    telemetry::{get_subscriber, init_subscriber},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let conf = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(conf).await?;
    application.run_until_stopped().await?;
    Ok(())
}
