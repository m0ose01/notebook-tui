use std::io::Write;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::utils::CaseExt;

#[derive(Debug)]
pub struct Folder {
    metadata: FolderMetadata,
    pub folders: Vec<Folder>, // TODO: add a nicer way of getting notes
    pub notes: Vec<Note>, // see above
    library: bool,
    path: PathBuf,
}

impl Folder {

    fn initialise(&mut self) -> std::io::Result<()> {
        std::fs::create_dir(&self.path)?;

        let metadata_path = &self.path.join(if self.library {"library.toml"} else {"folder.toml"});
        let mut metadata_file = std::fs::File::create(metadata_path)?;
        metadata_file.write_all(toml::to_string(&self.metadata).expect("could not convert to TOML").as_bytes())?;

        for folder in &mut self.folders {
            folder.initialise()?;
        }
        for note in &mut self.notes {
            note.initialise()?;
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
                path: PathBuf::from(path.as_ref()),
            }
        )
    }

    pub fn add_note(&mut self, title: &str, tags: Vec<String>, author: &str, date: &str) -> std::io::Result<()> {
        let mut note = Note {
            path: self.path.join(title.to_owned().to_kebab_case()),
            metadata: NoteMetadata {title: title.to_owned(), tags, author: author.to_owned(), date: date.to_owned()},
        };
        note.initialise()?;
        self.notes.push(note);
        Ok(())
    }

    pub fn add_folder(&mut self, title: String) -> std::io::Result<()> {
        let mut new_folder = Self {
            folders: vec![],
            notes: vec![],
            metadata: FolderMetadata{title: title.clone(), tags: vec![]},
            library: false,
            path: self.path.join(title.to_kebab_case()),
        };
        new_folder.initialise()?;
        self.folders.push(new_folder);
        Ok(())
    }
}

pub struct FolderBuilder {
    metadata: FolderMetadata,
    folders: Vec<Folder>, // TODO: add a nicer way of getting notes
    notes: Vec<Note>, // see above
    library: bool,
    path: Option<PathBuf>,
}

impl FolderBuilder {
    pub fn new(title: String, library: bool) -> Self {
        Self {
            metadata: FolderMetadata {title, tags: vec![]},
            folders: vec![],
            notes: vec![],
            library,
            path: None,
        }
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.metadata.tags = tags;
        self
    }

    pub fn with_path(mut self, path: impl AsRef<Path>) -> Self {
        self.path = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn build(self) -> std::io::Result<Folder> {
        let path = if self.path.is_some() { self.path.expect("Invalid state: path should be checked for none") } else {
            PathBuf::from(&self.metadata.title.to_kebab_case())
        };
        let mut folder = Folder {
            metadata: self.metadata,
            folders: self.folders,
            notes: self.notes,
            library: self.library,
            path,
        };
        folder.initialise()?;
        Ok(folder)
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
    path: PathBuf,
}

impl Note {
    fn initialise(&mut self) -> std::io::Result<()> {
        std::fs::create_dir(&self.path)?;

        let note_file_name = self.path.join("note.md");
        let metadata_file_name = self.path.join("note.toml");
        std::fs::File::create(note_file_name)?;
        let mut metadata_file = std::fs::File::create(metadata_file_name)?;
        metadata_file.write_all(toml::to_string(&self.metadata).expect("could not convert to TOML").as_bytes())?;
        Ok(())
    }

    fn open<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let metadata_file_name = path.as_ref().join("note.toml");
        let metadata_file = std::fs::read_to_string(metadata_file_name)?;
        let metadata = toml::from_str(&metadata_file).expect("could not convert from TOML");
        Ok(
            Note {
                metadata,
                path: PathBuf::from(path.as_ref()),
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
