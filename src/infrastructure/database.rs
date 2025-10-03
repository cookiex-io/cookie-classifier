use std::env;
use mongodb::{options::ClientOptions, Client};

pub async fn get_client() -> mongodb::error::Result<Client> {
    let db_uri = env::var("MONGODB_VCORE_URI").expect("MONGODB_VCORE_URI must be set");
    let client_options = ClientOptions::parse(db_uri).await?;
    let client = Client::with_options(client_options).expect("Failed to connect to MongoDB");
    Ok(client)
}