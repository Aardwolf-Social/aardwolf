pub mod templates {
use std::io::{self, Write};
use std::fmt::Display;

mod first_login;
pub use self::first_login::first_login;

pub mod error;

pub mod elements;

mod sign_up;
pub use self::sign_up::sign_up;

pub mod asides;

pub mod home;

mod home_base;
pub use self::home_base::home_base;

mod head;
pub use self::head::head;

mod footer;
pub use self::footer::footer;

pub mod containers;

mod sign_in;
pub use self::sign_in::sign_in;

pub mod posts;

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

