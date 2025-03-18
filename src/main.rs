use std::io::Write;
use std::path::Path;

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

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

    if let Commands::New { name: title } = &args.command {
        let tag = "mytag".to_string();
        let note = Note::new("Test Note", vec![tag.clone()], "me", "2025-03-17");
        let folder = Folder::new("Test Folder", vec![tag.clone()], vec![note]);
        let mut library = Library::new(&title, vec![tag], vec![folder]);

        library.initialise()?;
    }

    if let Commands::Open { name: title } = &args.command {
        let library = Library::open(&title)?;
        println!("{:?}", library);
    }

    Ok(())
}

#[derive(Debug)]
struct Library {
    metadata: LibraryMetadata,
    folders: Vec<Folder>,
    initialised: bool,
}

impl Library {
    fn new(title: &str, tags: Vec<String>, folders: Vec<Folder>) -> Self {
        let title = title.to_string();
        Self {
            folders,
            metadata: LibraryMetadata{title, tags},
            initialised: false,
        }
    }

    fn initialise(&mut self) -> std::io::Result<()> {
        let directory_name = &self.metadata.title.to_ascii_lowercase().replace(" ", "-");
        std::fs::create_dir(&directory_name)?;

        let metadata_file_name = format!("{}/{}", directory_name, "library.toml");
        let mut metadata_file = std::fs::File::create(metadata_file_name)?;
        metadata_file.write_all(toml::to_string(&self.metadata).expect("could not convert to TOML").as_bytes())?;

        for folder in &mut self.folders {
            folder.initialise(&directory_name)?;
        }
        self.initialised = true;
        Ok(())
    }

    fn open(path: impl AsRef<Path>) -> std::io::Result<Self> {

        let metadata_file_name = path.as_ref().join("library.toml");
        let metadata_file = std::fs::read_to_string(metadata_file_name).unwrap();
        let metadata = toml::from_str(&metadata_file).unwrap();

        let folders: Vec<Folder> = std::fs::read_dir(path.as_ref())?
            .filter_map(
                |n| Folder::open(n.expect("Failed to read directory").path()).ok()
            )
            .collect();
        Ok(
            Self {
                folders,
                metadata,
                initialised: true,
            }
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct LibraryMetadata {
    title: String,
    tags: Vec<String>,
}

#[derive(Debug)]
struct Folder {
    metadata: FolderMetadata,
    notes: Vec<Note>,
    initialised: bool,
}

impl Folder {
    fn new(title: &str, tags: Vec<String>, notes: Vec<Note>) -> Self {
        let title = title.to_string();
        Self {
            notes,
            metadata: FolderMetadata{title, tags},
            initialised: false,
        }
    }

    fn initialise(&mut self, parent_folder: &str) -> std::io::Result<()> {
        let subdirectory_name = &self.metadata.title.to_ascii_lowercase().replace(" ", "-");
        let directory_name = format!("{}/{}", parent_folder, subdirectory_name);
        std::fs::create_dir(&directory_name)?;

        let metadata_file_name = format!("{}/{}", directory_name, "folder.toml");
        let mut metadata_file = std::fs::File::create(metadata_file_name)?;
        metadata_file.write_all(toml::to_string(&self.metadata).expect("could not convert to TOML").as_bytes())?;

        for note in &mut self.notes {
            note.initialise(&directory_name)?;
        }
        self.initialised = true;
        Ok(())
    }

    fn open(path: impl AsRef<Path>) -> std::io::Result<Self> {

        let metadata_file_name = path.as_ref().join("folder.toml");
        let metadata_file = std::fs::read_to_string(metadata_file_name)?;
        let metadata = toml::from_str(&metadata_file).expect("Could not read folder metadata from TOML");

        let notes: Vec<Note> = std::fs::read_dir(path.as_ref())?
            .filter_map(
                |n| Note::open(n.expect("Failed to read directory").path()).ok()
            )
            .collect();
        Ok(
            Self {
                notes,
                metadata,
                initialised: true,
            }
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct FolderMetadata {
    title: String,
    tags: Vec<String>,
}

#[derive(Debug)]
struct Note {
    metadata: NoteMetadata,
    initialised: bool,
}

impl Note {
    fn new(title: &str, tags: Vec<String>, author: &str, date: &str) -> Self {
        let title = title.to_string();
        let author = author.to_string();
        let date = date.to_string();

        Self {
            metadata: NoteMetadata { title, tags, author, date },
            initialised: false,
        }
    }

    fn initialise(&mut self, parent_folder: &str) -> std::io::Result<()> {
        let subdirectory_name = &self.metadata.title.to_ascii_lowercase().replace(" ", "-");
        let directory_name = format!("{}/{}", parent_folder, subdirectory_name);
        std::fs::create_dir(&directory_name)?;

        let note_file_name = format!("{}/{}", directory_name, "note.md");
        let metadata_file_name = format!("{}/{}", directory_name, "note.toml");
        std::fs::File::create(note_file_name)?;
        let mut metadata_file = std::fs::File::create(metadata_file_name)?;
        metadata_file.write_all(toml::to_string(&self.metadata).expect("could not convert to TOML").as_bytes())?;
        self.initialised = true;
        Ok(())
    }

    fn open<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let metadata_file_name = path.as_ref().join("note.toml");
        let metadata_file = std::fs::read_to_string(metadata_file_name)?;
        let metadata = toml::from_str(&metadata_file).expect("could not convert from TOML");
        Ok(
            Note {
                metadata,
                initialised: true,
            }
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct NoteMetadata {
    title: String,
    tags: Vec<String>,
    author: String, // TODO: implement an author type
    date: String, // TODO: change this to a date type from a suitable crate
}
