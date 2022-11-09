use clap::{Args, Parser, Subcommand};
use log::debug;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Use verbose output
    #[arg(long)]
    pub verbose: bool,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Add(Add),
    List(List),
}

/// Add new torrent
#[derive(Args, Debug)]
pub struct Add {
    /// Download folder
    #[arg(long)]
    pub savepath: Option<String>,
    /// Cookie sent to download the .torrent file
    #[arg(long)]
    pub cookie: Option<String>,
    /// Category for the torrent
    #[arg(long)]
    pub category: Option<String>,
    /// Tags for the torrent, split by ','
    #[arg(long)]
    pub tags: Option<String>,
    /// Skip hash checking. Possible values are true, false (default)
    #[arg(long)]
    pub skip_checking: Option<String>,
    /// Add torrents in the paused state. Possible values are true, false (default)
    #[arg(long)]
    pub paused: Option<String>,
    /// Create the root folder. Possible values are true, false, unset (default)
    #[arg(long)]
    pub root_folder: Option<String>,
    /// Rename torrent
    #[arg(long)]
    pub rename: Option<String>,
    /// Set torrent upload speed limit. Unit in bytes/second
    #[arg(long)]
    pub up_limit: Option<i64>,
    /// Set torrent download speed limit. Unit in bytes/second
    #[arg(long)]
    pub dl_limit: Option<i64>,
    /// Set torrent share ratio limit
    #[arg(long)]
    pub ratio_limit: Option<f64>,
    /// Set torrent seeding time limit. Unit in minutes
    #[arg(long)]
    pub seeding_time_limit: Option<i64>,
    /// Whether Automatic Torrent Management should be used
    #[arg(long)]
    pub auto_tmm: Option<bool>,
    /// Enable sequential download. Possible values are true, false (default)
    #[arg(long)]
    pub sequential_download: Option<String>,
    /// Prioritize download first last piece. Possible values are true, false (default)
    #[arg(long)]
    pub first_last_piece_prio: Option<String>,
    /// URLs separated with newlines
    #[arg(value_parser = uri_parser)]
    pub uri: String,
}

/// Get torrent list
#[derive(Args, Debug)]
pub struct List {
    #[arg(short, long)]
    pub filter: Option<String>,
    #[arg(short, long)]
    pub category: Option<String>,
    #[arg(short, long)]
    pub tag: Option<String>,
    #[arg(short, long)]
    pub sort: Option<String>,
    #[arg(short, long)]
    pub reverse: Option<bool>,
    #[arg(short, long)]
    pub limit: Option<i32>,
    #[arg(short, long)]
    pub offset: Option<i32>,
    #[arg(long)]
    pub hashes: Option<String>,
}

const MAGNET_PREFIX: &str = "magnet:";

fn uri_parser(s: &str) -> Result<String, String> {
    if s.starts_with(MAGNET_PREFIX) {
        Ok(s.to_string())
    } else {
        Err(format!("Uri must starts with '{}'", MAGNET_PREFIX))
    }
}

impl Cli {
    pub fn load() -> Cli {
        let r = Cli::parse();
        debug!("Cli: {:?}", &r);
        r
    }
}
