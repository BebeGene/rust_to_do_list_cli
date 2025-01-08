# To-Do List Manager

This is a simple command-line To-Do list manager written in Rust. It allows you to add, remove, update, and list tasks. The tasks are stored in a local text file for persistence.

## Features

- **Add** tasks to your to-do list.
- **Remove** tasks by their ID.
- **Update** the status of tasks (pending/completed).
- **List** all tasks and their status.

## Prerequisites

Install [Rust](https://www.rust-lang.org/) installed to run this program. If you haven't installed Rust yet, follow the installation instructions on the official website.

## How to Run
- Add a task: cargo run add "Buy groceries" (replace your tasks in quotes)
- Remove a task: cargo run remove 1
- Update a task status: cargo run update 1 completed
- List tasks: cargo run list

### Clone the Repository

If you haven't already, clone the repository:

git clone <repository-url>
cd to_do_list_cli

