#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
mod template_new_html;
pub use self::template_new_html::new_html;

#[deprecated(since="0.7.4", note="please use `new_html` instead")]
pub use self::new_html as new;

