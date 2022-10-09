use crate::mackerelclient::new::new;
use clap::Parser;
use std::env::VarError;

mod hosts;
mod mackerelclient;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    command: Option<String>,
}

#[async_std::main]
async fn main() -> Result<(), VarError> {
    let client = match new() {
        Ok(client) => client,
        Err(e) => return Err(e),
    };

    let cli = Cli::parse();

    match cli.command.as_deref() {
        Some("hosts") => Ok(hosts::app::find_hosts(client).await),
        _ => Ok(println!("TODO: implement other command")),
    }
}
