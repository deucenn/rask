mod cli;
mod models;
mod storage;

use std::error::Error;
use clap::Parser;
use std::path::PathBuf;
use cli::{Args, Commands};
use models::TodoList;

fn main() -> Result<(), Box<dyn Error>> {
    // get args from cli
    let args = Args::parse();
    
    let file_path: PathBuf = storage::get_file_path()?;
    let mut todo_list = TodoList::new(storage::load_todos(&file_path)?);
    let mut modified = false;

    // handle cli args
    match &args.command {
        Commands::Add{title} => {
            todo_list.add(title.clone());
            storage::save_todos(&file_path, &todo_list.items)?;
            todo_list.list();
        }
        Commands::List => {
            todo_list.list();
        }
        Commands::Done{id} => {
            todo_list.mark_done(id);
            storage::save_todos(&file_path, &todo_list.items)?;
            todo_list.list();
        }
        Commands::Undone{id} => {
            todo_list.mark_undone(id);
            storage::save_todos(&file_path, &todo_list.items)?;
            todo_list.list();
        }
        Commands::Delete{id} => {
            todo_list.delete(id);
            storage::save_todos(&file_path, &todo_list.items)?;
            todo_list.list();
        }
        Commands::Clear => {
            todo_list.clear();
            storage::save_todos(&file_path, &todo_list.items)?;
            todo_list.list();
        }
    }

    Ok(())
}
