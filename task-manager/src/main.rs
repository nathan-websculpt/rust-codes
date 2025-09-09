use std::collections::HashMap;
use std::io::{self, Write};

// color-coded command-line todo list manager

// use 'add' to add a task
// use 'list' to list all tasks
// use 'update' to update a task status - the id will start at 1 and increment - you can view IDs with 'list'
// use 'quit' to exit the program

#[derive(Debug)]
enum Status {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
    status: Status,
}

struct TaskManager {
    tasks: HashMap<u32, Task>, // mapping task IDs (u32) to Task objects
    next_id: u32,              // to keep track of the next task ID
}

// functions for TaskManager
impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String) {
        let task = Task {
            id: self.next_id,
            title,
            status: Status::Todo,
        };
        self.tasks.insert(self.next_id, task);
        self.next_id += 1;
    }

    // mutates a task if it exists
    // get_mut allows for change of value directly
    fn update_status(&mut self, id: u32, status: Status) {
        if let Some(task) = self.tasks.get_mut(&id) {
            task.status = status;
        } else {
            println!("Task with id {} not found!", id);
        }
    }

    fn list_tasks(&self) {
        println!("\nID  | Status      | Title");
        println!("----|------------|----------------");
        for task in self.tasks.values() {
            let status_str = match task.status {
                Status::Todo => "\x1b[33mTodo\x1b[0m",         // Yellow
                Status::InProgress => "\x1b[34mInProgress\x1b[0m", // Blue
                Status::Done => "\x1b[32mDone\x1b[0m",         // Green
            };
            println!("{:<3} | {:<10} | {}", task.id, status_str, task.title);
        }
    }
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let mut manager = TaskManager::new();

    loop {
        println!("\nCommands: add, list, update, quit");
        let command = read_input("> ");

        match command.as_str() {
            "add" => {
                let title = read_input("Enter task title: ");
                manager.add_task(title);
            }
            "list" => manager.list_tasks(),
            "update" => {
                let id: u32 = read_input("Enter task ID: ")
                    .parse()
                    .unwrap_or(0);
                let status_str = read_input("Enter status (todo, inprogress, done): ");
                let status = match status_str.to_lowercase().as_str() {
                    "todo" => Status::Todo,
                    "inprogress" => Status::InProgress,
                    "done" => Status::Done,
                    _ => {
                        println!("Unknown status. Using Todo.");
                        Status::Todo
                    }
                };
                manager.update_status(id, status);
            }
            "quit" => break,
            _ => println!("Unknown command!"),
        }
    }

    println!("Goodbye!");
}
