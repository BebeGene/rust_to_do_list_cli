mod task;
mod task_manager;
mod storage;

// use task::{Task};             // Import Task from task.rs
use task_manager::{TaskManager}; // Import TaskManager from task_manager.rs
// use storage::{Storage};         // Import Storage from storage.rs

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: todo <command> [args]");
        process::exit(1);
    }

    let command = &args[1];
    let mut task_manager = TaskManager::load_from_file("tasks.txt");

    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                eprintln!("Usage: todo add <task_name>");
                process::exit(1);
            }
            let task_name = &args[2];
            task_manager.add_task(task_name);
        }
        "remove" => {
            if args.len() < 3 {
                eprintln!("Usage: todo remove <task_id>");
                process::exit(1);
            }
            let task_id: usize = args[2].parse().unwrap_or_else(|_| {
                eprintln!("Invalid task ID.");
                process::exit(1);
            });
            task_manager.remove_task(task_id);
        }
        "update" => {
            if args.len() < 4 {
                eprintln!("Usage: todo update <task_id> <status>");
                process::exit(1);
            }
            let task_id: usize = args[2].parse().unwrap_or_else(|_| {
                eprintln!("Invalid task ID.");
                process::exit(1);
            });
            let status = &args[3];
            task_manager.update_task_status(task_id, status);
        }
        "list" => {
            task_manager.list_tasks();
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            process::exit(1);
        }
    }

    // Save the task list back to the file
    task_manager.save_to_file("tasks.txt");
}
