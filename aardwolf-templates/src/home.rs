use gettext::Catalog;

use crate::{Renderable, Shortcuts};

pub struct Home<'a> {
    pub(crate) catalog: &'a Catalog,
    pub(crate) shortcuts: Shortcuts<'a>,
}

impl<'a> Home<'a> {
    pub fn new(catalog: &'a Catalog, profile_link: &'a str, username: &'a str) -> Self {
        Home {
            catalog,
            shortcuts: Shortcuts {
                catalog,
                profile_link,
                username,
            },
        }
    }
}

impl<'a> Renderable for Home<'a> {
    fn render(self, write: &mut std::io::Write) -> std::io::Result<()> {
        crate::templates::home(write, self)
    }
}
