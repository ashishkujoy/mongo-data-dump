use chrono::Utc;
use indicatif::ProgressStyle;
use indicatif::ProgressBar;
use crate::Client;
use crate::JsonFileWriter;
use structopt::StructOpt;
use tokio::stream::StreamExt;

#[derive(Debug, StructOpt)]
pub struct TakeDatabaseDump {
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

impl TakeDatabaseDump {
    pub async fn run(&self, client: &Client) {
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
            println!("Taking dump of {}", collection_name);
            let collection = client.database(&self.database).collection(&collection_name);
            let count = collection.count_documents(None, None).await.unwrap();
            let doc_progress = ProgressBar::new(count as u64)
                .with_style(ProgressStyle::default_bar().progress_chars(&"=>"));
            let mut cursor = collection
                .find(None, None)
                .await
                .expect("Failed to read from collection");

            let mut file_writer = JsonFileWriter::new(self.get_file_name(&collection_name));
            while let Some(Ok(doc)) = cursor.next().await {
                file_writer.write(&doc);
                doc_progress.inc(1);
            }
            doc_progress.finish_and_clear();
        }
    }

    fn get_file_name(&self, collection_name: &String) -> String {
        format!("{}_{}.json", collection_name, Utc::now().timestamp_millis())
    }
}
