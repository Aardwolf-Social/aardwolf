use gettext::Catalog;

use crate::Renderable;

pub struct Feed<'a> {
    pub(crate) catalog: &'a Catalog,
}

impl<'a> Feed<'a> {
    pub fn new(catalog: &'a Catalog) -> Self {
        Feed { catalog }
    }
}

impl<'a> Renderable for Feed<'a> {
    fn render(&self, write: &mut dyn std::io::Write) -> std::io::Result<()> {
        crate::templates::home::feed(write, self)
    }
}
