// use std::path::{Path, PathBuf};
use std::path::Path;

use chrono::DateTime;
use chrono::offset::Utc;
use diesel;
use diesel::pg::PgConnection;

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
    file_path: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl File {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn path(&self) -> &str {
        &self.file_path
    }
}

#[derive(Insertable)]
#[table_name = "files"]
pub struct NewFile {
    file_path: String,
}

impl NewFile {
    pub fn insert(self, conn: &PgConnection) -> Result<File, diesel::result::Error> {
        use diesel::prelude::*;

        diesel::insert_into(files::table)
            .values(&self)
            .get_result(conn)
    }

    pub fn new<P>(path: P) -> Result<Self, FileCreationError>
    where
        P: AsRef<Path>,
    {
        if path.as_ref().is_file() {
            path.as_ref()
                .to_str()
                .map(|p| NewFile {
                    file_path: p.to_owned(),
                })
                .ok_or(FileCreationError::Utf8)
        } else {
            Err(FileCreationError::Missing)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NewFile;
    use test_helper::*;

    #[test]
    fn create_file() {
        with_connection(|conn| with_file(conn, |_| Ok(())))
    }

    #[test]
    fn dont_create_nonexistant_file() {
        let new_file = NewFile::new("bad-file-path.invalid");

        assert!(new_file.is_err());
    }
}
