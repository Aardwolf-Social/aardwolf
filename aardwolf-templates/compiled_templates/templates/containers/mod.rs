#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
mod template_calendar_html;
pub use self::template_calendar_html::calendar_html;

#[deprecated(since="0.7.4", note="please use `calendar_html` instead")]
pub use self::calendar_html as calendar;

mod template_preferences_html;
pub use self::template_preferences_html::preferences_html;

#[deprecated(since="0.7.4", note="please use `preferences_html` instead")]
pub use self::preferences_html as preferences;

mod template_profile_edit_html;
pub use self::template_profile_edit_html::profile_edit_html;

#[deprecated(since="0.7.4", note="please use `profile_edit_html` instead")]
pub use self::profile_edit_html as profile_edit;

