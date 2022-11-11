use std::collections::HashMap;

use clap::{Args};
use log::debug;
use serde::Deserialize;
use serde::Serialize;

use crate::{login, QbtConfig};

/// Get torrent list
#[derive(Args, Debug)]
pub struct List {
    /// Filter torrent list by state. Allowed state filters: all, downloading, seeding, completed,
    /// paused, active, inactive, resumed, stalled, stalled_uploading, stalled_downloading, errored
    #[arg(short, long)]
    pub filter: Option<String>,
    /// Get torrents with the given category (empty string means "without category"; no "category"
    /// parameter means "any category". Remember to URL-encode the category name.
    /// For example, My category becomes My%20category
    #[arg(short, long)]
    pub category: Option<String>,
    /// Get torrents with the given tag (empty string means "without tag"; no "tag" parameter
    /// means "any tag". Remember to URL-encode the category name.
    /// For example, My tag becomes My%20tag
    #[arg(short, long)]
    pub tag: Option<String>,
    /// Sort torrents by given key. They can be sorted using any field of the response's JSON
    /// array (which are documented below) as the sort key.
    #[arg(short, long)]
    pub sort: Option<String>,
    /// Enable reverse sorting. Defaults to false
    #[arg(short, long, default_value_t = false)]
    pub reverse: bool,
    /// Limit the number of torrents returned
    #[arg(short, long)]
    pub limit: Option<i32>,
    /// Set offset (if less than 0, offset from end)
    #[arg(short, long)]
    pub offset: Option<i32>,
    /// Filter by hashes. Can contain multiple hashes separated by |
    #[arg(long)]
    pub hashes: Option<String>,
}

impl List {
    pub fn query_torrent_list(&self, config: &QbtConfig) {
        let resp = login(&config).post(&config.get_query_torrent_list_url())
            .form(&self.get_query_torrent_list_form())
            .send()
            .expect(format!("Query torrent list failed {}", &config.get_query_torrent_list_url()).as_str());
        let text = resp.text().unwrap();
        debug!("Query torrent list result: {:#?}", &text);
        let qbt_infos: Vec<Info> = serde_json::from_str(&text).unwrap();
        for qbt_info in qbt_infos {
            println!("{} {:4.1}% {:>11} {}", qbt_info.hash, qbt_info.progress * 100 as f64, qbt_info.state, qbt_info.name);
        }
    }

    fn get_query_torrent_list_form(&self) -> HashMap<&str, String> {
        let mut form: HashMap<&str, String> = HashMap::new();
        form.insert("reverse", self.reverse.to_string());
        if let Some(filter) = &self.filter {
            form.insert("filter", filter.to_string());
        }
        if let Some(category) = &self.category {
            form.insert("category", category.to_string());
        }
        if let Some(tag) = &self.tag {
            form.insert("tag", tag.to_string());
        }
        if let Some(sort) = &self.sort {
            form.insert("sort", sort.to_string());
        }
        if let Some(limit) = &self.limit {
            form.insert("limit", limit.to_string());
        }
        if let Some(offset) = &self.offset {
            form.insert("offset", offset.to_string());
        }
        if let Some(hashes) = &self.hashes {
            form.insert("hashes", hashes.to_string());
        }
        form
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
