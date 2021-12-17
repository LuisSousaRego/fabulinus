use mongodb::bson::Document;
use mongodb::{options::ClientOptions, Client};
use std::env;

pub async fn get_palavras_collection() -> mongodb::error::Result<mongodb::Collection<Document>> {
    let palavras_url = env::var("MONGO_PALAVRAS").unwrap();
    let mut client_options = ClientOptions::parse(palavras_url).await?;
    client_options.app_name = Some("Palavras".to_string());
    let client = Client::with_options(client_options)?;
    let db = client.database("palavras");

    Ok(db.collection::<Document>("palavrasCollection"))
}
