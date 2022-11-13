use std::collections::HashMap;
use std::error::Error;

use log::debug;
use reqwest::blocking::Client;
use serde::Deserialize;
use serde::Serialize;
use url::Url;

pub struct QbtClient {
    qbittorrent_host: String,
    username: String,
    password: String,
    reqwest_client: Option<Client>,
}


impl QbtClient {
    pub fn login(qbittorrent_host: &str, username: &str, password: &str) -> Result<QbtClient, Box<dyn Error>> {
        let mut result = QbtClient { qbittorrent_host: qbittorrent_host.to_string(), username: username.to_string(), password: password.to_string(), reqwest_client: None };
        result.do_login()?;
        Ok(result)
    }

    fn do_login(&mut self) -> Result<(), Box<dyn Error>> {
        let client = Client::builder().cookie_store(true).build()?;
        let resp = client.post(&self.get_login_url())
            .form(&(("username", &self.username), ("password", &self.password)))
            .send()
            .expect(format!("Login failed: {}", &self.get_login_url()).as_str());
        debug!("Login result: {:#?}", resp.text()?);
        self.reqwest_client = Some(client);
        Ok(())
    }

    pub fn query_torrent_list(&self, params: &HashMap<&str, String>) -> Result<Vec<Info>, Box<dyn Error>> {
        let resp = self.reqwest_client.as_ref().expect("Login needed").post(&self.get_query_torrent_list_url())
            .form(params)
            .send()
            .expect(format!("Query torrent list failed {}", &self.get_query_torrent_list_url()).as_str());
        let text = resp.text()?;
        debug!("Query torrent list result: {:#?}", &text);
        Ok(serde_json::from_str(&text)?)
    }

    pub fn add_magnet(&self, params: &HashMap<&str, String>) -> Result<(), Box<dyn Error>> {
        let resp = self.reqwest_client.as_ref().expect("Login needed").post(&self.get_add_torrent_url())
            .form(&params)
            .send()
            .expect(format!("Add torrent failed {}", &self.get_add_torrent_url()).as_str());
        debug!("Add torrent result: {:#?}", resp.text()?);
        Ok(())
    }

    fn get_login_url(&self) -> String {
        self.get_parsed_host().join("/api/v2/auth/login").unwrap().to_string()
    }

    fn get_add_torrent_url(&self) -> String {
        self.get_parsed_host().join("/api/v2/torrents/add").unwrap().to_string()
    }

    fn get_query_torrent_list_url(&self) -> String {
        self.get_parsed_host().join("/api/v2/torrents/info").unwrap().to_string()
    }

    fn get_parsed_host(&self) -> Url {
        Url::parse(self.qbittorrent_host.as_str()).expect("Invalid qbittorrent_host")
    }
}

/// See https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-4.1)#get-torrent-list
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    #[serde(rename = "added_on")]
    pub added_on: i64,
    #[serde(rename = "amount_left")]
    pub amount_left: i64,
    #[serde(rename = "auto_tmm")]
    pub auto_tmm: bool,
    pub availability: f64,
    pub category: String,
    pub completed: i64,
    #[serde(rename = "completion_on")]
    pub completion_on: i64,
    #[serde(rename = "content_path")]
    pub content_path: String,
    #[serde(rename = "dl_limit")]
    pub dl_limit: i64,
    pub dlspeed: i64,
    #[serde(rename = "download_path")]
    pub download_path: String,
    pub downloaded: i64,
    #[serde(rename = "downloaded_session")]
    pub downloaded_session: i64,
    pub eta: i64,
    #[serde(rename = "f_l_piece_prio")]
    pub f_l_piece_prio: bool,
    #[serde(rename = "force_start")]
    pub force_start: bool,
    pub hash: String,
    #[serde(rename = "infohash_v1")]
    pub infohash_v1: String,
    #[serde(rename = "infohash_v2")]
    pub infohash_v2: String,
    #[serde(rename = "last_activity")]
    pub last_activity: i64,
    #[serde(rename = "magnet_uri")]
    pub magnet_uri: String,
    #[serde(rename = "max_ratio")]
    pub max_ratio: i64,
    #[serde(rename = "max_seeding_time")]
    pub max_seeding_time: i64,
    pub name: String,
    #[serde(rename = "num_complete")]
    pub num_complete: i64,
    #[serde(rename = "num_incomplete")]
    pub num_incomplete: i64,
    #[serde(rename = "num_leechs")]
    pub num_leechs: i64,
    #[serde(rename = "num_seeds")]
    pub num_seeds: i64,
    pub priority: i64,
    pub progress: f64,
    pub ratio: f64,
    #[serde(rename = "ratio_limit")]
    pub ratio_limit: i64,
    #[serde(rename = "save_path")]
    pub save_path: String,
    #[serde(rename = "seeding_time")]
    pub seeding_time: i64,
    #[serde(rename = "seeding_time_limit")]
    pub seeding_time_limit: i64,
    #[serde(rename = "seen_complete")]
    pub seen_complete: i64,
    #[serde(rename = "seq_dl")]
    pub seq_dl: bool,
    pub size: i64,
    pub state: String,
    #[serde(rename = "super_seeding")]
    pub super_seeding: bool,
    pub tags: String,
    #[serde(rename = "time_active")]
    pub time_active: i64,
    #[serde(rename = "total_size")]
    pub total_size: i64,
    pub tracker: String,
    #[serde(rename = "trackers_count")]
    pub trackers_count: i64,
    #[serde(rename = "up_limit")]
    pub up_limit: i64,
    pub uploaded: i64,
    #[serde(rename = "uploaded_session")]
    pub uploaded_session: i64,
    pub upspeed: i64,
}
