use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    command: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    match cli.command.as_deref() {
        Some("hosts") => println!("TODO: implement `mkrust hosts`"),
        _ => unimplemented!(),
    }
}
