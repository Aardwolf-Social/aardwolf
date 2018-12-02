pub mod templates {
use std::io::{self, Write};
use std::fmt::Display;

pub mod email;

mod template_sign_up;
pub use self::template_sign_up::sign_up;

pub mod asides;

pub mod home;

mod template_base;
pub use self::template_base::base;

mod template_html_head;
pub use self::template_html_head::html_head;

pub mod widgets;

mod template_sign_in;
pub use self::template_sign_in::sign_in;

pub mod posts;

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
