pub mod templates {
use std::io::{self, Write};
use std::fmt::Display;

<<<<<<< HEAD
mod template_first_login_html;
pub use self::template_first_login_html::first_login_html;
=======
mod template_shortcuts;
pub use self::template_shortcuts::shortcuts;

mod template_head;
pub use self::template_head::head;

pub mod home;
>>>>>>> banjo/documentation-updates

#[deprecated(since="0.7.4", note="please use `first_login_html` instead")]
pub use self::first_login_html as first_login;

<<<<<<< HEAD
pub mod error;
=======
mod template_sign_in;
pub use self::template_sign_in::sign_in;

pub mod ui;
>>>>>>> banjo/documentation-updates

pub mod elements;

mod template_sign_up_html;
pub use self::template_sign_up_html::sign_up_html;

#[deprecated(since="0.7.4", note="please use `sign_up_html` instead")]
pub use self::sign_up_html as sign_up;

<<<<<<< HEAD
pub mod asides;
=======
mod template_home;
pub use self::template_home::home;
>>>>>>> banjo/documentation-updates

mod template_sign_up;
pub use self::template_sign_up::sign_up;

mod template_base_html;
pub use self::template_base_html::base_html;

#[deprecated(since="0.7.4", note="please use `base_html` instead")]
pub use self::base_html as base;

mod template_footer_html;
pub use self::template_footer_html::footer_html;

#[deprecated(since="0.7.4", note="please use `footer_html` instead")]
pub use self::footer_html as footer;

mod template_html_head_html;
pub use self::template_html_head_html::html_head_html;

#[deprecated(since="0.7.4", note="please use `html_head_html` instead")]
pub use self::html_head_html as html_head;

pub mod containers;

mod template_sign_in_html;
pub use self::template_sign_in_html::sign_in_html;

<<<<<<< HEAD
#[deprecated(since="0.7.4", note="please use `sign_in_html` instead")]
pub use self::sign_in_html as sign_in;

pub mod posts;

=======
>>>>>>> banjo/documentation-updates
/// This trait should be implemented for any value that can be the
/// result of an expression in a template.
///
/// This trait decides how to format the given object as html.
/// There exists a default implementation for any `T: Display` that
/// formats the value using Display and then html-encodes the result.
pub trait ToHtml {
    /// Write self to `out`, which is in html representation.
    fn to_html(&self, out: &mut dyn Write) -> io::Result<()>;
}

/// Wrapper object for data that should be outputted as raw html
/// (objects that may contain markup).
#[allow(dead_code)]
pub struct Html<T>(pub T);

impl<T: Display> ToHtml for Html<T> {
    #[inline]
    fn to_html(&self, out: &mut dyn Write) -> io::Result<()> {
        write!(out, "{}", self.0)
    }
}

impl<T: Display> ToHtml for T {
    #[inline]
    fn to_html(&self, out: &mut dyn Write) -> io::Result<()> {
        write!(ToHtmlEscapingWriter(out), "{}", self)
    }
}

struct ToHtmlEscapingWriter<'a>(&'a mut dyn Write);

impl<'a> Write for ToHtmlEscapingWriter<'a> {
    #[inline]
    // This takes advantage of the fact that `write` doesn't have to write everything,
    // and the call will be retried with the rest of the data
    // (it is a part of `write_all`'s loop or similar.)
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        // quickly skip over data that doesn't need escaping
        let n = data
            .iter()
            .take_while(|&&c| {
                c != b'"' && c != b'&' && c != b'\'' && c != b'<' && c != b'>'
            })
            .count();
        if n > 0 {
            self.0.write(&data[0..n])
        } else {
            Self::write_one_byte_escaped(&mut self.0, data)
        }
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

impl<'a> ToHtmlEscapingWriter<'a> {
    #[inline(never)]
    fn write_one_byte_escaped(
        out: &mut impl Write,
        data: &[u8],
    ) -> io::Result<usize> {
        let next = data.get(0);
        out.write_all(match next {
            Some(b'"') => b"&quot;",
            Some(b'&') => b"&amp;",
            Some(b'<') => b"&lt;",
            Some(b'>') => b"&gt;",
            None => return Ok(0),
            // we know this function is called only for chars that need escaping,
            // so we don't have to handle the "other" case (this one is for `'`)
            _ => b"&#39;",
        })?;
        Ok(1)
    }
}

}
