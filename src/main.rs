use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemtry::{get_subscriber, init_sbuscriber};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // We are falling back to printing all spans at info-level or above
    // if the RUST_LOG environment variable has not been set
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_sbuscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    run(listener, connection_pool)?.await
}
