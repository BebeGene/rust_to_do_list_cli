use crate::task::Task;
use crate::storage::Storage;

pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {

    pub fn load_from_file(file_path: &str) -> Self {
        let tasks = Storage::load_tasks(file_path);
        TaskManager { tasks }
    }

    pub fn save_to_file(&self, file_path: &str) {
        Storage::save_tasks(file_path, &self.tasks);
    }

    pub fn add_task(&mut self, task_name: &str) {
        let id = self.tasks.len() + 1;
        let task = Task::new(id, task_name);
        self.tasks.push(task);
        println!("Task '{}' added.", task_name);
    }

    pub fn remove_task(&mut self, task_id: usize) {
        if let Some(pos) = self.tasks.iter().position(|task| task.id == task_id) {
            self.tasks.remove(pos);
            println!("Task with ID {} removed.", task_id);
        } else {
            eprintln!("Task with ID {} not found.", task_id);
        }
    }

    pub fn update_task_status(&mut self, task_id: usize, status: &str) {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == task_id) {
            task.update_status(status);
            println!("Task {} updated to '{}' status.", task_id, status);
        } else {
            eprintln!("Task with ID {} not found.", task_id);
        }
    }

    pub fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks found.");
        } else {
            for task in &self.tasks {
                println!("ID: {}, Name: {}, Status: {}", task.id, task.name, task.status);
            }
        }
    }
}