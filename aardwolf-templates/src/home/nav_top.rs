use aardwolf_types::forms::posts::{
    PostCreationFormState, ValidatePostCreationFail, ValidateSourceError,
};
use gettext::Catalog;
use gettext_macros::i18n;

use crate::{Renderable, aside::{Shortcuts}, elements::{Alert, AlertKind, InputSelect, InputText, InputTextarea}};

pub struct NewPost<'a> {
    pub(crate) csrf: &'a str,
    pub(crate) alert: Option<Alert>,
    pub(crate) source: InputTextarea<'a>,
    pub(crate) visibility: InputSelect<'a>,
    pub(crate) name: InputText<'a>,
}

pub struct Home<'a> {
    pub(crate) catalog: &'a Catalog,
    pub(crate) new_post: NewPost<'a>,
    pub(crate) shortcuts: Shortcuts<'a>,
}

impl<'a> Home<'a> {
    pub fn new(
        catalog: &'a Catalog,
        profile_link: &'a str,
        username: &'a str,
        csrf: &'a str,
        state: &'a PostCreationFormState,
        validation_error: Option<&'a ValidatePostCreationFail>,
        server_error: bool,
    ) -> Self {
        Home {
            catalog,
            new_post: NewPost {
                csrf,
                alert: if server_error {
                    Some(Alert {
                        kind: AlertKind::Error,
                        message: i18n!(catalog, "There was an error creating your post"),
                    })
                } else {
                    None
                },
                source: InputTextarea {
                    name: "source",
                    label: None,
                    icon: None,
                    placeholder: Some(i18n!(catalog, "What's on your mind?")),
                    value: &state.source,
                    error: validation_error.and_then(|e| {
                        e.source.as_ref().map(|e| match *e {
                            ValidateSourceError::Empty => i18n!(catalog, "Post cannot be empty"),
                        })
                    }),
                },
                visibility: InputSelect {
                    name: "visibility",
                    label: i18n!(catalog, "Post Visibility"),
                    selected: state.visibility.to_string(),
                    options: InputSelect::visibility_options(catalog),
                    error: validation_error
                        .and_then(|e| e.visibility.as_ref().map(|e| match *e {})),
                },
                name: InputText {
                    name: "name",
                    label: i18n!(catalog, "Content Warning"),
                    icon: None,
                    placeholder: Some(i18n!(catalog, "mh, nsfw, etc.")),
                    value: state.name.as_ref().map(|s| (*s).as_ref()).unwrap_or(""),
                    error: validation_error.and_then(|e| e.name.as_ref().map(|e| match *e {})),
                },
            },
            shortcuts: Shortcuts {
                catalog,
                profile_link,
                username,
            },
        }
    }
}

impl<'a> Renderable for Home<'a> {
    fn render(&self, write: &mut dyn std::io::Write) -> std::io::Result<()> {
        crate::templates::home(write, self)
    }
}
