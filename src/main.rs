use std::collections::HashMap;
use std::error::Error;

use log::debug;
use reqwest::blocking::Client;

use myserde::Info;

use crate::cli::{Add, Cli, Commands, List};
use crate::config::QbtConfig;

mod cli;
mod config;
mod myserde;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::load();
    if cli.verbose {
        simple_logger::init_with_level(log::Level::Debug).unwrap();
    } else {
        simple_logger::init_with_level(log::Level::Info).unwrap();
    }
    let qbt_cfg = QbtConfig::load();
    match &cli.command {
        Commands::Add(add_cmd) => {
            add_magnet(&qbt_cfg, add_cmd);
        }
        Commands::List(list_cmd) => {
            query_torrent_list(&qbt_cfg, list_cmd);
        }
    }
    Ok(())
}

fn add_magnet(config: &QbtConfig, add_cmd: &Add) {
    let resp = login(&config).post(&config.get_add_torrent_url())
        .form(&get_add_magnet_form(&add_cmd))
        .send()
        .expect(format!("Add torrent failed {}", &config.get_add_torrent_url()).as_str());
    debug!("Add torrent result: {:#?}", resp.text().unwrap());
}

fn get_add_magnet_form(cmd: &Add) -> HashMap<&str, String> {
    let mut form: HashMap<&str, String> = HashMap::new();
    form.insert("urls", cmd.uri.to_string());
    form.insert("autoTmm", cmd.auto_tmm.to_string());
    if let Some(savepath) = &cmd.savepath {
        form.insert("savepath", savepath.to_string());
    }
    if let Some(cookie) = &cmd.cookie {
        form.insert("cookie", cookie.to_string());
    }
    if let Some(category) = &cmd.category {
        form.insert("category", category.to_string());
    }
    if let Some(tags) = &cmd.tags {
        form.insert("tags", tags.to_string());
    }
    if let Some(skip_checking) = &cmd.skip_checking {
        form.insert("skipChecking", skip_checking.to_string());
    }
    if let Some(paused) = &cmd.paused {
        form.insert("paused", paused.to_string());
    }
    if let Some(root_folder) = &cmd.root_folder {
        form.insert("rootFolder", root_folder.to_string());
    }
    if let Some(rename) = &cmd.rename {
        form.insert("rename", rename.to_string());
    }
    if let Some(up_limit) = &cmd.up_limit {
        form.insert("upLimit", up_limit.to_string());
    }
    if let Some(dl_limit) = &cmd.dl_limit {
        form.insert("dlLimit", dl_limit.to_string());
    }
    if let Some(ratio_limit) = &cmd.ratio_limit {
        form.insert("ratioLimit", ratio_limit.to_string());
    }
    if let Some(seeding_time_limit) = &cmd.seeding_time_limit {
        form.insert("seedingTimeLimit", seeding_time_limit.to_string());
    }
    if let Some(sequential_download) = &cmd.sequential_download {
        form.insert("sequentialDownload", sequential_download.to_string());
    }
    if let Some(first_last_piece_prio) = &cmd.first_last_piece_prio {
        form.insert("firstLastPiecePrio", first_last_piece_prio.to_string());
    }
    form
}


fn query_torrent_list(config: &QbtConfig, cmd: &List) {
    let resp = login(&config).post(&config.get_query_torrent_list_url())
        .form(&get_query_torrent_list_form(&cmd))
        .send()
        .expect(format!("Query torrent list failed {}", &config.get_query_torrent_list_url()).as_str());
    let text = resp.text().unwrap();
    debug!("Query torrent list result: {:#?}", &text);
    let qbt_infos: Vec<Info> = serde_json::from_str(&text).unwrap();
    for qbt_info in qbt_infos {
        println!("{}\t{}%", qbt_info.name, (qbt_info.progress * 100 as f64).floor())
    }
}

fn get_query_torrent_list_form(cmd: &List) -> HashMap<&str, String> {
    let mut form: HashMap<&str, String> = HashMap::new();
    form.insert("reverse", cmd.reverse.to_string());
    if let Some(filter) = &cmd.filter {
        form.insert("filter", filter.to_string());
    }
    if let Some(category) = &cmd.category {
        form.insert("category", category.to_string());
    }
    if let Some(tag) = &cmd.tag {
        form.insert("tag", tag.to_string());
    }
    if let Some(sort) = &cmd.sort {
        form.insert("sort", sort.to_string());
    }
    if let Some(limit) = &cmd.limit {
        form.insert("limit", limit.to_string());
    }
    if let Some(offset) = &cmd.offset {
        form.insert("offset", offset.to_string());
    }
    if let Some(hashes) = &cmd.hashes {
        form.insert("hashes", hashes.to_string());
    }
    form
}

fn login(config: &&QbtConfig) -> Client {
    let client = Client::builder().cookie_store(true).build().unwrap();
    let resp = client.post(&config.get_login_url())
        .form(&(("username", &config.username), ("password", &config.password)))
        .send()
        .expect(format!("Login failed: {}", &config.get_login_url()).as_str());
    debug!("Login result: {:#?}", resp.text().unwrap());
    client
}
