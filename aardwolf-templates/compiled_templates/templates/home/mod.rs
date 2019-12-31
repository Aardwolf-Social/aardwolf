#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
mod template_home;
pub use self::template_home::home;

mod template_home_nav_top;
pub use self::template_home_nav_top::home_nav_top;

mod template_home_base;
pub use self::template_home_base::home_base;

mod template_feed;
pub use self::template_feed::feed;

