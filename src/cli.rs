use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rask")]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// add todo - USAGE: rask add [title]
    Add { title: String },
    /// list all todos - USAGE: rask list
    List,
    /// change todo status to done - USAGE: rask done [id]
    Done { id: usize },
    /// change todo status to undone - USAGE: rask undone [id]
    Undone { id: usize },
    /// delete todo - USAGE: rask delete [id]
    Delete { id: usize },
}
