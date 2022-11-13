use std::error::Error;

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
