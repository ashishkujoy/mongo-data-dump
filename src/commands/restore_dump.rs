use mongodb::{bson::Document, Client};
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
        collection
            .insert_many(self.read_dump(), None)
            .await
            .unwrap();
    }

    fn read_dump(&self) -> Vec<Document> {
        let f = File::open(&self.backup_file).unwrap();
        let reader = BufReader::new(f);

        serde_json::from_reader(reader).unwrap()
    }
}
