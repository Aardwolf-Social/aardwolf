use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use crate::InputSelect;

pub fn input_select_html<W>(mut out: &mut W, input: &InputSelect) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
out.write_all(b"<div class=\"aardwolf-input-wrapper\">\r\n    <label for=\"")?;
input.name.to_html(&mut out)?;
out.write_all(b"\">\r\n        ")?;
input.label.to_html(&mut out)?;
out.write_all(b"\r\n    </label>\r\n    <div class=\"aardwolf-input aardwolf-select-input\">\r\n        <select name=\"")?;
input.name.to_html(&mut out)?;
out.write_all(b"\">\r\n            ")?;
for option in input.options.iter() {
out.write_all(b"\r\n                ")?;
if option.value == input.selected {
out.write_all(b"\r\n                    <option value=\"")?;
option.value.to_html(&mut out)?;
out.write_all(b"\" selected>\r\n                        ")?;
option.display.to_html(&mut out)?;
out.write_all(b"\r\n                    </option>\r\n                ")?;
} else {
out.write_all(b"\r\n                    <option value=\"")?;
option.value.to_html(&mut out)?;
out.write_all(b"\">\r\n                        ")?;
option.display.to_html(&mut out)?;
out.write_all(b"\r\n                    </option>\r\n                ")?;
}
out.write_all(b"\r\n            ")?;
}
out.write_all(b"</select>\r\n    </div>\r\n    <div class=\"aardwolf-input-meta\">\r\n        ")?;
if let Some(ref error) = input.error {
out.write_all(b"\r\n            <span class=\"aardwolf-input-error\">")?;
error.to_html(&mut out)?;
out.write_all(b"</span>\r\n        ")?;
}
out.write_all(b"\r\n    </div>\r\n</div>\r\n")?;
Ok(())
}
