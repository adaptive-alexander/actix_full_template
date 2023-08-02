use crate::prelude::*;
use mongodb::{options::ClientOptions, Client};

pub async fn get_db_pool() -> Result<Client> {
    dotenvy::dotenv().expect("Failed to read .env file");

    // --- Connect to and return client
    let client_options = ClientOptions::parse(f!(
        "mongodb://{}:{}",
        std::env::var("MDB_HOST").unwrap(),
        std::env::var("MDB_PORT").unwrap()
    ))
    .await?;

    Ok(Client::with_options(client_options)?)
}
