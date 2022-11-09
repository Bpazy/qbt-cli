use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub verbose: bool,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Add(Add),
    List(List),
}

#[derive(Args, Debug)]
pub struct Add {
    #[arg(short, long, default_value_t = String::from(""))]
    pub rename: String,
    #[arg(short, long, default_value_t = String::from(""))]
    pub category: String,
    #[arg(value_parser = uri_parser)]
    pub uri: String,
}

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
    #[arg(short, long)]
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
        println!("Cli: {:?}", &r);
        r
    }
}
