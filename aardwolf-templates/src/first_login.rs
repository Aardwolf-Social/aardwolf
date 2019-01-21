use aardwolf_models::sql_types::{FollowPolicy, PostVisibility};
use aardwolf_types::forms::personas::{
    ValidateDisplayNameFail, ValidatePersonaCreationFail, ValidateShortnameFail,
};
use gettext::Catalog;
use gettext_macros::i18n;

use crate::{Alert, AlertKind, CheckboxInput, Renderable, SelectInput, SelectOption, TextInput};

pub struct FirstLogin<'a> {
    pub(crate) catalog: &'a Catalog,
    pub(crate) csrf: &'a str,
    pub(crate) alert: Option<Alert>,
    pub(crate) display_name: TextInput<'a>,
    pub(crate) shortname: TextInput<'a>,
    pub(crate) follow_policy: SelectInput<'a>,
    pub(crate) default_visibility: SelectInput<'a>,
    pub(crate) is_searchable: CheckboxInput<'a>,
}

impl<'a> FirstLogin<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        catalog: &'a Catalog,
        csrf: &'a str,
        display_name: &'a str,
        shortname: &'a str,
        follow_policy: FollowPolicy,
        default_visibility: PostVisibility,
        is_searchable: bool,
        validation_error: Option<&'a ValidatePersonaCreationFail>,
        server_error: bool,
    ) -> Self {
        FirstLogin {
            catalog,
            csrf,
            alert: if server_error {
                Some(Alert {
                    kind: AlertKind::Error,
                    message: i18n!(catalog, "There was an error creating your persona"),
                })
            } else {
                None
            },
            display_name: TextInput {
                name: "display_name",
                label: i18n!(catalog, "Display Name"),
                icon: None,
                placeholder: Some(i18n!(catalog, "Display name")),
                value: display_name,
                error: validation_error.and_then(|e| {
                    e.display_name.as_ref().map(|e| match *e {
                        ValidateDisplayNameFail::Empty => {
                            i18n!(catalog, "Display name must not be empty")
                        }
                    })
                }),
            },
            shortname: TextInput {
                name: "shortname",
                label: i18n!(catalog, "Username"),
                icon: None,
                placeholder: Some(i18n!(catalog, "Username")),
                value: shortname,
                error: validation_error.and_then(|e| {
                    e.shortname.as_ref().map(|e| match *e {
                        ValidateShortnameFail::Empty => {
                            i18n!(catalog, "Username must not be empty")
                        }
                        ValidateShortnameFail::SpecialCharacters => {
                            i18n!(catalog, "Username must not contain special characters")
                        }
                        ValidateShortnameFail::TooLong => i18n!(catalog, "Username is too long"),
                    })
                }),
            },
            follow_policy: SelectInput {
                name: "follow_policy",
                label: i18n!(catalog, "Follow Policy"),
                selected: follow_policy.to_string(),
                options: vec![
                    SelectOption {
                        value: "ACCEPT",
                        display: i18n!(catalog, "Automatically accept new followers"),
                    },
                    SelectOption {
                        value: "REJECT",
                        display: i18n!(catalog, "Automatically reject new followers"),
                    },
                    SelectOption {
                        value: "MANUAL",
                        display: i18n!(catalog, "Manually review new followers"),
                    },
                ],
                error: validation_error.and_then(|e| e.follow_policy.as_ref().map(|e| match *e {})),
            },
            default_visibility: SelectInput {
                name: "default_visibility",
                label: i18n!(catalog, "Post Visibility"),
                selected: default_visibility.to_string(),
                options: vec![
                    SelectOption {
                        value: "PUB",
                        display: i18n!(catalog, "Visible to everyone"),
                    },
                    SelectOption {
                        value: "FL",
                        display: i18n!(catalog, "Visible to followers"),
                    },
                    SelectOption {
                        value: "MUT",
                        display: i18n!(catalog, "Visible to mutuals"),
                    },
                    SelectOption {
                        value: "LIST",
                        display: i18n!(catalog, "Only visible to mentioned users"),
                    },
                ],
                error: validation_error
                    .and_then(|e| e.default_visibility.as_ref().map(|e| match *e {})),
            },
            is_searchable: CheckboxInput {
                name: "is_searchable",
                label: i18n!(catalog, "Allow people to search for this profile"),
                icon: None,
                checked: is_searchable,
                error: validation_error.and_then(|e| e.is_searchable.as_ref().map(|e| match *e {})),
            },
        }
    }
}

impl<'a> Renderable for FirstLogin<'a> {
    fn render(&self, write: &mut std::io::Write) -> std::io::Result<()> {
        crate::templates::first_login(write, self)
    }
}
