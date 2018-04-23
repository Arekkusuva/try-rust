#[macro_use]
extern crate log;
extern crate log4rs;
#[macro_use]
extern crate serde_derive;

use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};
use std::path::Path;
use std::thread;
use std::sync::mpsc;

mod conf;
mod watcher;

fn main() {
    let conf_path = Path::new("./data/conf.toml");
    initiate_log();
    let conf = conf::parse(conf_path);
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let w = watcher::Watcher::new(conf);
        w.show();

        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    for received in rx {
        println!("Received {}", received);
    }
}

fn initiate_log() {
//    log4rs::init_file(conf_path, Default::default()).unwrap();

    let main_file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)(utc)} [{level} from {M} l-{line}]: {m}{n}")))
        .build("output/output.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("main_file", Box::new(main_file)))
        .build(Root::builder().appender("main_file").build(LevelFilter::Trace))
        .unwrap();

    log4rs::init_config(config).unwrap();
    info!("log was configured")
}
