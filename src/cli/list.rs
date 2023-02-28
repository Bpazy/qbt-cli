use std::collections::HashMap;

use clap::Args;

use qbittorrent_rs::QbtClient;

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
    pub fn query_torrent_list(&self, client: &QbtClient) {
        let qbt_infos = client.query_torrent_list(&self.get_query_torrent_list_form()).unwrap();
        for qbt_info in qbt_infos {
            println!("{} {:4.1}% {:>11} {:4.2}MiB/s {}", qbt_info.hash, qbt_info.progress * 100 as f64, qbt_info.state, qbt_info.dlspeed as f64 / 1024.0 / 1024.0, qbt_info.name);
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
