#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
mod template_home;
pub use self::template_home::home;

mod template_footer;
pub use self::template_footer::footer;

mod template_top_nav;
pub use self::template_top_nav::top_nav;

mod template_feed;
pub use self::template_feed::feed;

