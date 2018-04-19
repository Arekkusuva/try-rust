#[macro_use]
extern crate serde_derive;

use std::path::Path;

mod conf;
mod watcher;

fn main() {
    let conf_path = Path::new("./data/conf.toml");
    let targets = conf::parse(conf_path);
    watcher::start_watching(targets)
}
