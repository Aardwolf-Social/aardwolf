#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
mod template_feed;
pub use self::template_feed::feed;

mod template_top_nav;
pub use self::template_top_nav::top_nav;

mod template_home;
pub use self::template_home::home;

