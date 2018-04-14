use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq)]
pub enum Scope {
    Read,
    Write,
    Follow,
    ReadWrite,
    ReadFollow,
    WriteFollow,
    ReadWriteFollow,
}

#[derive(Debug, Fail)]
#[fail(display = "Error decoding 'scope'")]
pub struct ScopeDecodeError;

impl TryFrom<String> for Scope {
    type Error = ScopeDecodeError;

    fn try_from(scope: String) -> Result<Self, Self::Error> {
        let mut read: bool = false;
        let mut write: bool = false;
        let mut follow: bool = false;

        // because "read write follow" is length 17 and that is the longest string that should be
        // coming in here
        if scope.len() > 17 {
            return Err(ScopeDecodeError);
        }

        let parts = scope.splitn(3, ' ');

        for part in parts {
            if part == "read" {
                read = true;
            }
            if part == "write" {
                write = true;
            }
            if part == "follow" {
                follow = true;
            }
        }

        Ok(match (read, write, follow) {
            (true, false, false) => Scope::Read,
            (false, true, false) => Scope::Write,
            (false, false, true) => Scope::Follow,
            (true, true, false) => Scope::ReadWrite,
            (true, false, true) => Scope::ReadFollow,
            (false, true, true) => Scope::WriteFollow,
            (true, true, true) => Scope::ReadWriteFollow,
            _ => {
                return Err(ScopeDecodeError);
            }
        })
    }
}
