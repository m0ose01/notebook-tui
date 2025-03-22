mod note;
mod tui;
mod utils;

use clap::{Parser, Subcommand};

use crate::{
    note::{Folder, LibraryBuilder},
};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new library.
    New { name: String },
    /// Open an existing library.
    Open { name: String },
}

fn main() -> std::io::Result<()> {

    let args = Args::parse();

    // TODO; Define directory structure for notes, allow tags, folders.

    // TODO: Implement CLI for creating new markdown notes.

    // TODO: Automatically open notes in a program configured by the user, e.g., neovim, VSCode.

    // TODO: Allow searching, filtering, etc.

    // TODO: Support rendering to pdf/html, maybe include server.

    // TODO: Implement backlinks to specific notes.

    // TODO: Implement creation of notes from PDF slides.

    // TODO: if possible, redefine library as a folder, and have some kind of way to distinguish
    // 'libraries' from their subfolders, maybe by redefining library/folder as traits?

    if let Commands::New { name: title } = &args.command {
        let mut library: Folder = LibraryBuilder::new(&title)
            .with_tags(vec!["College".to_owned()])
            .build()?;

        library.add_folder("Nested Folder".to_owned())?;
        library.add_note("Top Level Note", vec!["Physiology".to_owned()], "John Smith", "2025/03/22")?;
    }

    if let Commands::Open { name: title } = &args.command {
        let mut library = Folder::open_library(&title)?;
        //library.add_note("Top Level Note 2", vec!["Physiology".to_owned()], "John Doe", "2025/03/22")?;
        let idx = tui::run(&library)?;
        library.notes[idx].edit("nvim");
    }

    Ok(())
}

