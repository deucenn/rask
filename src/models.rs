use serde::{Deserialize, Serialize};
use chrono::{Local};
use comfy_table::{Cell, ContentArrangement, Row, Table};

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    id: usize,
    title: String,
    done: bool,
    creation_date: String,
}

pub struct TodoList {
    pub items: Vec<Todo>
}

impl TodoList {
    pub fn new(items: Vec<Todo>) -> Self {
        Self {items}
    }

    pub fn list(&self) {
        let mut table = Table::new();
        table.set_content_arrangement(ContentArrangement::Dynamic);
        table.set_width(85);
        table.set_header(vec!["ID","Task","Status","Created"]);

        for todo in &self.items {
            let status = if todo.done {
                "Done"
            } else {
                "Not done"
            };

            table.add_row([todo.id.to_string(), todo.title.clone(), status.to_string(), todo.creation_date.to_string()]);
        }

        println!("{table}");
    }
    
    pub fn add(&mut self, title: String) {
        // logic for id creation, always takes the biggest and adds 1
        let max_id = self.items.iter().map(|t| t.id).max().unwrap_or(0);
        let new_id = max_id + 1;

        // get current time
        let date_now = Local::now().format("%Y/%m/%d").to_string();

        self.items.push(Todo {
             id: new_id,
             title: title.clone(),
             done: false,
             creation_date: date_now,
        });
    }
    
    pub fn mark_done(&mut self, id: &usize) {
        if let Some(todo) = self.items.iter_mut().find(|t| t.id == *id) {
            todo.done = true;
            println!("todo {} is marked as done", id);
        } else {
            println!("can't find todo");
        }
    }

    pub fn mark_undone(&mut self, id: &usize) {
        if let Some(todo) = self.items.iter_mut().find(|t| t.id == *id) {
            todo.done = false;
            println!("todo {} is marked as undone", id);
        } else {
            println!("can't find todo");
        }
    }

    pub fn delete(&mut self, id: &usize) {
        let initial_len = self.items.len();
        self.items.retain(|t| t.id != *id);

        if self.items.len() < initial_len {
            println!("todo {} got removed", id);
        } else {
            println!("cant find todo {}", id);
        }
    }

    pub fn clear(&mut self) {
        let initial_len = self.items.len();
        self.items.retain(|t| t.done == false);
    }
}
