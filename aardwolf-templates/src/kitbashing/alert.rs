use gettext::Catalog;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AlertKind {
    Error,
    #[allow(dead_code)]
    Warning,
    #[allow(dead_code)]
    Info,
}

impl std::fmt::Display for AlertKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match *self {
            AlertKind::Error => "error",
            AlertKind::Warning => "warning",
            AlertKind::Info => "info",
        };

        write!(f, "{}", s)
    }
}

pub struct Alert<'a> {
    pub(crate) catalog: &'a Catalog,
    pub(crate) kind: AlertKind,
    pub(crate) message: &'a str,
}
