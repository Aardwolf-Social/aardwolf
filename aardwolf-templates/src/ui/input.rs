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

impl<'a> From<&'a PasswordInput<'a>> for Input<'a> {
    fn from(p: &'a PasswordInput<'a>) -> Self {
        Input {
            catalog: p.catalog,
            kind: "password",
            name: p.name,
            label: Some(p.label),
            placeholder: p.placeholder,
            icon: Some("lock"),
            value: "",
            error: p.error,
        }
    }
}

impl<'a> From<&'a EmailInput<'a>> for Input<'a> {
    fn from(e: &'a EmailInput<'a>) -> Self {
        Input {
            catalog: e.catalog,
            kind: "email",
            name: e.name,
            label: Some(e.label),
            placeholder: e.placeholder,
            icon: Some("envelope"),
            value: e.value,
            error: e.error,
        }
    }
}

impl<'a> From<&'a TextInput<'a>> for Input<'a> {
    fn from(t: &'a TextInput<'a>) -> Self {
        Input {
            catalog: t.catalog,
            kind: "text",
            name: t.name,
            label: Some(t.label),
            placeholder: t.placeholder,
            icon: t.icon,
            value: t.value,
            error: t.error,
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
