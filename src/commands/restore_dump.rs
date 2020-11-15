use mongodb::{bson::Document, Client};
use serde_json::Map;
use serde_json::Value;
use std::convert::TryFrom;
use std::fs::File;
use std::io::BufReader;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct RestoreDatabaseDump {
    /// Collection name for which dump is to be restore
    #[structopt(long = "collection")]
    collection: String,

    /// Database name to which collection belongs.
    #[structopt(long = "database", short = "db")]
    database: String,

    /// Name of backup file which is to be restore.
    #[structopt(long = "file", short = "f")]
    backup_file: String,
}

impl RestoreDatabaseDump {
    pub async fn run(&self, client: &Client) {
        let collection = client.database(&self.database).collection(&self.collection);
        let docs = self
            .read_dump()
            .into_iter()
            .map(|hashmap| Document::try_from(hashmap).unwrap());

        collection.insert_many(docs, None).await.unwrap();
    }

    fn read_dump(&self) -> Vec<Map<String, Value>> {
        let f = File::open(&self.backup_file).unwrap();
        let reader = BufReader::new(f);
        serde_json::from_reader(reader).unwrap()
    }
}
