use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: u8,
    title: String,
    done: bool,
} 

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "saves.json";

    // init json file
    let mut todos: Vec<Todo> = if fs::metadata(file_path).is_ok() {
        let content = fs::read_to_string(file_path)?;
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        Vec::new()
    };
    
    println!("{:?}", todos);

    let new_todo = Todo {
        id: (todos.len() as u8) + 1,
        title: "Work".to_string(),
        done: false,
    };

    todos.push(new_todo);

    let json_data = serde_json::to_string_pretty(&todos)?;
    
    // save todos
    fs::write(file_path, json_data);
    println!("Json Data is written");
    
    Ok(())
}
