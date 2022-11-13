use config::Config;
use directories::BaseDirs;
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QbtConfig {
    pub qbittorrent_host: String,
    pub username: String,
    pub password: String,
}

impl QbtConfig {

    pub fn load() -> QbtConfig {
        let base_dirs = BaseDirs::new().unwrap();
        let home_dir_path = base_dirs.home_dir();
        let buf = home_dir_path.join(".config/qbt/config");
        let settings = Config::builder()
            .add_source(config::File::with_name(buf.to_str().unwrap()))
            .build()
            .expect("Failed to read config file");

        let config = settings.try_deserialize::<QbtConfig>().expect("Failed to deserialize config file");
        debug!("Loaded config: {:?}", config);
        config
    }
}


