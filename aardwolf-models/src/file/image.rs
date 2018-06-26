<<<<<<< HEAD
use chrono::offset::Utc;
use chrono::DateTime;
=======
use chrono::{offset::Utc, DateTime};
>>>>>>> origin/master

use file::File;
use schema::images;

#[derive(Debug, Identifiable, Queryable, QueryableByName)]
#[table_name = "images"]
pub struct Image {
    id: i32,
    width: u32,
    height: u32,
    file_id: i32, // foreign key to File
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Image {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn file_id(&self) -> i32 {
        self.file_id
    }
}

#[derive(Insertable)]
#[table_name = "images"]
pub struct NewImage {
    width: i32,
    height: i32,
    file_id: i32,
}

impl NewImage {
    pub fn new(file: &File, width: u32, height: u32) -> Self {
        NewImage {
            width: width as i32,
            height: height as i32,
            file_id: file.id(),
        }
    }
}
