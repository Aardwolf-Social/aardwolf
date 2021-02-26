#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
mod template_alert_html;
pub use self::template_alert_html::alert_html;

#[deprecated(since="0.7.4", note="please use `alert_html` instead")]
pub use self::alert_html as alert;

mod template_icon_html;
pub use self::template_icon_html::icon_html;

#[deprecated(since="0.7.4", note="please use `icon_html` instead")]
pub use self::icon_html as icon;

mod template_input_html;
pub use self::template_input_html::input_html;

#[deprecated(since="0.7.4", note="please use `input_html` instead")]
pub use self::input_html as input;

mod template_input_checkbox_html;
pub use self::template_input_checkbox_html::input_checkbox_html;

#[deprecated(since="0.7.4", note="please use `input_checkbox_html` instead")]
pub use self::input_checkbox_html as input_checkbox;

mod template_input_email_html;
pub use self::template_input_email_html::input_email_html;

#[deprecated(since="0.7.4", note="please use `input_email_html` instead")]
pub use self::input_email_html as input_email;

mod template_input_password_html;
pub use self::template_input_password_html::input_password_html;

#[deprecated(since="0.7.4", note="please use `input_password_html` instead")]
pub use self::input_password_html as input_password;

mod template_input_password_confirm_html;
pub use self::template_input_password_confirm_html::input_password_confirm_html;

#[deprecated(since="0.7.4", note="please use `input_password_confirm_html` instead")]
pub use self::input_password_confirm_html as input_password_confirm;

mod template_input_select_html;
pub use self::template_input_select_html::input_select_html;

#[deprecated(since="0.7.4", note="please use `input_select_html` instead")]
pub use self::input_select_html as input_select;

mod template_input_text_html;
pub use self::template_input_text_html::input_text_html;

#[deprecated(since="0.7.4", note="please use `input_text_html` instead")]
pub use self::input_text_html as input_text;

mod template_input_textarea_html;
pub use self::template_input_textarea_html::input_textarea_html;

#[deprecated(since="0.7.4", note="please use `input_textarea_html` instead")]
pub use self::input_textarea_html as input_textarea;

mod template_lang_dropdown_html;
pub use self::template_lang_dropdown_html::lang_dropdown_html;

#[deprecated(since="0.7.4", note="please use `lang_dropdown_html` instead")]
pub use self::lang_dropdown_html as lang_dropdown;

mod template_notification_content_html;
pub use self::template_notification_content_html::notification_content_html;

#[deprecated(since="0.7.4", note="please use `notification_content_html` instead")]
pub use self::notification_content_html as notification_content;

mod template_notification_dropdown_html;
pub use self::template_notification_dropdown_html::notification_dropdown_html;

#[deprecated(since="0.7.4", note="please use `notification_dropdown_html` instead")]
pub use self::notification_dropdown_html as notification_dropdown;

mod template_search_bar_html;
pub use self::template_search_bar_html::search_bar_html;

#[deprecated(since="0.7.4", note="please use `search_bar_html` instead")]
pub use self::search_bar_html as search_bar;

