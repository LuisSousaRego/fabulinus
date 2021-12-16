use mongodb::{options::ClientOptions, Client};
use mongodb::bson::{Document};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {

    let palavras_url: &'static str = env!("MONGO_PALAVRAS","MONGO_URL env var missing");
    // Parse your connection string into an options struct
    let mut client_options =
        ClientOptions::parse(palavras_url)
            .await?;
    // Manually set an option
    client_options.app_name = Some("Palavras".to_string());
    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;
    let db = client.database("palavras");


    let collection = db.collection::<Document>("palavrasCollection");
    

    Ok(())
}
