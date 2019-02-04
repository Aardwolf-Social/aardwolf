#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
mod template_settings;
pub use self::template_settings::settings;

mod template_shortcuts;
pub use self::template_shortcuts::shortcuts;

