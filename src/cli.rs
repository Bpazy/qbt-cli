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
    Add(Add)
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
