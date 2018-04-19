extern crate toml;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub once: Option<HashMap<String, OnceConf>>,
    pub watch: Option<Vec<WatchConf>>,
}

#[derive(Debug, Deserialize)]
pub struct OnceConf {
    pub deferred: Option<bool>,
    pub actions: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct WatchConf {
    pub name: Option<String>,
    pub do_before: Option<Vec<String>>,
    pub actions: Option<Vec<String>>,
    pub do_after: Option<Vec<String>>,
}


pub fn parse(conf_path: &Path) -> Config {
    let mut f = File::open(conf_path).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    let decoded: Config = toml::from_str(contents.as_str()).unwrap();
    decoded
}