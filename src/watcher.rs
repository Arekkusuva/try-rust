use std::collections::HashMap;
use super::conf;

// TODO: Add threads.
pub fn start_watching(targets: conf::Config) {
    let mut once_map: HashMap<String, conf::OnceConf> = HashMap::new();

    // Handle `once` items.
    match targets.once {
        Some(list) => {
            list.into_iter().for_each(|(task_name, task)| {
                if task.deferred == Some(true) {
                    println!("skip task: {}", task_name);
                } else {
                    match task.actions {
                        Some(ref actions) => {
                            handle_once(&task_name, actions);
                        }
                        _ => ()
                    }
                }
                once_map.insert(task_name, task);
            })
        },
        _ => ()
    }
    // Handle `watch` items.
}

fn handle_once(task_name: &String, actions: &Vec<String>) {
    println!("Run {}, {:#?}", task_name, actions);
}

fn handle_watch() {

}
