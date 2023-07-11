use std::collections::HashMap;

use clap::Args;

use qbittorrent_rs::QbtClient;

/// Add new torrent
#[derive(Args, Debug)]
pub struct Add {
    /// Download folder
    #[arg(long)]
    pub savepath: Option<String>,
    /// Cookie sent to download the .torrent file
    #[arg(long)]
    pub cookie: Option<String>,
    /// Category for the torrent
    #[arg(long, short)]
    pub category: Option<String>,
    /// Tags for the torrent, split by ','
    #[arg(long, short)]
    pub tags: Option<String>,
    /// Skip hash checking. Possible values are true, false (default)
    #[arg(long)]
    pub skip_checking: Option<String>,
    /// Add torrents in the paused state. Possible values are true, false (default)
    #[arg(long)]
    pub paused: Option<String>,
    /// Create the root folder. Possible values are true, false, unset (default)
    #[arg(long)]
    pub root_folder: Option<String>,
    /// Rename torrent
    #[arg(long, short)]
    pub rename: Option<String>,
    /// Set torrent upload speed limit. Unit in bytes/second
    #[arg(long)]
    pub up_limit: Option<i64>,
    /// Set torrent download speed limit. Unit in bytes/second
    #[arg(long)]
    pub dl_limit: Option<i64>,
    /// Set torrent share ratio limit
    #[arg(long)]
    pub ratio_limit: Option<f64>,
    /// Set torrent seeding time limit. Unit in minutes
    #[arg(long)]
    pub seeding_time_limit: Option<i64>,
    /// Whether Automatic Torrent Management should be used
    #[arg(long, default_value_t = false)]
    pub auto_tmm: bool,
    /// Enable sequential download. Possible values are true, false (default)
    #[arg(long)]
    pub sequential_download: Option<String>,
    /// Prioritize download first last piece. Possible values are true, false (default)
    #[arg(long)]
    pub first_last_piece_prio: Option<String>,
    /// URLs separated with newlines
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

impl Add {
    pub fn add_magnet(&self, client: &QbtClient) {
        match client.add_magnet(&self.get_add_magnet_form()) {
            Ok(()) => {},
            Err(error) => {
                println!("Add magnet failed message: {}", error)
            }
        }
    }

    fn get_add_magnet_form(&self) -> HashMap<&str, String> {
        let mut form: HashMap<&str, String> = HashMap::new();
        form.insert("urls", self.uri.to_string());
        form.insert("autoTmm", self.auto_tmm.to_string());
        if let Some(savepath) = &self.savepath {
            form.insert("savepath", savepath.to_string());
        }
        if let Some(cookie) = &self.cookie {
            form.insert("cookie", cookie.to_string());
        }
        if let Some(category) = &self.category {
            form.insert("category", category.to_string());
        }
        if let Some(tags) = &self.tags {
            form.insert("tags", tags.to_string());
        }
        if let Some(skip_checking) = &self.skip_checking {
            form.insert("skipChecking", skip_checking.to_string());
        }
        if let Some(paused) = &self.paused {
            form.insert("paused", paused.to_string());
        }
        if let Some(root_folder) = &self.root_folder {
            form.insert("rootFolder", root_folder.to_string());
        }
        if let Some(rename) = &self.rename {
            form.insert("rename", rename.to_string());
        }
        if let Some(up_limit) = &self.up_limit {
            form.insert("upLimit", up_limit.to_string());
        }
        if let Some(dl_limit) = &self.dl_limit {
            form.insert("dlLimit", dl_limit.to_string());
        }
        if let Some(ratio_limit) = &self.ratio_limit {
            form.insert("ratioLimit", ratio_limit.to_string());
        }
        if let Some(seeding_time_limit) = &self.seeding_time_limit {
            form.insert("seedingTimeLimit", seeding_time_limit.to_string());
        }
        if let Some(sequential_download) = &self.sequential_download {
            form.insert("sequentialDownload", sequential_download.to_string());
        }
        if let Some(first_last_piece_prio) = &self.first_last_piece_prio {
            form.insert("firstLastPiecePrio", first_last_piece_prio.to_string());
        }
        form
    }
}
