use crate::writer::json_writer::JsonFileWriter;
use mongodb::Client;
use structopt::StructOpt;
mod commands;
mod writer;

#[derive(StructOpt, Debug)]
pub struct Cli {
    #[structopt(flatten)]
    config: Config,
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    /// Take dump of database
    #[structopt(name = "dump")]
    Dump(commands::take_dump::TakeDatabaseDump),
    /// Restore dump of database
    #[structopt(name = "restore")]
    Restore(commands::restore_dump::RestoreDatabaseDump),
}

#[derive(StructOpt, Debug)]
pub struct Config {
    /// Mongo connection string for connecting to database
    #[structopt(long = "connection-url", default_value = "mongodb://localhost:27017")]
    connection_string: String,
}

impl Cli {
    pub async fn run(&self) {
        let client = self.get_client().await;
        match &self.command {
            Command::Dump(take_database_dump) => {
                take_database_dump.run(&client).await;
            }
            Command::Restore(restore_database_dump) => {
                restore_database_dump.run(&client).await;
            }
        }
    }

    async fn get_client(&self) -> Client {
        Client::with_uri_str(&self.config.connection_string)
            .await
            .expect("Failed to connect")
    }
}
