fn main() -> std::io::Result<()> {

    // TODO; Define directory structure for notes, allow tags, folders.

    // TODO: Implement CLI for creating new markdown notes.

    // TODO: Automatically open notes in a program configured by the user, e.g., neovim, VSCode.

    // TODO: Allow searching, filtering, etc.

    // TODO: Support rendering to pdf/html, maybe include server.

    // TODO: Implement backlinks to specific notes.

    // TODO: Implement creation of notes from PDF slides.

    let tag = "mytag".to_string();
    let note = Note::new("Test Note", vec![tag.clone()], "me", "2025-03-17");
    let folder = Folder::new("Test Folder", vec![tag.clone()], vec![note]);
    let mut library = Library::new("Test Lib", vec![tag], vec![folder]);

    library.initialise()?;

    Ok(())
}

struct Library {
    title: String,
    tags: Vec<String>,
    folders: Vec<Folder>,
    initialised: bool,
}

impl Library {
    fn new(title: &str, tags: Vec<String>, folders: Vec<Folder>) -> Self {
        let title = title.to_string();
        Self {
            title,
            tags,
            folders,
            initialised: false,
        }
    }

    fn initialise(&mut self) -> std::io::Result<()> {
        let directory_name = &self.title.to_ascii_lowercase().replace(" ", "-");
        std::fs::create_dir(&directory_name)?;

        let metadata_file_name = format!("{}/{}", directory_name, "library.toml");
        std::fs::File::create(&metadata_file_name)?;

        for folder in &mut self.folders {
            folder.initialise(&directory_name)?;
        }
        self.initialised = true;
        Ok(())
    }
}

struct Folder {
    title: String,
    tags: Vec<String>,
    notes: Vec<Note>,
    initialised: bool,
}

impl Folder {
    fn new(title: &str, tags: Vec<String>, notes: Vec<Note>) -> Self {
        let title = title.to_string();
        Self {
            title,
            tags,
            notes,
            initialised: false,
        }
    }

    fn initialise(&mut self, parent_folder: &str) -> std::io::Result<()> {
        let subdirectory_name = &self.title.to_ascii_lowercase().replace(" ", "-");
        let directory_name = format!("{}/{}", parent_folder, subdirectory_name);
        std::fs::create_dir(&directory_name)?;

        let metadata_file_name = format!("{}/{}", directory_name, "folder.toml");
        std::fs::File::create(&metadata_file_name)?;

        for note in &mut self.notes {
            note.initialise(&directory_name)?;
        }
        self.initialised = true;
        Ok(())
    }
}

struct Note {
    title: String,
    tags: Vec<String>,
    author: String, // TODO: implement an author type
    date: String, // TODO: change this to a date type from a suitable crate
    initialised: bool,
}

impl Note {
    fn new(title: &str, tags: Vec<String>, author: &str, date: &str) -> Self {
        let title = title.to_string();
        let author = author.to_string();
        let date = date.to_string();

        Self {
            title,
            tags,
            author,
            date,
            initialised: false,
        }
    }

    fn initialise(&mut self, parent_folder: &str) -> std::io::Result<()> {
        let subdirectory_name = &self.title.to_ascii_lowercase().replace(" ", "-");
        let directory_name = format!("{}/{}", parent_folder, subdirectory_name);
        std::fs::create_dir(&directory_name)?;

        let note_file_name = format!("{}/{}", directory_name, "note.md");
        let metadata_file_name = format!("{}/{}", directory_name, "note.toml");
        std::fs::File::create(note_file_name)?;
        std::fs::File::create(metadata_file_name)?;
        self.initialised = true;
        Ok(())
    }
}
