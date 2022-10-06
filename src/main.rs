use clap::Parser;
use mackerel_client::*;
use std::env;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    command: Option<String>,
}

#[async_std::main]
async fn main() {
    let apikey = env::var("MACKEREL_API_KEY").unwrap();
    let client = Client::new(&apikey);
    println!("{:?}", client.list_hosts().await);

    let cli = Cli::parse();

    match cli.command.as_deref() {
        Some("hosts") => println!("TODO: implement `mkrust hosts`"),
        _ => println!("TODO: implement other command"),
    }
}
