use std::collections::HashMap;

use clap::Args;

use qbittorrent_rs::QbtClient;

/// Delete torrent
#[derive(Args, Debug)]
pub struct Delete {
    /// The hashes of the torrents you want to delete. `hashes` can contain multiple hashes separated by `|`,
    /// to delete multiple torrents, or set to all, to delete all torrents.
    hashes: String,

    /// If set to true, the downloaded data will also be deleted, otherwise has no effect.
    #[arg(short = 'f', long, default_value_t = false)]
    delete_files: bool,
}

impl Delete {
    pub fn delete_torrent(&self, client: &QbtClient) {
        client.delete_torrent(&self.get_delete_magnet_form()).unwrap();
    }

    fn get_delete_magnet_form(&self) -> HashMap<&str, String> {
        let mut form: HashMap<&str, String> = HashMap::new();
        form.insert("deleteFiles", self.delete_files.to_string());
        form.insert("hashes", self.hashes.to_string());
        form
    }
}
