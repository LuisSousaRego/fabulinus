use futures::stream::StreamExt;
use mongodb::bson::Document;
use mongodb::{options::ClientOptions, Client};
use std::env;

async fn get_palavras_collection() -> mongodb::error::Result<mongodb::Collection<Document>> {
    let palavras_url = env::var("MONGO_PALAVRAS").unwrap();
    let mut client_options = ClientOptions::parse(palavras_url).await?;
    client_options.app_name = Some("Palavras".to_string());
    let client = Client::with_options(client_options)?;
    let db = client.database("palavras");
    Ok(db.collection::<Document>("palavrasCollection"))
}

pub async fn get_question_and_answers() -> String {
    let coll = get_palavras_collection().await.unwrap();

    // let mut cursor = coll.find(None, None).await.unwrap();
    // let mut result: Vec<Document> = Vec::new();
    // while let Some(doc) = cursor.next().await {
    //     //result.push(doc.unwrap());
    //     println!("{}", doc.unwrap().to_string());
    // }

    return "test".to_string();
}
