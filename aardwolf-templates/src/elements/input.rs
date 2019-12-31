pub struct Input<'a> {
    pub(crate) kind: &'a str,
    pub(crate) name: &'a str,
    pub(crate) label: Option<String>,
    pub(crate) icon: Option<&'a str>,
    pub(crate) placeholder: Option<String>,
    pub(crate) value: &'a str,
    pub(crate) error: Option<String>,
}

impl<'a> From<&'a InputPassword<'a>> for Input<'a> {
    fn from(p: &'a InputPassword<'a>) -> Self {
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

impl<'a> From<&'a InputEmail<'a>> for Input<'a> {
    fn from(e: &'a InputEmail<'a>) -> Self {
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

impl<'a> From<&'a InputText<'a>> for Input<'a> {
    fn from(t: &'a InputText<'a>) -> Self {
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

pub struct InputPassword<'a> {
    pub(crate) name: &'a str,
    pub(crate) label: String,
    pub(crate) placeholder: Option<String>,
    pub(crate) error: Option<String>,
}

pub struct InputEmail<'a> {
    pub(crate) name: &'a str,
    pub(crate) label: String,
    pub(crate) placeholder: Option<String>,
    pub(crate) value: &'a str,
    pub(crate) error: Option<String>,
}

pub struct InputText<'a> {
    pub(crate) name: &'a str,
    pub(crate) label: String,
    pub(crate) placeholder: Option<String>,
    pub(crate) icon: Option<&'a str>,
    pub(crate) value: &'a str,
    pub(crate) error: Option<String>,
}

pub struct InputCheckbox<'a> {
    pub(crate) name: &'a str,
    pub(crate) label: String,
    pub(crate) icon: Option<&'a str>,
    pub(crate) checked: bool,
    pub(crate) error: Option<String>,
}
