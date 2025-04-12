use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;

use jiff::{Zoned, Timestamp};
use serde::{Deserialize, Serialize};
use toml::value::Datetime;

use crate::utils::CaseExt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Folder {
    metadata: FolderMetadata,
    pub folders: Vec<Folder>, // TODO: add a nicer way of getting notes
    pub notes: Vec<Note>, // see above
    pub library: bool,
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

    pub fn open_library(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let library_metadata_path = path.as_ref().join("library.toml");
        Folder::open(library_metadata_path)
    }

    fn open(metadata_path: impl AsRef<Path>) -> std::io::Result<Self> {

        let metadata_file = std::fs::read_to_string(&metadata_path)?;
        let library = metadata_path.as_ref().file_name().expect("Could not get file name").to_str() == Some("library.toml");
        let metadata = toml::from_str(&metadata_file).expect("Could not read folder metadata from TOML");

        // This is probably not the most efficient way of doing things, but it's simple
        let folder_parent_path = &metadata_path.as_ref().parent().expect("No parent folder for folder metadata.");
        let mut folders: Vec<Folder> = std::fs::read_dir(folder_parent_path)?
            .filter_map(
                |n| Folder::open(n.expect("Failed to read directory").path().join("folder.toml")).ok()
            )
            .collect();
        folders.sort_by_key(|folder| folder.metadata.title.clone());

        let note_parent_path = &metadata_path.as_ref().parent().expect("No parent folder for note metadata.");
        let mut notes: Vec<Note> = std::fs::read_dir(note_parent_path)?
            .filter_map(
                |n| Note::open(n.expect("Failed to read directory").path()).ok()
            )
            .collect();
        notes.sort_by_key(|note| note.metadata.title.clone());
        Ok(
            Self {
                folders,
                notes,
                metadata,
                library,
                path: PathBuf::from(folder_parent_path),
            }
        )
    }

    pub fn add_note(&mut self, title: &str, tags: Vec<String>, author: &str, date: &Zoned) -> std::io::Result<()> {
        let date = Datetime::from_str(&date.timestamp().to_string()).expect(&format!("Could not parse date: {}", &date.to_string()));
        let mut note = Note {
            path: self.path.join(title.to_owned().to_kebab_case()),
            metadata: NoteMetadata {title: title.to_owned(), tags, author: author.to_owned(), date},
        };
        note.initialise()?;
        self.notes.push(note);
        Ok(())
    }

    pub fn add_folder(&mut self, title: &str) -> std::io::Result<()> {
        let title = title.to_owned();
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

    pub fn title(&self) -> &str {
        &self.metadata.title
    }
}

pub struct LibraryBuilder {
    metadata: FolderMetadata,
    folders: Vec<Folder>, // TODO: add a nicer way of getting notes
    notes: Vec<Note>, // see above
    library: bool,
    path: Option<PathBuf>,
}

impl LibraryBuilder {
    pub fn new(title: &str) -> Self {
        Self {
            metadata: FolderMetadata {title: title.to_owned(), tags: vec![]},
            folders: vec![],
            notes: vec![],
            library: true,
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct FolderMetadata {
    title: String,
    tags: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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

    pub fn edit(&self, editor: impl AsRef<Path>) {
        let note_path = &self.path.join("note.md");
        Command::new(editor.as_ref().as_os_str())
            .arg(&note_path)
            .status()
            .expect("Unable to spawn process");
    }

    pub fn title(&self) -> &str {
        &self.metadata.title
    }

    pub fn author(&self) -> &str {
        &self.metadata.author
    }

    pub fn date(&self) -> Result<Timestamp, jiff::Error> {
        Timestamp::from_str(
            &self.metadata.date.to_string()
        )
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct NoteMetadata {
    title: String,
    tags: Vec<String>,
    author: String, // TODO: implement an author type
    date: Datetime,
}
