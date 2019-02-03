pub struct SelectInput<'a> {
    pub(crate) name: &'a str,
    pub(crate) label: String,
    pub(crate) selected: String,
    pub(crate) options: Vec<SelectOption<'a>>,
    pub(crate) error: Option<String>,
}

pub struct SelectOption<'a> {
    pub(crate) value: &'a str,
    pub(crate) display: String,
}
