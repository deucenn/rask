use std::error::Error;
use std::fs;
use dirs;
use std::path::PathBuf; 
use crate::models::Todo;

pub fn get_file_path() -> Result<PathBuf, Box<dyn Error>> {
    // moved the save filen location to the app data folder
    let mut app_dir = dirs::data_local_dir().expect("cant find app data folder");
    app_dir.push("rask");

    fs::create_dir_all(&app_dir)?;

    let file_path = app_dir.join("saves.json");

    return Ok(file_path);
}

pub fn load_todos(path: &PathBuf) -> Result<Vec<Todo>, Box<dyn Error>> {
    if fs::metadata(&path).is_ok() {
        let content = fs::read_to_string(&path)?;
        Ok(serde_json::from_str(&content).unwrap_or_default())
    } else {
        Ok(Vec::new())
    }
}

pub fn save_todos(path: &PathBuf, todos: &[Todo]) -> Result<(), Box<dyn Error>> {
    let json_data = serde_json::to_string_pretty(&todos)?;
    fs::write(path, json_data)?;
    Ok(()) 
}
