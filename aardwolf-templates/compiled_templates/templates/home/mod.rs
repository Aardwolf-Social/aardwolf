#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
mod template_home_html;
pub use self::template_home_html::home_html;

#[deprecated(since="0.7.4", note="please use `home_html` instead")]
pub use self::home_html as home;

mod template_nav_top_html;
pub use self::template_nav_top_html::nav_top_html;

#[deprecated(since="0.7.4", note="please use `nav_top_html` instead")]
pub use self::nav_top_html as nav_top;

mod template_feed_html;
pub use self::template_feed_html::feed_html;

#[deprecated(since="0.7.4", note="please use `feed_html` instead")]
pub use self::feed_html as feed;

