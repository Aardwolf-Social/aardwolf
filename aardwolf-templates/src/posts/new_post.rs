use gettext::Catalog;

use crate::elements::{Alert, InputSelect, InputText, InputTextarea};

pub struct NewPost<'a> {
    pub(crate) csrf: &'a str,
    pub(crate) alert: Option<Alert>,
    pub(crate) catalog: &'a Catalog,
    pub(crate) username: &'a str,
    pub(crate) source: InputTextarea<'a>,
    pub(crate) visibility: InputSelect<'a>,
    pub(crate) name: InputText<'a>,
}
