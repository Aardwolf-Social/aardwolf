use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
<<<<<<< Updated upstream:aardwolf-templates/compiled_templates/templates/widgets/template_alert.rs
<<<<<<< HEAD
use rocket_i18n::i18n;
use crate::{Alert, templates::widgets::alert};
=======
use crate::{Alert, templates::widgets::icon};
>>>>>>> 4171ff0286ed26c81a011663cd6f23b09128cf9f
=======
use crate::{Alert, templates::widgets::icon};
>>>>>>> Stashed changes:aardwolf-templates/compiled_templates/templates/ui/template_alert.rs

pub fn alert(out: &mut Write, alert: &Alert) -> io::Result<()> {
out.write_all(b"<div class=\"aardwolf-alert aardwolf-alert-")?;
alert.kind.to_html(out)?;
out.write_all(b"\">\n    <div class=\"aardwolf-alert-meta\">\n        ")?;
icon(out, "warning")?;
out.write_all(b"\n    </div>\n    <div class=\"aardwolf-alert-message\">\n        ")?;
alert.message.to_html(out)?;
out.write_all(b"\n    </div>\n</div>\n")?;
Ok(())
}
