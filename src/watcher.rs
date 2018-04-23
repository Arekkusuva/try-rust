#![allow(dead_code)]

use std::collections::HashMap;
use super::conf;

#[derive(Debug)]
pub struct Watcher {
    once: HashMap<String, Vec<String>>,
    watch: Vec<Watch>,
}

#[derive(Debug)]
struct Watch {
    name: String,
    do_before: Vec<String>,
    actions: Vec<String>,
    do_after: Vec<String>,
}

impl Watcher {
    /// Returns new instance.
    pub fn new(c: conf::Config) -> Watcher {
        let (once, watch) = from_conf(c);
        Watcher { once, watch }
    }

    /// Updates watcher data.
    pub fn update(&mut self, c: conf::Config) {
        unimplemented!();
    }

    /// Starts task process, if not started.
    pub fn start(&mut self, task_name: String) {
        unimplemented!();
    }

    /// Starts task process, if started.
    pub fn stop(&mut self, task_name: String) {
        unimplemented!();
    }

    /// Returns current status of task.
    /// True - if task is active.
    /// False - if task is not active.
    pub fn status(&self, task_name: String) -> bool {
        unimplemented!();
    }

    // TODO: Remove.
    pub fn show(&self) {
        println!("once: {:#?}", self.once);
        println!("watch: {:#?}", self.watch);
    }
}

/// Returns transformed data from config.
fn from_conf(c: conf::Config) -> (HashMap<String, Vec<String>>, Vec<Watch>) {
    let mut once_result: HashMap<String, Vec<String>> = HashMap::new();
    let mut watch_result: Vec<Watch> = Vec::new();

    // Handle `once` items.
    match c.once {
        Some(list) => {
            list.into_iter().for_each(|(task_name, task)| {
                match task.actions {
                    Some(actions) => {
                        println!("add task: {}", task_name);
                        once_result.insert(task_name, actions);
                    }
                    _ => ()
                }
            })
        },
        _ => ()
    }

    // Handle `watch` items.
    match c.watch {
        Some(list) => {
            list.into_iter().for_each(|watch_conf| {
                let watch = Watch {
                    name: match watch_conf.name {
                        Some(v) => v,
                        _ => String::new(),
                    },
                    do_before: match watch_conf.do_before {
                        Some(v) => v,
                        _ => Vec::<String>::new(),
                    },
                    actions: match watch_conf.actions {
                        Some(v) => v,
                        _ => Vec::<String>::new(),
                    },
                    do_after: match watch_conf.do_after {
                        Some(v) => v,
                        _ => Vec::<String>::new(),
                    },
                };
                watch_result.push(watch);
            })
        },
        _ => (),
    }

    (once_result, watch_result)
}

fn handle_once(task_name: &String, actions: &Vec<String>) {
    println!("Run {}, {:#?}", task_name, actions);
}

fn handle_watch() {
    unimplemented!();
}
