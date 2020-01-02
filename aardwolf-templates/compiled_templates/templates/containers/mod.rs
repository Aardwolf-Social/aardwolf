#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
mod template_container_calendar;
pub use self::template_container_calendar::container_calendar;

mod template_container_profile_edit;
pub use self::template_container_profile_edit::container_profile_edit;

mod template_container_preferences;
pub use self::template_container_preferences::container_preferences;

