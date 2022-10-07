use crate::mackerelclient::new::new;
use clap::Parser;

mod hosts;
mod mackerelclient;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    command: Option<String>,
}

#[async_std::main]
async fn main() {
    let client = new();

    let cli = Cli::parse();

    match cli.command.as_deref() {
        Some("hosts") => hosts::app::find_hosts(client).await,
        _ => println!("TODO: implement other command"),
    }
}
