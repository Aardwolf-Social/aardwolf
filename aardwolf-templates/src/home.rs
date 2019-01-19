use aardwolf_types::forms::posts::{PostCreationFormState, ValidatePostCreationFail};
use gettext::Catalog;

use crate::{
    Alert, AlertKind, Renderable, SelectInput, SelectOption, Shortcuts, TextInput, TextareaInput,
};

pub struct NewPost<'a> {
    pub(crate) csrf: &'a str,
    pub(crate) alert: Option<Alert<'a>>,
    pub(crate) source: TextareaInput<'a>,
    pub(crate) visibility: SelectInput<'a>,
    pub(crate) name: TextInput<'a>,
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
                        catalog,
                        kind: AlertKind::Error,
                        message: "There was an error creating your post",
                    })
                } else {
                    None
                },
                source: TextareaInput {
                    catalog,
                    name: "source",
                    label: None,
                    icon: None,
                    placeholder: Some("What's on your mind?"),
                    value: &state.source,
                    error: validation_error.and_then(|e| e.source.as_ref()),
                },
                visibility: SelectInput {
                    catalog,
                    name: "visibility",
                    label: "Post Visibility",
                    selected: state.visibility.to_string(),
                    options: vec![
                        SelectOption {
                            value: "PUB",
                            display: "Visible to everyone",
                        },
                        SelectOption {
                            value: "FL",
                            display: "Visible to followers",
                        },
                        SelectOption {
                            value: "MUT",
                            display: "Visible to mutuals",
                        },
                        SelectOption {
                            value: "LIST",
                            display: "Only visible to mentioned users",
                        },
                    ],
                    error: validation_error.and_then(|e| e.visibility.as_ref()),
                },
                name: TextInput {
                    catalog,
                    name: "name",
                    label: "Content Warning",
                    icon: None,
                    placeholder: Some("mh, nsfw, etc."),
                    value: state.name.as_ref().map(|s| (*s).as_ref()).unwrap_or(""),
                    error: validation_error.and_then(|e| e.source.as_ref()),
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
    fn render(&self, write: &mut std::io::Write) -> std::io::Result<()> {
        crate::templates::home(write, self)
    }
}
