use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
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
    // add todo
    add {
        title: String,
    },
    // list all todos
    list,
    // change todo status
    done {
        id: u8,
    },
    // delete todo              
    delete {
        id: u8,
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: u8,
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
    
    // handle cli args
    match &args.command {
        Commands::add{title} => {
            todos.push(Todo {
                id: (todos.len() as u8) + 1,
                title: title.clone(),
                done: false,
            });
        }
        Commands::list => {
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
        Commands::done{id} => {
            let index: usize = (id - 1).into();
            let mut todo_found: bool = false;
            for todo in &todos {
                if *id == todo.id {
                    todo_found = true;
                }
            };

            if todo_found == false {
                println!("cant find todo {}", id);
            } else {
                if todos[index].done == false {
                    todos[index].done = true;
                }
            }
        }
        Commands::delete{id} => {
            let index: usize = (id - 1).into();
            todos.remove(index);
            println!("todo {} got removed", id);
        }
    }

    let json_data = serde_json::to_string_pretty(&todos)?;
    
    // save todos
    fs::write(file_path, json_data);
    println!("Updated your todos.");
    
    Ok(())
}
