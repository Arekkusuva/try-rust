use std::collections::HashMap;
use super::conf;

// TODO: Add threads.
pub fn start_watching(targets: conf::Config) {
    let mut once_map: HashMap<String, Vec<String>> = HashMap::new();

    // Handle `once` items.
    match targets.once {
        Some(list) => {
            list.into_iter().for_each(|(task_name, task)| {
                let mut deferred = task.deferred == Some(true);
                match task.actions {
                    Some(actions) => {
                        if !deferred {
                            handle_once(&task_name, &actions);
                        } else {
                            println!("skip task: {}", task_name);
                        }
                        once_map.insert(task_name, actions);
                    }
                    _ => ()
                }
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
