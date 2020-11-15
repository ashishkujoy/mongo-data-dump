use std::{collections::HashMap, fs::File};

use futures::stream::StreamExt;
use mongodb::{bson::Bson, Client};
use serde_json;
use std::io::prelude::*;
use structopt::StructOpt;

#[tokio::main]
async fn main() {
    let cli = Cli::from_args();

    println!("{:?}", cli);
    cli.run().await;
}

#[derive(Debug, StructOpt)]
struct Cli {
    /// Mongo connection string for connecting to database
    #[structopt(long = "connection-url", default_value = "mongodb://localhost:27017")]
    connection_string: String,

    /// Collection name for which dump is required
    #[structopt(long = "collection")]
    collection: Option<String>,

    /// Database name from collection is to be read.
    #[structopt(long = "database", short = "db")]
    database: String,
}

impl Cli {
    pub async fn run(&self) {
        let client = Client::with_uri_str(&self.connection_string)
            .await
            .expect("Failed to connect");
        let collection_names = self.get_collection_names(&client).await;
        self.create_dump(&client, collection_names).await;
        // let mut cursor = collection
        //     .find(None, None)
        //     .await
        //     .expect("Failed to read from collection");

        // let mut file = File::create("data-dump.json").unwrap();
        // file.write(b"[").unwrap();
        // let mut not_first_entry = false;
        // while let Some(Ok(doc)) = cursor.next().await {
        //     if not_first_entry {
        //         file.write(b",").unwrap();
        //     }
        //     serde_json::to_writer_pretty(&file, &doc).unwrap();
        //     not_first_entry = true;
        // }
        // file.write(b"]").unwrap();
    }

    async fn get_collection_names(&self, client: &Client) -> Vec<String> {
        match &self.collection {
            Some(name) => vec![name.clone()],
            None => client
                .database(&self.database)
                .list_collection_names(None)
                .await
                .unwrap(),
        }
    }

    async fn create_dump(&self, client: &Client, collection_names: Vec<String>) {
        for collection_name in collection_names {
            let collection = client.database(&self.database).collection(&collection_name);

            let mut cursor = collection
                .find(None, None)
                .await
                .expect("Failed to read from collection");

            let mut file = File::create(format!("{}.json", &collection_name)).unwrap();
            file.write(b"[").unwrap();

            let mut not_first_entry = false;
            while let Some(Ok(doc)) = cursor.next().await {
                if not_first_entry {
                    file.write(b",").unwrap();
                }
                serde_json::to_writer_pretty(&file, &doc).unwrap();
                not_first_entry = true;
            }
            file.write(b"]").unwrap();
        }
    }
}
