use std::error::Error;

use log::debug;
use reqwest::blocking::Client;

use crate::cli::cli::{Cli, Commands};
use crate::config::QbtConfig;

mod config;
mod cli;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::load();
    if cli.verbose {
        simple_logger::init_with_level(log::Level::Debug).unwrap();
    } else {
        simple_logger::init_with_level(log::Level::Info).unwrap();
    }
    let qbt_cfg = QbtConfig::load();
    match &cli.command {
        Commands::Add(cmd) => {
            cmd.add_magnet(&qbt_cfg);
        }
        Commands::List(cmd) => {
            cmd.query_torrent_list(&qbt_cfg);
        }
    }
    Ok(())
}

fn login(config: &&QbtConfig) -> Client {
    let client = Client::builder().cookie_store(true).build().unwrap();
    let resp = client.post(&config.get_login_url())
        .form(&(("username", &config.username), ("password", &config.password)))
        .send()
        .expect(format!("Login failed: {}", &config.get_login_url()).as_str());
    debug!("Login result: {:#?}", resp.text().unwrap());
    client
}
