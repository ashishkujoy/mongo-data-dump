use futures::stream::StreamExt;
use mongodb::Client;
use std::{fs::File, io::Write};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
    /// Mongo connection string for connecting to database
    #[structopt(long = "connection-url", default_value = "mongodb://localhost:27017")]
    connection_string: String,

    /// Collection name for which dump is required, no value is provided
    /// data dump will be taken for all collections.
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
