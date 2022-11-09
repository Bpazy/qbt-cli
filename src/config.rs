use config::Config;
use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct AquConfig {
    pub qbittorrent_host: String,
    pub username: String,
    pub password: String,
}

impl AquConfig {

    pub fn load() -> AquConfig {
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

    pub fn get_login_url(&self) -> String {
        self.get_parsed_host().join("/api/v2/auth/login").unwrap().to_string()
    }

    pub fn get_add_torrent_url(&self) -> String {
        self.get_parsed_host().join("/api/v2/torrents/add").unwrap().to_string()
    }

    pub fn get_query_torrent_list_url(&self) -> String {
        self.get_parsed_host().join("/api/v2/torrents/info").unwrap().to_string()
    }

    pub fn get_parsed_host(&self) -> Url {
        Url::parse(self.qbittorrent_host.as_str()).expect("Invalid qbittorrent_host")
    }
}


