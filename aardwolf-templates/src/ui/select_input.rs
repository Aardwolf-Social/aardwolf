use gettext::Catalog;

pub struct SelectInput<'a> {
    pub(crate) catalog: &'a Catalog,
    pub(crate) name: &'a str,
    pub(crate) label: &'a str,
    pub(crate) selected: String,
    pub(crate) options: Vec<SelectOption<'a>>,
    pub(crate) error: Option<&'a String>,
}

pub struct SelectOption<'a> {
    pub(crate) value: &'a str,
    pub(crate) display: &'a str,
}
