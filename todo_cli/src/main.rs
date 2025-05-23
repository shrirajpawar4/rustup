use std::fs;
use std::path::PathBuf;
use chrono::{DateTime, Local};
use clap::{Parser, Subcommand}; // command line argument parser
use colored::*;
use serde::{Deserialize, Serialize}; // json serializer and deserializer

#[derive(Parser)] // read command line arguments
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]


enum Commands {
    /// Add a new todo item
    Add {
        description: String,
    },
    /// List all todo items
    List,
    /// Mark a todo item as done
    Done {
        index: usize,
    },
    /// Remove a todo item
    Remove {
        index: usize,
    },
}

#[derive(Debug, Serialize, Deserialize)]

// this like interface in typescript
struct Todo {
    description: String,
    completed: bool,
    created_at: DateTime<Local>,
}

struct TodoList {
    todos: Vec<Todo>,
    file_path: PathBuf,
}

// this like class in typescript
impl TodoList {
    // create a new todo list / json file
    fn new() -> Self {
        let file_path = PathBuf::from("todos.json");
        let todos = if file_path.exists() {
            let content = fs::read_to_string(&file_path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Vec::new()
        };
        TodoList { todos, file_path }
    }

    // save the todo list to the json file
    fn save(&self) {
        let content = serde_json::to_string_pretty(&self.todos).unwrap();
        fs::write(&self.file_path, content).unwrap();
    }

    // add a new todo item to the todo list. &mut is mutable reference like it can be changed
    fn add(&mut self, description: String) {
        let todo = Todo {
            description,
            completed: false,
            created_at: Local::now(),
        };
        self.todos.push(todo);
        self.save();
    }

    // list all todo items
    fn list(&self) {
        if self.todos.is_empty() {
            println!("{}", "No todos yet! Add some using `todo add <description>`".yellow());
            return;
        }

        //for loop to iterate over the todo items
        for (i, todo) in self.todos.iter().enumerate() {
            let status = if todo.completed {
                "✓".green()
            } else {
                "✗".red()
            };
            let date = todo.created_at.format("%Y-%m-%d %H:%M");
            println!(
                "{} [{}] {} ({})",
                i + 1,
                status,
                todo.description,
                date.to_string().blue()
            );
        }
    }

    // mark a todo item as done
    fn mark_done(&mut self, index: usize) {
        if let Some(todo) = self.todos.get_mut(index.saturating_sub(1)) {
            todo.completed = true;
            self.save();
            println!("{}", "Todo marked as done!".green());
        } else {
            println!("{}", "Todo not found!".red());
        }
    }

    // remove a todo item
    fn remove(&mut self, index: usize) {
        if index > 0 && index <= self.todos.len() {
            self.todos.remove(index - 1);
            self.save();
            println!("{}", "Todo removed!".green());
        } else {
            println!("{}", "Todo not found!".red());
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let mut todo_list = TodoList::new();

    match cli.command {
        Commands::Add { description } => {
            todo_list.add(description);
            println!("{}", "Todo added!".green());
        }
        Commands::List => todo_list.list(),
        Commands::Done { index } => todo_list.mark_done(index),
        Commands::Remove { index } => todo_list.remove(index),
    }
} 