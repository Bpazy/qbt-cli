use clap::{Parser, Subcommand};
use log::debug;

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


impl Cli {
    pub fn load() -> Cli {
        let r = Cli::parse();
        debug!("Cli: {:?}", &r);
        r
    }
}
