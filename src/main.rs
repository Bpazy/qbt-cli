use std::collections::HashMap;
use std::error::Error;

use log::debug;
use reqwest::blocking::Client;

use myserde::Info;

use crate::cli::{Add, Cli, Commands, List};
use crate::config::AquConfig;

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
    let aqu_cfg = AquConfig::load();
    match &cli.command {
        Commands::Add(add_cmd) => {
            add_magnet(&aqu_cfg, add_cmd);
        }
        Commands::List(list_cmd) => {
            query_torrent_list(&aqu_cfg, list_cmd);
        }
    }
    Ok(())
}

fn add_magnet(config: &AquConfig, add_cmd: &Add) {
    let resp = login(&config).post(&config.get_add_torrent_url())
        .form(&(
            ("urls", &add_cmd.uri),
            ("autoTMM", true),
            ("cookie", ""),
            ("rename", &add_cmd.rename),
            ("category", &add_cmd.category),
            ("paused", "false"),
            ("contentLayout", "Original"),
            ("dlLimit", "NaN"),
            ("upLimit", "NaN"),
        ))
        .send()
        .expect(format!("Add torrent failed {}", &config.get_add_torrent_url()).as_str());
    debug!("Add torrent result: {:#?}", resp.text().unwrap());
}


fn query_torrent_list(config: &AquConfig, cmd: &List) {
    let resp = login(&config).post(&config.get_query_torrent_list_url())
        .form(&get_form(&cmd))
        .send()
        .expect(format!("Query torrent list failed {}", &config.get_query_torrent_list_url()).as_str());
    let text = resp.text().unwrap();
    debug!("Query torrent list result: {:#?}", &text);
    let qbt_infos: Vec<Info> = serde_json::from_str(&text).unwrap();
    for qbt_info in qbt_infos {
        println!("{}\t{}%", qbt_info.name, (qbt_info.progress * 100 as f64).floor())
    }
}

fn get_form(cmd: &List) -> HashMap<&str, String> {
    let mut form: HashMap<&str, String> = HashMap::new();
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
    if let Some(reverse) = &cmd.reverse {
        form.insert("reverse", reverse.to_string());
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

fn login(config: &&AquConfig) -> Client {
    let client = Client::builder().cookie_store(true).build().unwrap();
    let resp = client.post(&config.get_login_url())
        .form(&(("username", &config.username), ("password", &config.password)))
        .send()
        .expect(format!("Login failed: {}", &config.get_login_url()).as_str());
    debug!("Login result: {:#?}", resp.text().unwrap());
    client
}
