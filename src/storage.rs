use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

use crate::task::Task;

pub struct Storage;

impl Storage {
    pub fn load_tasks(file_path: &str) -> Vec<Task> {
        let path = Path::new(file_path);
        if !path.exists() {
            return Vec::new();
        }

        let mut file = File::open(file_path).expect("Unable to open file.");
        let mut content = String::new();
        file.read_to_string(&mut content).expect("Unable to read file.");

        content
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.splitn(3, ',').collect();
                if parts.len() == 3 {
                    let id = parts[0].parse::<usize>().ok()?;
                    let status = parts[2].to_string();
                    Some(Task {
                        id,
                        name: parts[1].to_string(),
                        status,
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn save_tasks(file_path: &str, tasks: &[Task]) {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file_path)
            .expect("Unable to open file for writing.");

        for task in tasks {
            writeln!(file, "{},{},{}", task.id, task.name, task.status)
                .expect("Unable to write to file.");
        }
    }
}
