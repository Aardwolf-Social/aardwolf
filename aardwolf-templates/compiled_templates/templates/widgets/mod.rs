#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
<<<<<<< HEAD
=======
mod template_textarea_input;
pub use self::template_textarea_input::textarea_input;

mod template_checkbox_input;
pub use self::template_checkbox_input::checkbox_input;

mod template_password_input;
pub use self::template_password_input::password_input;

>>>>>>> 4171ff0286ed26c81a011663cd6f23b09128cf9f
mod template_text_input;
pub use self::template_text_input::text_input;

mod template_icon;
pub use self::template_icon::icon;

mod template_alert;
pub use self::template_alert::alert;

mod template_select_input;
pub use self::template_select_input::select_input;

mod template_input;
pub use self::template_input::input;

mod template_password_input;
pub use self::template_password_input::password_input;

