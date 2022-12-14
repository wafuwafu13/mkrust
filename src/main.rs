use crate::error::*;
use clap::Parser;

mod error;
mod format;
mod hosts;
mod mackerelclient;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    command: Option<String>,
}

#[async_std::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command.as_deref() {
        Some("hosts") => Ok(println!("{:?}", hosts::command::do_hosts().await)),
        _ => Ok(println!("TODO: implement other command")),
    }
}
