use gettext::Catalog;

pub struct Input<'a> {
    pub(crate) catalog: &'a Catalog,
    pub(crate) kind: &'a str,
    pub(crate) name: &'a str,
    pub(crate) label: Option<&'a str>,
    pub(crate) icon: Option<&'a str>,
    pub(crate) placeholder: Option<&'a str>,
    pub(crate) value: &'a str,
    pub(crate) error: Option<&'a String>,
}

impl<'a> From<PasswordInput<'a>> for Input<'a> {
    fn from(p: PasswordInput<'a>) -> Self {
        let PasswordInput {
            catalog,
            name,
            label,
            placeholder,
            error,
        } = p;

        Input {
            catalog,
            kind: "password",
            name,
            label: Some(label),
            placeholder,
            icon: Some("lock"),
            value: "",
            error,
        }
    }
}

impl<'a> From<EmailInput<'a>> for Input<'a> {
    fn from(e: EmailInput<'a>) -> Self {
        let EmailInput {
            catalog,
            name,
            label,
            placeholder,
            value,
            error,
        } = e;

        Input {
            catalog,
            kind: "email",
            name,
            label: Some(label),
            placeholder,
            icon: Some("envelope"),
            value,
            error,
        }
    }
}

impl<'a> From<TextInput<'a>> for Input<'a> {
    fn from(e: TextInput<'a>) -> Self {
        let TextInput {
            catalog,
            name,
            label,
            placeholder,
            icon,
            value,
            error,
        } = e;

        Input {
            catalog,
            kind: "text",
            name,
            label: Some(label),
            placeholder,
            icon,
            value,
            error,
        }
    }
}

pub struct PasswordInput<'a> {
    pub(crate) catalog: &'a Catalog,
    pub(crate) name: &'a str,
    pub(crate) label: &'a str,
    pub(crate) placeholder: Option<&'a str>,
    pub(crate) error: Option<&'a String>,
}

pub struct EmailInput<'a> {
    pub(crate) catalog: &'a Catalog,
    pub(crate) name: &'a str,
    pub(crate) label: &'a str,
    pub(crate) placeholder: Option<&'a str>,
    pub(crate) value: &'a str,
    pub(crate) error: Option<&'a String>,
}

pub struct TextInput<'a> {
    pub(crate) catalog: &'a Catalog,
    pub(crate) name: &'a str,
    pub(crate) label: &'a str,
    pub(crate) placeholder: Option<&'a str>,
    pub(crate) icon: Option<&'a str>,
    pub(crate) value: &'a str,
    pub(crate) error: Option<&'a String>,
}
