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

impl Cli {
    fn unwrap_uri(&self) -> String {
        match self.uri.as_ref() {
            None => {
                println!("magnet uri is required");
                exit(0)
            }
            Some(uri) => {
                if uri.starts_with(MAGNET_PREFIX) {
                    println!("url: {:?}", uri);
                    return uri.clone();
                } else {
                    println!("magnet uri is invalid");
                    exit(0);
                }
            }
        }
    }
}

const MAGNET_PREFIX: &str = "magnet:";

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    go_qbittorrent(load_config(), cli.unwrap_uri());
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

fn go_qbittorrent(config: AquConfig, magnet_uri: String) {
    let client = reqwest::blocking::Client::builder().cookie_store(true).build().unwrap();
    let resp = client.post(&config.get_login_url())
        .form(&(("username", &config.username), ("password", &config.password)))
        .send()
        .expect(format!("Login failed: {}", &config.get_login_url()).as_str());
    println!("Login result: {:#?}", resp.text().unwrap());

    client.post(&config.get_add_torrent_url())
        .form(&(("username", &config.username), ("password", &config.password)))
        .send()
        .expect(format!("Add torrent failed {}", &config.get_login_url()).as_str());
}
