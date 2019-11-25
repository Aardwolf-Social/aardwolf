#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
mod template_aside_shortcuts;
pub use self::template_aside_shortcuts::aside_shortcuts;

mod template_aside_settings;
pub use self::template_aside_settings::aside_settings;

