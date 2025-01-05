use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use chrono::{DateTime, Local};
use clap::{Parser, Subcommand};
use colored::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: usize,
    title: String,
    completed: bool,
    created_at: DateTime<Local>,
}

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple todo CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        #[arg(value_parser = clap::value_parser!(String))]
        title: String,
    },
    List,
    Done {
        id: usize,
    },
    Remove {
        id: usize,
    },
}

impl Todo {
    fn new(id: usize, title: String) -> Self {
        Todo {
            id,
            title,
            completed: false,
            created_at: Local::now(),
        }
    }
}

fn load_todos() -> Vec<Todo> {
    let home = std::env::var("HOME").unwrap();
    let todo_path = PathBuf::from(home).join(".todo.json");
    
    if !todo_path.exists() {
        return Vec::new();
    }

    let contents = fs::read_to_string(todo_path).unwrap();
    serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new())
}

fn save_todos(todos: &[Todo]) {
    let home = std::env::var("HOME").unwrap();
    let todo_path = PathBuf::from(home).join(".todo.json");
    
    let json = serde_json::to_string_pretty(todos).unwrap();
    let mut file = File::create(todo_path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

fn main() {
    let cli = Cli::parse();
    let mut todos = load_todos();

    match cli.command {
        Commands::Add { title } => {
            let id = todos.len() + 1;
            let todo = Todo::new(id, title);
            todos.push(todo);
            println!("{}", "Todo added successfully!".green());
        }
        Commands::List => {
            if todos.is_empty() {
                println!("{}", "No todos found!".yellow());
                return;
            }
            
            for todo in &todos {
                let status = if todo.completed {
                    "✓".green()
                } else {
                    "✗".red()
                };
                println!(
                    "{} [{}] {} ({})",
                    todo.id,
                    status,
                    todo.title,
                    todo.created_at.format("%Y-%m-%d %H:%M")
                );
            }
        }
        Commands::Done { id } => {
            if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                todo.completed = true;
                println!("{}", "Todo marked as done!".green());
            } else {
                println!("{}", "Todo not found!".red());
            }
        }
        Commands::Remove { id } => {
            if let Some(pos) = todos.iter().position(|t| t.id == id) {
                todos.remove(pos);
                println!("{}", "Todo removed successfully!".green());
            } else {
                println!("{}", "Todo not found!".red());
            }
        }
    }

    save_todos(&todos);
}
