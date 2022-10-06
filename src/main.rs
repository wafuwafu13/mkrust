use crate::mackerelclient::new::new;
use clap::Parser;

mod mackerelclient;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    command: Option<String>,
}

#[async_std::main]
async fn main() {
    let client = new();
    println!("{:?}", client.list_hosts().await);

    let cli = Cli::parse();

    match cli.command.as_deref() {
        Some("hosts") => println!("TODO: implement `mkrust hosts`"),
        _ => println!("TODO: implement other command"),
    }
}
