use clap::{Parser, Subcommand};
use log::debug;

use qbittorrent_rs::QbtClient;

use crate::cli::add::Add;
use crate::cli::list::List;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Use verbose output
    #[arg(long, default_value_t = false)]
    pub verbose: bool,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Add(Add),
    List(List),
}

impl Commands {
    pub fn exec(&self, qbt_client: &QbtClient) {
        match &self {
            Commands::Add(cmd) => {
                cmd.add_magnet(qbt_client);
            }
            Commands::List(cmd) => {
                cmd.query_torrent_list(qbt_client);
            }
        }
    }
}


impl Cli {
    pub fn load() -> Cli {
        let r = Cli::parse();
        debug!("Cli: {:?}", &r);
        r
    }
}
