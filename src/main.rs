use clap::Parser;
use template::run;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run().await
}
