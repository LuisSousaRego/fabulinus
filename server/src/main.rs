mod mongo;

use crate::mongo::*;

#[tokio::main]
async fn main() {
    let c = get_palavras_collection().await.unwrap();
    println!("{:#?}", c);
}
