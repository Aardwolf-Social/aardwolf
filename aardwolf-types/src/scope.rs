use std::{fmt, str::FromStr};

use serde::{self, Deserialize, Deserializer, Serialize, Serializer};

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

impl Scope {
    pub fn read(&self) -> bool {
        match *self {
            Scope::Read | Scope::ReadWrite | Scope::ReadFollow | Scope::ReadWriteFollow => true,
            _ => false,
        }
    }

    pub fn write(&self) -> bool {
        match *self {
            Scope::Write | Scope::ReadWrite | Scope::WriteFollow | Scope::ReadWriteFollow => true,
            _ => false,
        }
    }

    pub fn follow(&self) -> bool {
        match *self {
            Scope::Follow | Scope::ReadFollow | Scope::WriteFollow | Scope::ReadWriteFollow => true,
            _ => false,
        }
    }
}

impl<'de> Deserialize<'de> for Scope {
    fn deserialize<D>(deserializer: D) -> Result<Scope, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<Scope>().map_err(serde::de::Error::custom)
    }
}

impl Serialize for Scope {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let read = "read";
        let write = "write";
        let follow = "follow";

        let scope = match *self {
            Scope::Read => vec![read],
            Scope::Write => vec![write],
            Scope::Follow => vec![follow],
            Scope::ReadWrite => vec![read, write],
            Scope::ReadFollow => vec![read, follow],
            Scope::WriteFollow => vec![write, follow],
            Scope::ReadWriteFollow => vec![read, write, follow],
        };

        write!(f, "{}", scope.join(" "))
    }
}

#[derive(Debug, Fail)]
#[fail(display = "Error decoding 'scope'")]
pub struct ScopeDecodeError;

impl FromStr for Scope {
    type Err = ScopeDecodeError;

    fn from_str(scope: &str) -> Result<Self, Self::Err> {
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
