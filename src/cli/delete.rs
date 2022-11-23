use clap::Args;

use qbittorrent_rs::QbtClient;

/// Delete torrent
#[derive(Args, Debug)]
pub struct Delete {
    /// The hashes of the torrents you want to delete. `hashes` can contain multiple hashes separated by `|`,
    /// to delete multiple torrents, or set to all, to delete all torrents.
    hashes: String,
}

impl Delete {
    pub fn delete_torrent(&self, client: &QbtClient) {
        client.delete_torrent(self.hashes.split("|").collect()).unwrap();
    }
}
