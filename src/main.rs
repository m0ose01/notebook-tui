mod cli;
mod note;
mod tui;
mod utils;

use std::error::Error;

use clap::Parser;
use ratatui;

use crate::{
    note::{Folder, LibraryBuilder},
    cli::{Args, Commands},
};

fn main() -> Result<(), Box<dyn Error>> {

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

    if let Commands::New(subcommand_args) = &args.command {
        let mut library = LibraryBuilder::new(&subcommand_args.name)
            .with_tags(vec!["College".to_owned()]);
        if let Some(path) = &subcommand_args.path {
            library = library.with_path(path);
        }

        let mut library = library.build()?;

        if let Some(editor) = &subcommand_args.editor {
            let mut terminal = ratatui::init();
            library.run(&mut terminal, editor)?;
            ratatui::restore();
        }

    }

    if let Commands::Open(subcommand_args) = &args.command {
        let mut library = Folder::open_library(&subcommand_args.name)?;

        let mut terminal = ratatui::init();
        let editor = &subcommand_args.editor.clone().unwrap_or("nvim".to_owned());
        library.run(&mut terminal, editor)?;
        ratatui::restore();
    }

    Ok(())
}

