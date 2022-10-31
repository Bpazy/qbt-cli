use std::collections::HashMap;
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    peek(&cli);

    let client = reqwest::blocking::Client::new();
    let resp = client.post("http://qbittorrent.example.com/api/v2/auth/login")
        .form(&(("username", "admin"), ("password", "YOUR_PASSWORD")))
        .send()?;
    println!("{:#?}", resp.text()?);

    Ok(())
}

fn peek(cli: &Cli) {
    println!("verbose: {:?}", cli);

    match &cli.uri {
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
