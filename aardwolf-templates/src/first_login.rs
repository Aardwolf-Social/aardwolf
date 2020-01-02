use aardwolf_types::forms::personas::{
    PersonaCreationFormState, ValidateDisplayNameFail, ValidatePersonaCreationFail,
    ValidateShortnameFail,
};
use gettext::Catalog;
use gettext_macros::i18n;

use crate::{Alert, AlertKind, InputCheckbox, Renderable, InputSelect, InputText};

pub struct FirstLogin<'a> {
    pub(crate) catalog: &'a Catalog,
    pub(crate) csrf: &'a str,
    pub(crate) alert: Option<Alert>,
    pub(crate) display_name: InputText<'a>,
    pub(crate) shortname: InputText<'a>,
    pub(crate) follow_policy: InputSelect<'a>,
    pub(crate) default_visibility: InputSelect<'a>,
    pub(crate) is_searchable: InputCheckbox<'a>,
}

impl<'a> FirstLogin<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        catalog: &'a Catalog,
        csrf: &'a str,
        state: &'a PersonaCreationFormState,
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
            display_name: InputText {
                name: "display_name",
                label: i18n!(catalog, "Display Name"),
                icon: None,
                placeholder: Some(i18n!(catalog, "Display name")),
                value: &state.display_name,
                error: validation_error.and_then(|e| {
                    e.display_name.as_ref().map(|e| match *e {
                        ValidateDisplayNameFail::Empty => {
                            i18n!(catalog, "Display name must not be empty")
                        }
                    })
                }),
            },
            shortname: InputText {
                name: "shortname",
                label: i18n!(catalog, "Username"),
                icon: None,
                placeholder: Some(i18n!(catalog, "Username")),
                value: &state.shortname,
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
            follow_policy: InputSelect {
                name: "follow_policy",
                label: i18n!(catalog, "Follow Policy"),
                selected: state.follow_policy.to_string(),
                options: InputSelect::follow_policy_options(catalog),
                error: validation_error.and_then(|e| e.follow_policy.as_ref().map(|e| match *e {})),
            },
            default_visibility: InputSelect {
                name: "default_visibility",
                label: i18n!(catalog, "Post Visibility"),
                selected: state.default_visibility.to_string(),
                options: InputSelect::visibility_options(catalog),
                error: validation_error
                    .and_then(|e| e.default_visibility.as_ref().map(|e| match *e {})),
            },
            is_searchable: InputCheckbox {
                name: "is_searchable",
                label: i18n!(catalog, "Allow people to search for this profile"),
                icon: None,
                checked: state.is_searchable,
                error: validation_error.and_then(|e| e.is_searchable.as_ref().map(|e| match *e {})),
            },
        }
    }
}

impl<'a> Renderable for FirstLogin<'a> {
    fn render(&self, write: &mut dyn std::io::Write) -> std::io::Result<()> {
        crate::templates::first_login(write, self)
    }
}
