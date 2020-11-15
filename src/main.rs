use mongo_data_dump::Cli;
use structopt::StructOpt;

#[tokio::main]
async fn main() {
    Cli::from_args().run().await;
}
