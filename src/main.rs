mod logging;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    dotenvy::dotenv().expect("Failed to read .env file");

    tracing::info!("Parsing arguments");
    let _args = Args::parse();
}
