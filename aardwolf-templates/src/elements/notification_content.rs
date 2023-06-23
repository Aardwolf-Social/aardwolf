use gettext::Catalog;

pub struct NotificationContent<'a> {
    pub(crate) catalog: &'a Catalog,
}
