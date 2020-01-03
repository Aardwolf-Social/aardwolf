use gettext::Catalog;

pub struct ReplyPost<'a> {
    pub(crate) catalog: &'a Catalog,
    pub(crate) profile_link: &'a str,
    pub(crate) username: &'a str,
}
