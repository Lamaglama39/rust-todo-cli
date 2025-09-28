use clap::{Parser, Subcommand};
use todo_cli::Todo;
use rusqlite::Result;

#[derive(Parser)]
#[command(name = "todo-cli")]
#[command(about = "A simple command-line todo application")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new todo item
    Add {
        /// The task to add
        task: String,
    },
    /// Mark a todo item as completed
    Complete {
        /// The task to complete
        task: String,
    },
    /// List all todo items
    List,
    /// Delete a todo item
    Delete {
        /// The task to delete
        task: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let todo = Todo::new()?;

    match cli.command {
        Commands::Add { task } => {
            todo.insert(task.clone())?;
            println!("Added: '{}'", task);
        }
        Commands::Complete { task } => {
            if todo.complete(&task)? {
                println!("Completed: '{}'", task);
            } else {
                println!("Task '{}' not found or already completed", task);
            }
        }
        Commands::List => {
            println!("Todo List:");
            todo.print_list()?;
        }
        Commands::Delete { task } => {
            if todo.delete(&task)? {
                println!("Deleted: '{}'", task);
            } else {
                println!("Task '{}' not found", task);
            }
        }
    }

    Ok(())
}