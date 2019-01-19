use aardwolf_models::sql_types::{FollowPolicy, PostVisibility};
use aardwolf_types::forms::personas::ValidatePersonaCreationFail;
use gettext::Catalog;

use crate::{Alert, AlertKind, CheckboxInput, Renderable, SelectInput, SelectOption, TextInput};

pub struct FirstLogin<'a> {
    pub(crate) catalog: &'a Catalog,
    pub(crate) csrf: &'a str,
    pub(crate) alert: Option<Alert<'a>>,
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
                    catalog,
                    kind: AlertKind::Error,
                    message: "There was an error creating your persona",
                })
            } else {
                None
            },
            display_name: TextInput {
                catalog,
                name: "display_name",
                label: "Display Name",
                icon: None,
                placeholder: Some("Display name"),
                value: display_name,
                error: validation_error.and_then(|e| e.display_name.as_ref()),
            },
            shortname: TextInput {
                catalog,
                name: "shortname",
                label: "Username",
                icon: None,
                placeholder: Some("Username"),
                value: shortname,
                error: validation_error.and_then(|e| e.shortname.as_ref()),
            },
            follow_policy: SelectInput {
                catalog,
                name: "follow_policy",
                label: "Follow Policy",
                selected: follow_policy.to_string(),
                options: vec![
                    SelectOption {
                        value: "ACCEPT",
                        display: "Automatically accept new followers",
                    },
                    SelectOption {
                        value: "REJECT",
                        display: "Automatically reject new followers",
                    },
                    SelectOption {
                        value: "MANUAL",
                        display: "Manually review new followers",
                    },
                ],
                error: validation_error.and_then(|e| e.follow_policy.as_ref()),
            },
            default_visibility: SelectInput {
                catalog,
                name: "default_visibility",
                label: "Post Visibility",
                selected: default_visibility.to_string(),
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
                error: validation_error.and_then(|e| e.default_visibility.as_ref()),
            },
            is_searchable: CheckboxInput {
                catalog,
                name: "is_searchable",
                label: "Allow people to search for this profile",
                icon: None,
                checked: is_searchable,
                error: validation_error.and_then(|e| e.is_searchable.as_ref()),
            },
        }
    }
}

impl<'a> Renderable for FirstLogin<'a> {
    fn render(&self, write: &mut std::io::Write) -> std::io::Result<()> {
        crate::templates::first_login(write, self)
    }
}
