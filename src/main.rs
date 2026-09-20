use std::error::Error;
use std::fs;
use serde::{Deserialize, Serialize};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rask")]
struct Args {
    // rask command you wanna use
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// add todo - USAGE: rask add [title] 
    Add {
        title: String,
    },
    /// list all todos - USAGE: rask list
    List,
    /// change todo status - USAGE: rask done [id]
    Done {
        id: usize,
    },
    /// delete todo - USAGE: rask delete [id]             
    Delete {
        id: usize,
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: usize,
    title: String,
    done: bool,
} 

fn main() -> Result<(), Box<dyn Error>> {
    // get args from cli
    let args = Args::parse();

    let file_path = "saves.json";

    // get data from json file, if no data new Vec
    let mut todos: Vec<Todo> = if fs::metadata(file_path).is_ok() {
        let content = fs::read_to_string(file_path)?;
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        Vec::new()
    };

    
    // helper to see if file needs to get rewritten
    let mut rewrite_file = false;

    // handle cli args
    match &args.command {
        Commands::Add{title} => {
            // logic for id creation, always takes the biggest and adds 1
            let max_id = todos.iter().map(|t| t.id).max().unwrap_or(0);
            let new_id = max_id + 1;

            todos.push(Todo {
                id: new_id,
                title: title.clone(),
                done: false,
            });
            
            rewrite_file = true;
        }
        Commands::List => {
            if todos.is_empty() {
                println!("no todos in this list");
            } else {
                println!("your todos:");
                for todo in &todos {
                    let status = if todo.done {
                        "Done"
                    } else {
                        "Not done"
                    };
                    println!("id: {}: {}, status: {}", todo.id, todo.title, status);
                }
            }
        }
        Commands::Done{id} => {
            if let Some(todo) = todos.iter_mut().find(|t| t.id == *id) {
                todo.done = true;
                println!("todo {} is marked as done", id);
                
                rewrite_file = true;
            } else {
                println!("can't find todo");
            };

        }
        Commands::Delete{id} => {
            let initial_len = todos.len();
            todos.retain(|t| t.id != *id);

            if todos.len() < initial_len {
                println!("todo {} got removed", id);
                rewrite_file = true;
            } else {
                println!("cant find todo {}", id);
            };

        }
    }

    
    // save todos
    if rewrite_file {
        let json_data = serde_json::to_string_pretty(&todos)?;
        fs::write(file_path, json_data)?;
        println!("Updated your todos.");
    }
    
    Ok(())
}
