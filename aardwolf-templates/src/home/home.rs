use aardwolf_types::forms::posts::{
    PostCreationFormState, ValidatePostCreationFail, ValidateSourceError,
};
use gettext::Catalog;
use gettext_macros::i18n;

use crate::{
    asides::Shortcuts,
    elements::{Alert, AlertKind, InputSelect, InputText, InputTextarea},
    home::Feed,
    home::NavTop,
    posts::NewPost,
    Renderable,
};

pub struct Home<'a> {
    pub(crate) catalog: &'a Catalog,
    pub(crate) new_post: NewPost<'a>,
    pub(crate) shortcuts: Shortcuts<'a>,
    pub(crate) nav_top: NavTop<'a>,
    pub(crate) feed: Feed<'a>,
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
            nav_top: NavTop { catalog },
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
                catalog: &catalog,
                username: username,
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
            feed: Feed { catalog },
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
        crate::templates::home::home(write, self)
    }
}
