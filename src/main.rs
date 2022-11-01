use std::error::Error;
use std::process::exit;

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

    uri: Option<String>,
}

const MAGNET_PREFIX: &str = "magnet:";

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    peek(&cli);
    let config_map = load_config();
    go_qbittorrent(config_map);
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

    let map = settings.try_deserialize::<AquConfig>().expect("Failed to deserialize config file");
    println!("{:?}", map);
    map
}

fn go_qbittorrent(config: AquConfig) {
    let client = reqwest::blocking::Client::builder().cookie_store(true).build().unwrap();
    let url = config.get_login_url();
    let url_str = url.as_str();
    let resp = client.post(url_str)
        .form(&(("username", config.username), ("password", config.password)))
        .send()
        .expect(format!("Failed to send request to qbittorrent server {}", url_str).as_str());
    println!("{:#?}", resp.text().unwrap());
}

fn peek(cli: &Cli) {
    println!("verbose: {:?}", cli);

    match &cli.uri {
        None => {
            println!("magnet uri is required");
            exit(0)
        }
        Some(uri) => {
            if uri.starts_with(MAGNET_PREFIX) {
                println!("url: {:?}", uri);
            } else {
                println!("magnet uri is invalid");
            }
        }
    }
}
