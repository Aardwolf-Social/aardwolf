use aardwolf_types::forms::posts::{
    PostCreationFormState, ValidatePostCreationFail,
};
use gettext::Catalog;

use crate::{
    elements::{Alert, InputSelect, InputText, InputTextarea},
    Renderable,
};

pub struct NewPost<'a> {
    pub(crate) csrf: &'a str,
    pub(crate) alert: Option<Alert>,
    pub(crate) source: InputTextarea<'a>,
    pub(crate) visibility: InputSelect<'a>,
    pub(crate) name: InputText<'a>,
}

pub struct Feed<'a> {
    pub(crate) catalog: &'a Catalog,
}

impl<'a> Feed<'a> {
    pub fn new(
        catalog: &'a Catalog,
        profile_link: &'a str,
        username: &'a str,
        csrf: &'a str,
        state: &'a PostCreationFormState,
        validation_error: Option<&'a ValidatePostCreationFail>,
        server_error: bool,
    ) -> Self {
        Feed {
            catalog,
        }
    }
}

impl<'a> Renderable for Feed<'a> {
    fn render(&self, write: &mut dyn std::io::Write) -> std::io::Result<()> {
        crate::templates::home::feed(write, self)
    }
}
