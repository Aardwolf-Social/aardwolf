use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use crate::SelectInput;

pub fn input_select<W>(mut out: &mut W, input: &SelectInput) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
out.write_all(b"<div class=\"aardwolf-input-wrapper\">\n    <label for=\"")?;
input.name.to_html(&mut out)?;
out.write_all(b"\">\n        ")?;
input.label.to_html(&mut out)?;
out.write_all(b"\n    </label>\n    <div class=\"aardwolf-input aardwolf-select-input\">\n        <select name=\"")?;
input.name.to_html(&mut out)?;
out.write_all(b"\">\n            ")?;
for option in input.options.iter() {
out.write_all(b"\n                ")?;
if option.value == input.selected {
out.write_all(b"\n                    <option value=\"")?;
option.value.to_html(&mut out)?;
out.write_all(b"\" selected>\n                        ")?;
option.display.to_html(&mut out)?;
out.write_all(b"\n                    </option>\n                ")?;
} else {
out.write_all(b"\n                    <option value=\"")?;
option.value.to_html(&mut out)?;
out.write_all(b"\">\n                        ")?;
option.display.to_html(&mut out)?;
out.write_all(b"\n                    </option>\n                ")?;
}
out.write_all(b"\n            ")?;
}
out.write_all(b"</select>\n    </div>\n    <div class=\"aardwolf-input-meta\">\n        ")?;
if let Some(ref error) = input.error {
out.write_all(b"\n            <span class=\"aardwolf-input-error\">")?;
error.to_html(&mut out)?;
out.write_all(b"</span>\n        ")?;
}
out.write_all(b"\n    </div>\n</div>\n")?;
Ok(())
}
