use template::logging::{get_subscriber,init_subscriber};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {}

fn main() {
    let subscriber = get_subscriber("template".into(), "info".into());
    init_subscriber(subscriber);

    dotenvy::dotenv().expect("Failed to read .env file");

    tracing::info!("Parsing args");

    let _args = Args::parse();
}
