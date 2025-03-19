use std::io::Write;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Folder {
    metadata: FolderMetadata,
    pub folders: Vec<Folder>, // TODO: add a nicer way of getting notes
    pub notes: Vec<Note>, // see above
    library: bool,
    path: Option<PathBuf>,
}

impl Folder {
    pub fn new(title: &str, tags: Vec<String>, folders: Vec<Folder>, notes: Vec<Note>, library: bool) -> Self {
        // TODO: create some kind of 'builder' instead of having multiple ways of adding
        // folders/notes.
        let title = title.to_string();
        Self {
            folders,
            notes,
            metadata: FolderMetadata{title, tags},
            library,
            path: None,
        }
    }

    pub fn initialise(&mut self, parent_folder: &impl AsRef<Path>) -> std::io::Result<()> {
        let subdirectory_name = &self.metadata.title.to_ascii_lowercase().replace(" ", "-");
        let directory_path =  parent_folder.as_ref().join(subdirectory_name);
        if let None = self.path {
            std::fs::create_dir(&directory_path)?;

            let metadata_path = directory_path.join(if self.library {"library.toml"} else {"folder.toml"});
            let mut metadata_file = std::fs::File::create(metadata_path)?;
            metadata_file.write_all(toml::to_string(&self.metadata).expect("could not convert to TOML").as_bytes())?;

            self.path = Some(PathBuf::from(&directory_path));
        }
        for folder in &mut self.folders {
            folder.initialise(&directory_path)?;
        }
        for note in &mut self.notes {
            note.initialise(&directory_path)?;
        }
        Ok(())
    }

    pub fn open(path: impl AsRef<Path>) -> std::io::Result<Self> {

        let library_metadata_path = path.as_ref().join("library.toml");
        println!("{:?}", library_metadata_path);
        let folder_metadata_path = path.as_ref().join("folder.toml");
        println!("{:?}", folder_metadata_path);

        let library = library_metadata_path.is_file();

        let metadata_file = match std::fs::read_to_string(&library_metadata_path) {
            Ok(f) => f,
            Err(_) => std::fs::read_to_string(folder_metadata_path)?,
        };
        let metadata = toml::from_str(&metadata_file).expect("Could not read folder metadata from TOML");

        // This is probably not the most efficient way of doing things, but it's simple
        let folders: Vec<Folder> = std::fs::read_dir(path.as_ref())?
            .filter_map(
                |n| Folder::open(n.expect("Failed to read directory").path()).ok()
            )
            .collect();

        let notes: Vec<Note> = std::fs::read_dir(path.as_ref())?
            .filter_map(
                |n| Note::open(n.expect("Failed to read directory").path()).ok()
            )
            .collect();
        Ok(
            Self {
                folders,
                notes,
                metadata,
                library,
                path: Some(PathBuf::from(path.as_ref())),
            }
        )
    }

    pub fn add_note(&mut self, mut note: Note) -> std::io::Result<()> {
        if let Some(parent_path) = &self.path {
            note.initialise(parent_path)?;
        }
        self.notes.push(note);
        Ok(())
    }

    pub fn add_folder(&mut self, mut folder: Folder) -> std::io::Result<()> {
        if let Some(parent_path) = &self.path {
            folder.initialise(parent_path)?;
        }
        self.folders.push(folder);
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct FolderMetadata {
    title: String,
    tags: Vec<String>,
}

#[derive(Debug)]
pub struct Note {
    metadata: NoteMetadata,
    path: Option<PathBuf>,
}

impl Note {
    pub fn new(title: &str, tags: Vec<String>, author: &str, date: &str) -> Self {
        let title = title.to_string();
        let author = author.to_string();
        let date = date.to_string();

        Self {
            metadata: NoteMetadata { title, tags, author, date },
            path: None,
        }
    }

    fn initialise(&mut self, parent_folder: impl AsRef<Path>) -> std::io::Result<()> {
        if let None = self.path {
            let subdirectory_name = &self.metadata.title.to_ascii_lowercase().replace(" ", "-");
            let directory_path = parent_folder.as_ref().join(subdirectory_name);
            std::fs::create_dir(&directory_path)?;

            let note_file_name = directory_path.join("note.md");
            let metadata_file_name = directory_path.join("note.toml");
            std::fs::File::create(note_file_name)?;
            let mut metadata_file = std::fs::File::create(metadata_file_name)?;
            metadata_file.write_all(toml::to_string(&self.metadata).expect("could not convert to TOML").as_bytes())?;
            self.path = Some(PathBuf::from(&subdirectory_name));
        }
        Ok(())
    }

    fn open<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let metadata_file_name = path.as_ref().join("note.toml");
        let metadata_file = std::fs::read_to_string(metadata_file_name)?;
        let metadata = toml::from_str(&metadata_file).expect("could not convert from TOML");
        Ok(
            Note {
                metadata,
                path: Some(PathBuf::from(path.as_ref())),
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
