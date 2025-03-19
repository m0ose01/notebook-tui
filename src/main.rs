mod note;

use clap::{Parser, Subcommand};

use crate::note::{Folder, Note};

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
        let tag = "mytag".to_string();
        let folder = Folder::new("Test Folder", vec![tag.clone()], vec![], vec![], false);
        let mut library = Folder::new(&title, vec![tag.clone()], vec![folder], vec![], true);
        library.initialise(&std::path::PathBuf::from("."))?;

        let nested_folder = Folder::new("Test Nested Folder", vec![tag.clone()], vec![], vec![], false);
        library.folders[0].add_folder(nested_folder)?;

        let note = Note::new("Test Note", vec![tag.clone()], "me", "2025-03-17");
        library.folders[0].folders[0].add_note(note)?;

    }

    if let Commands::Open { name: title } = &args.command {
        let library = Folder::open(&title)?;
        println!("{:#?}", library);
    }

    Ok(())
}

