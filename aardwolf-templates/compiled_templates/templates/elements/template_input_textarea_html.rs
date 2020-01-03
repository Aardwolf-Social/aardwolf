use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use crate::{InputTextarea, templates::elements::icon};

pub fn input_textarea_html<W>(mut out: &mut W, input: &InputTextarea) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
out.write_all(b"<div class=\"aardwolf-input-wrapper\">\r\n    <label for=\"")?;
input.name.to_html(&mut out)?;
out.write_all(b"\">\r\n        ")?;
if let Some(i) = input.icon {
out.write_all(b"\r\n            ")?;
icon(&mut out, i)?;
out.write_all(b"\r\n        ")?;
}
out.write_all(b"\r\n        ")?;
if let Some(ref label) = input.label {
out.write_all(b"\r\n            ")?;
label.to_html(&mut out)?;
out.write_all(b"\r\n        ")?;
}
out.write_all(b"\r\n    </label>\r\n    <div class=\"aardwolf-input aardwolf-textarea-input\">\r\n        ")?;
if let Some(ref placeholder) = input.placeholder {
out.write_all(b"\r\n            <textarea name=\"")?;
input.name.to_html(&mut out)?;
out.write_all(b"\" placeholder=\"")?;
placeholder.to_html(&mut out)?;
out.write_all(b"\" value=\"")?;
input.value.to_html(&mut out)?;
out.write_all(b"\"></textarea>\r\n        ")?;
} else {
out.write_all(b"\r\n            <textarea name=\"")?;
input.name.to_html(&mut out)?;
out.write_all(b"\" value=\"")?;
input.value.to_html(&mut out)?;
out.write_all(b"\"></textarea>\r\n        ")?;
}
out.write_all(b"\r\n    </div>\r\n    <div class=\"aardwolf-input-meta\">\r\n        ")?;
if let Some(ref error) = input.error {
out.write_all(b"\r\n            <span class=\"aardwolf-input-error\">")?;
error.to_html(&mut out)?;
out.write_all(b"</span>\r\n        ")?;
}
out.write_all(b"\r\n    </div>\r\n</div>\r\n")?;
Ok(())
}
