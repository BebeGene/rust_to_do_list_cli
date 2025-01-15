# To-Do List Manager
This is a simple command line to-do list manager written in Rust. It allows you to add, remove, update, and list tasks. The tasks are stored in a local text file for persistence.

## Features
* **Add** tasks to your to-do list.
* **Remove** tasks by their ID.
* **Update** the status of tasks (pending/completed).
* **List** all tasks and their status.

## Prerequisites
* Install [Rust](https://www.rust-lang.org/)
* Clone repo 

## Run
* cd cli_to_do_list

* Add a task: cargo run add "Buy groceries" (replace your tasks in quotes)
* Update a task status: cargo run update 1 completed
* List tasks: cargo run list
* Remove a task: cargo run remove

## Screenshots
### Add task:
![Alt Text](https://github.com/BebeGene/rust_to_do_list_cli/blob/master/Screenshots/CLI_to_do_list_Step1.png) <br><br>

### Update task:
![Alt Text](https://github.com/BebeGene/rust_to_do_list_cli/blob/master/Screenshots/CLI_to_do_list_Step2.png) <br><br>

### Remove task:
![Alt Text](https://github.com/BebeGene/rust_to_do_list_cli/blob/master/Screenshots/CLI_to_do_list_Step3.png) <br><br>

### View task list:
![Alt Text](https://github.com/BebeGene/rust_to_do_list_cli/blob/master/Screenshots/CLI_to_do_list_ViewList.png) <br>
