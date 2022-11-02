use std::error::Error;

use clap::Parser;
use config::Config;
use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    verbose: bool,
    #[arg(short, long, default_value_t = String::from(""))]
    rename: String,
    #[arg(short, long, default_value_t = String::from(""))]
    category: String,

    #[arg(value_parser = uri_parser)]
    uri: String,
}

const MAGNET_PREFIX: &str = "magnet:";

fn uri_parser(s: &str) -> Result<String, String> {
    if s.starts_with(MAGNET_PREFIX) {
        Ok(s.to_string())
    } else {
        Err(format!("Uri must starts with '{}'", MAGNET_PREFIX))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    println!("Cli: {:?}", &cli);
    go_qbittorrent(load_config(), &cli);
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct AquConfig {
    qbittorrent_host: String,
    username: String,
    password: String,
}

impl AquConfig {
    fn get_login_url(&self) -> String {
        self.get_parsed_host().join("/api/v2/auth/login").unwrap().to_string()
    }

    fn get_parsed_host(&self) -> Url {
        Url::parse(self.qbittorrent_host.as_str()).expect("Invalid qbittorrent_host")
    }

    fn get_add_torrent_url(&self) -> String {
        self.get_parsed_host().join("/api/v2/torrents/add").unwrap().to_string()
    }
}

fn load_config() -> AquConfig {
    let base_dirs = BaseDirs::new().unwrap();
    let home_dir_path = base_dirs.home_dir();
    let buf = home_dir_path.join(".config/aqu/config");
    let settings = Config::builder()
        .add_source(config::File::with_name(buf.to_str().unwrap()))
        .build()
        .expect("Failed to read config file");

    let config = settings.try_deserialize::<AquConfig>().expect("Failed to deserialize config file");
    println!("Loaded config: {:?}", config);
    config
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
