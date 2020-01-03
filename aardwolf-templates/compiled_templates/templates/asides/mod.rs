#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
mod template_settings_html;
pub use self::template_settings_html::settings_html;

#[deprecated(since="0.7.4", note="please use `settings_html` instead")]
pub use self::settings_html as settings;

mod template_shortcuts_html;
pub use self::template_shortcuts_html::shortcuts_html;

#[deprecated(since="0.7.4", note="please use `shortcuts_html` instead")]
pub use self::shortcuts_html as shortcuts;

