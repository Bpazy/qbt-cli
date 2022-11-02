use std::error::Error;

use crate::cli::Cli;
use crate::config::AquConfig;

mod cli;
mod config;

fn main() -> Result<(), Box<dyn Error>> {
    go_qbittorrent(AquConfig::load(), &Cli::load());
    Ok(())
}

fn go_qbittorrent(config: AquConfig, cli: &Cli) {
    let client = reqwest::blocking::Client::builder().cookie_store(true).build().unwrap();
    let resp = client.post(&config.get_login_url())
        .form(&(("username", &config.username), ("password", &config.password)))
        .send()
        .expect(format!("Login failed: {}", &config.get_login_url()).as_str());
    println!("Login result: {:#?}", resp.text().unwrap());

    let resp = client.post(&config.get_add_torrent_url())
        .form(&(
            ("urls", &cli.uri),
            ("autoTMM", true),
            ("cookie", ""),
            ("rename", &cli.rename),
            ("category", &cli.category),
            ("paused", "false"),
            ("contentLayout", "Original"),
            ("dlLimit", "NaN"),
            ("upLimit", "NaN"),
        ))
        .send()
        .expect(format!("Add torrent failed {}", &config.get_add_torrent_url()).as_str());
    println!("Add torrent result: {:#?}", resp.text().unwrap());
}
