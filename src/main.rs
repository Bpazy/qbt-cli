use std::error::Error;
use qbittorrent_rs::QbtClient;

use crate::cli::cli::{Cli, Commands};
use crate::config::QbtConfig;

mod config;
mod cli;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::load();
    if cli.verbose {
        simple_logger::init_with_level(log::Level::Debug)?;
    } else {
        simple_logger::init_with_level(log::Level::Info)?;
    }
    let qbt_cfg = QbtConfig::load();
    let qbt_client = QbtClient::login(&qbt_cfg.qbittorrent_host, &qbt_cfg.username, &qbt_cfg.password)?;
    match &cli.command {
        Commands::Add(cmd) => {
            cmd.add_magnet(&qbt_client);
        }
        Commands::List(cmd) => {
            cmd.query_torrent_list(&qbt_client);
        }
    }
    Ok(())
}
