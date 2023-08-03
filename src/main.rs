use tracing::log::info;
use template::prelude::f;
use template::run;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    info!("Server starting on port {}", port);

    let listener = std::net::TcpListener::bind(f!("127.0.0.1:{}", port))?;
    run(listener)?.await
}
