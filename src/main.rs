use std::process::exit;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    verbose: bool,

    uri: Option<String>,
}

const MAGNET_PREFIX: &str = "magnet:";

fn main() {
    let cli = Cli::parse();
    peek(&cli);

    match cli.uri {
        None => {
            println!("magnet uri is required");
            exit(0)
        }
        Some(uri) => {
            if uri.starts_with(MAGNET_PREFIX) {
                println!("url: {:?}", uri);
            } else {
                println!("magnet uri is invalid");
            }
        }
    }
}

fn peek(cli: &Cli) {
    println!("verbose: {:?}", cli);
}
