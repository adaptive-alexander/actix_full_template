mod logging;

use env_logger::Env;
use tracing::info;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    dotenvy::dotenv().expect("Failed to read .env file");

    info!("Parsing arguments");
    let _args = Args::parse();
}
