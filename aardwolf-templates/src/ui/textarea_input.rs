use gettext::Catalog;

pub struct TextareaInput<'a> {
    pub(crate) catalog: &'a Catalog,
    pub(crate) name: &'a str,
    pub(crate) label: Option<&'a str>,
    pub(crate) icon: Option<&'a str>,
    pub(crate) placeholder: Option<&'a str>,
    pub(crate) value: &'a str,
    pub(crate) error: Option<&'a String>,
}
