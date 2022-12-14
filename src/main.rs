use std::net::TcpListener;

use zero2prod::{configuration::get_configuration, startup::run};

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration");
    // We have removed the hard coded '8000' - it's coming from our settings
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind a port");
    // Otherwise call .await on our Server
    run(listener)?.await
}
