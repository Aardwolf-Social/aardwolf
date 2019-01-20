pub struct Input<'a> {
    pub(crate) kind: &'a str,
    pub(crate) name: &'a str,
    pub(crate) label: Option<String>,
    pub(crate) icon: Option<&'a str>,
    pub(crate) placeholder: Option<String>,
    pub(crate) value: &'a str,
    pub(crate) error: Option<String>,
}

impl<'a> From<&'a PasswordInput<'a>> for Input<'a> {
    fn from(p: &'a PasswordInput<'a>) -> Self {
        Input {
            kind: "password",
            name: p.name,
            label: Some(p.label.clone()),
            placeholder: p.placeholder.clone(),
            icon: Some("lock"),
            value: "",
            error: p.error.clone(),
        }
    }
}

impl<'a> From<&'a EmailInput<'a>> for Input<'a> {
    fn from(e: &'a EmailInput<'a>) -> Self {
        Input {
            kind: "email",
            name: e.name,
            label: Some(e.label.clone()),
            placeholder: e.placeholder.clone(),
            icon: Some("envelope"),
            value: e.value,
            error: e.error.clone(),
        }
    }
}

impl<'a> From<&'a TextInput<'a>> for Input<'a> {
    fn from(t: &'a TextInput<'a>) -> Self {
        Input {
            kind: "text",
            name: t.name,
            label: Some(t.label.clone()),
            placeholder: t.placeholder.clone(),
            icon: t.icon,
            value: t.value,
            error: t.error.clone(),
        }
    }
}

pub struct PasswordInput<'a> {
    pub(crate) name: &'a str,
    pub(crate) label: String,
    pub(crate) placeholder: Option<String>,
    pub(crate) error: Option<String>,
}

pub struct EmailInput<'a> {
    pub(crate) name: &'a str,
    pub(crate) label: String,
    pub(crate) placeholder: Option<String>,
    pub(crate) value: &'a str,
    pub(crate) error: Option<String>,
}

pub struct TextInput<'a> {
    pub(crate) name: &'a str,
    pub(crate) label: String,
    pub(crate) placeholder: Option<String>,
    pub(crate) icon: Option<&'a str>,
    pub(crate) value: &'a str,
    pub(crate) error: Option<String>,
}

pub struct CheckboxInput<'a> {
    pub(crate) name: &'a str,
    pub(crate) label: String,
    pub(crate) icon: Option<&'a str>,
    pub(crate) checked: bool,
    pub(crate) error: Option<String>,
}
