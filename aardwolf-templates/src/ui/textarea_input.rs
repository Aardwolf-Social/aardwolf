pub struct TextareaInput<'a> {
    pub(crate) name: &'a str,
    pub(crate) label: Option<String>,
    pub(crate) icon: Option<&'a str>,
    pub(crate) placeholder: Option<String>,
    pub(crate) value: &'a str,
    pub(crate) error: Option<String>,
}
