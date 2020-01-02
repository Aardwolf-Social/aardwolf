use gettext::Catalog;

pub struct Settings<'a> {
    pub(crate) catalog: &'a Catalog,
    pub(crate) http_error: &'a str,
}
