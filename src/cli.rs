use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new library.
    New(NewArgs),
    /// Open an existing library.
    Open(OpenArgs),
}

#[derive(Parser)]
pub struct NewArgs {
    pub name: String,
    #[arg(short, long)]
    pub path: Option<PathBuf>,
    #[arg(short, long)]
    pub editor: Option<String>,
}

#[derive(Parser)]
pub struct OpenArgs {
    pub name: String,
    #[arg(short, long)]
    pub editor: Option<String>,
}

