use std::error::Error;
use qbittorrent_rs::QbtClient;

use crate::cli::cli::{Cli};
use crate::config::QbtConfig;

mod config;
mod cli;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::load();
    set_logger_level(&cli);
    let qbt_cfg = QbtConfig::load();
    let qbt_client = QbtClient::login(&qbt_cfg.qbittorrent_host, &qbt_cfg.username, &qbt_cfg.password)?;
    cli.command.exec(&qbt_client);
    Ok(())
}

fn set_logger_level(cli: &Cli) {
    if cli.verbose {
        simple_logger::init_with_level(log::Level::Debug).unwrap();
    } else {
        simple_logger::init_with_level(log::Level::Info).unwrap();
    }
}
