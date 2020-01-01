use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use crate::{InputTextarea, templates::elements::icon};

pub fn input_textarea_html<W>(mut out: &mut W, input: &InputTextarea) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
out.write_all(b"<div class=\"aardwolf-input-wrapper\">\n    <label for=\"")?;
input.name.to_html(&mut out)?;
out.write_all(b"\">\n        ")?;
if let Some(i) = input.icon {
out.write_all(b"\n            ")?;
icon(&mut out, i)?;
out.write_all(b"\n        ")?;
}
out.write_all(b"\n        ")?;
if let Some(ref label) = input.label {
out.write_all(b"\n            ")?;
label.to_html(&mut out)?;
out.write_all(b"\n        ")?;
}
out.write_all(b"\n    </label>\n    <div class=\"aardwolf-input aardwolf-textarea-input\">\n        ")?;
if let Some(ref placeholder) = input.placeholder {
out.write_all(b"\n            <textarea name=\"")?;
input.name.to_html(&mut out)?;
out.write_all(b"\" placeholder=\"")?;
placeholder.to_html(&mut out)?;
out.write_all(b"\" value=\"")?;
input.value.to_html(&mut out)?;
out.write_all(b"\"></textarea>\n        ")?;
} else {
out.write_all(b"\n            <textarea name=\"")?;
input.name.to_html(&mut out)?;
out.write_all(b"\" value=\"")?;
input.value.to_html(&mut out)?;
out.write_all(b"\"></textarea>\n        ")?;
}
out.write_all(b"\n    </div>\n    <div class=\"aardwolf-input-meta\">\n        ")?;
if let Some(ref error) = input.error {
out.write_all(b"\n            <span class=\"aardwolf-input-error\">")?;
error.to_html(&mut out)?;
out.write_all(b"</span>\n        ")?;
}
out.write_all(b"\n    </div>\n</div>\n")?;
Ok(())
}
