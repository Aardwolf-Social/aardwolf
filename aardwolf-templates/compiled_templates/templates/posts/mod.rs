#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
mod template_reply_post_html;
pub use self::template_reply_post_html::reply_post_html;

#[deprecated(since="0.7.4", note="please use `reply_post_html` instead")]
pub use self::reply_post_html as reply_post;

mod template_new_post_html;
pub use self::template_new_post_html::new_post_html;

#[deprecated(since="0.7.4", note="please use `new_post_html` instead")]
pub use self::new_post_html as new_post;

