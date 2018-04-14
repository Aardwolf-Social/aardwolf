use std::path::{Path, PathBuf};

use chrono::DateTime;
use chrono::offset::Utc;

pub mod image;

use schema::files;

#[derive(Clone, Copy, Debug, Eq, Fail, PartialEq)]
pub enum FileCreationError {
    #[fail(display = "File is missing")]
    Missing,
    #[fail(display = "File path contains invalid utf8")]
    Utf8,
}

#[derive(Debug, Identifiable, Queryable, QueryableByName)]
#[table_name = "files"]
pub struct File {
    id: i32,
    file_path: PathBuf,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl File {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn path(&self) -> &Path {
        &self.file_path
    }
}

#[derive(Insertable)]
#[table_name = "files"]
pub struct Newfile {
    file_path: String,
}

impl Newfile {
    pub fn new<P>(path: P) -> Result<Self, FileCreationError>
    where
        P: AsRef<Path>,
    {
        if path.as_ref().is_file() {
            path.as_ref()
                .to_str()
                .map(|p| Newfile {
                    file_path: p.to_owned(),
                })
                .ok_or(FileCreationError::Utf8)
        } else {
            Err(FileCreationError::Missing)
        }
    }
}
