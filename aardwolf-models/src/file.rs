use std::path::Path;

use chrono::{offset::Utc, DateTime};
use diesel::{self, pg::PgConnection};
use thiserror::Error;

pub mod image;

use crate::schema::files;

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum FileCreationError {
    #[error("File is missing")]
    Missing,
    #[error("File path contains invalid utf8")]
    Utf8,
}

#[derive(Debug, Identifiable, Queryable, QueryableByName)]
#[diesel(table_name = files)]
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

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

#[derive(Insertable)]
#[diesel(table_name = files)]
pub struct NewFile {
    file_path: String,
}

impl NewFile {
    pub fn insert(self, conn: &mut PgConnection) -> Result<File, diesel::result::Error> {
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
    use crate::test_helper::*;

    #[test]
    fn create_file() {
        with_connection(|conn| {
            let file = make_file(conn);

            assert!(file.is_ok());

            Ok(())
        })
    }

    #[test]
    fn dont_create_nonexistant_file() {
        let new_file = NewFile::new("bad-file-path.invalid");

        assert!(new_file.is_err());
    }
}
