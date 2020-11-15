use mongo_data_dump::Cli;
use structopt::StructOpt;

#[tokio::main]
async fn main() {
    let cli = Cli::from_args();

    println!("{:?}", cli);
    cli.run().await;
}
