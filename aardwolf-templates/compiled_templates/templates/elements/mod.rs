#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
mod template_icon;
pub use self::template_icon::icon;

mod template_input_select;
pub use self::template_input_select::input_select;

mod template_input_email;
pub use self::template_input_email::input_email;

mod template_input_text;
pub use self::template_input_text::input_text;

mod template_alert;
pub use self::template_alert::alert;

mod template_input_checkbox;
pub use self::template_input_checkbox::input_checkbox;

mod template_input_textarea;
pub use self::template_input_textarea::input_textarea;

mod template_input;
pub use self::template_input::input;

