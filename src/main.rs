use std::error::Error;

use crate::cli::{Add, Cli, Commands};
use crate::config::AquConfig;

mod cli;
mod config;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::load();
    match &cli.command {
        Commands::Add(add) => {
            add_magnet(AquConfig::load(), add);
        }
    }
    Ok(())
}

fn add_magnet(config: AquConfig, add: &Add) {
    let client = reqwest::blocking::Client::builder().cookie_store(true).build().unwrap();
    let resp = client.post(&config.get_login_url())
        .form(&(("username", &config.username), ("password", &config.password)))
        .send()
        .expect(format!("Login failed: {}", &config.get_login_url()).as_str());
    println!("Login result: {:#?}", resp.text().unwrap());

    let resp = client.post(&config.get_add_torrent_url())
        .form(&(
            ("urls", &add.uri),
            ("autoTMM", true),
            ("cookie", ""),
            ("rename", &add.rename),
            ("category", &add.category),
            ("paused", "false"),
            ("contentLayout", "Original"),
            ("dlLimit", "NaN"),
            ("upLimit", "NaN"),
        ))
        .send()
        .expect(format!("Add torrent failed {}", &config.get_add_torrent_url()).as_str());
    println!("Add torrent result: {:#?}", resp.text().unwrap());
}
