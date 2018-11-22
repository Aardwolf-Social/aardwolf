pub mod templates {
use std::io::{self, Write};
use std::fmt::Display;

mod template_base;
pub use self::template_base::base;

mod template_settings;
pub use self::template_settings::settings;

mod template_sign_up;
pub use self::template_sign_up::sign_up;

mod template_home;
pub use self::template_home::home;

pub mod home;

mod template_new_post;
pub use self::template_new_post::new_post;

mod template_head;
pub use self::template_head::head;

mod template_sign_in;
pub use self::template_sign_in::sign_in;

mod template_shortcuts;
pub use self::template_shortcuts::shortcuts;

mod template_footer;
pub use self::template_footer::footer;

/// This trait should be implemented for any value that can be the
/// result of an expression in a template.
///
/// This trait decides how to format the given object as html.
/// There exists a default implementation for any `T: Display` that
/// formats the value using Display and then html-encodes the result.
pub trait ToHtml {
    /// Write self to `out`, which is in html representation.
    fn to_html(&self, out: &mut Write) -> io::Result<()>;
}

/// Wrapper object for data that should be outputted as raw html
/// (objects that may contain markup).
#[allow(dead_code)]
pub struct Html<T> (pub T);

impl<T: Display> ToHtml for Html<T> {
    fn to_html(&self, out: &mut Write) -> io::Result<()> {
        write!(out, "{}", self.0)
    }
}

impl<T: Display> ToHtml for T {
    fn to_html(&self, out: &mut Write) -> io::Result<()> {
        let mut buf = Vec::new();
        write!(buf, "{}", self)?;
        out.write_all(&buf.into_iter().fold(Vec::new(), |mut v, c| {
            match c {
                b'<' => v.extend_from_slice(b"&lt;"),
                b'>' => v.extend_from_slice(b"&gt;"),
                b'&' => v.extend_from_slice(b"&amp;"),
                c => v.push(c),
            };
            v
        }))
    }
}

}
