use chrono::{offset::Utc, DateTime};

use crate::{
    base_post::BasePost,
    schema::links,
    sql_types::{Lang, Url},
};

#[derive(Debug, Identifiable, Queryable, QueryableByName)]
#[table_name = "links"]
pub struct Link {
    id: i32,
    href: Url, // max_length: 2048
    href_lang: Lang,
    height: u32,
    width: u32,
    preview: String,
    base_post: i32, // foreign key to BasePost
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Link {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn href(&self) -> &Url {
        &self.href
    }

    pub fn href_lang(&self) -> Lang {
        self.href_lang
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn preview(&self) -> &str {
        &self.preview
    }

    pub fn base_post(&self) -> i32 {
        self.base_post
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

#[derive(Insertable)]
#[table_name = "links"]
pub struct NewLink {
    href: Url,
    href_lang: Lang,
    height: i32,
    width: i32,
    preview: String,
    base_post: i32,
}

impl NewLink {
    pub fn new<U: Into<Url>>(
        href: U,
        href_lang: Lang,
        height: u32,
        width: u32,
        preview: String,
        base_post: &BasePost,
    ) -> Self {
        NewLink {
            href: href.into(),
            href_lang,
            height: height as i32,
            width: width as i32,
            preview,
            base_post: base_post.id(),
        }
    }
}
