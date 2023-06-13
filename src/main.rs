use template::prelude::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {}

#[tokio::main]
async fn main() {
    let subscriber = get_subscriber("template".into(), "info".into());
    init_subscriber(subscriber);

    dotenvy::dotenv().expect("Failed to read .env file");

    tracing::info!("Parsing args");

    let _args = Args::parse();
}
