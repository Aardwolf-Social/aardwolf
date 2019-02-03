use std::{error::Error as StdError, fmt, io::Write, str::FromStr};

use diesel::{backend::Backend, deserialize, serialize, sql_types::Text};
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

#[derive(AsExpression, Clone, Copy, Debug, Eq, FromSqlRow, Hash, PartialEq)]
#[sql_type = "Text"]
pub enum PostVisibility {
    Public,
    FollowersOnly,
    FriendsOnly,
    ListedPeopleOnly,
}

impl PostVisibility {
    pub fn as_str(&self) -> &str {
        match *self {
            PostVisibility::Public => "PUB",
            PostVisibility::FollowersOnly => "FL",
            PostVisibility::FriendsOnly => "MUT",
            PostVisibility::ListedPeopleOnly => "LIST",
        }
    }
}

impl fmt::Display for PostVisibility {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Serialize for PostVisibility {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl FromStr for PostVisibility {
    type Err = VisibilityParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PUB" => Ok(PostVisibility::Public),
            "FL" => Ok(PostVisibility::FollowersOnly),
            "MUT" => Ok(PostVisibility::FriendsOnly),
            "LIST" => Ok(PostVisibility::ListedPeopleOnly),
            _ => Err(VisibilityParseError),
        }
    }
}

impl<'de> Deserialize<'de> for PostVisibility {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<PostVisibility>()
            .map_err(serde::de::Error::custom)
    }
}

impl<DB> serialize::ToSql<Text, DB> for PostVisibility
where
    DB: Backend,
{
    fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, DB>) -> serialize::Result {
        serialize::ToSql::<Text, DB>::to_sql(&format!("{}", self), out)
    }
}

impl<DB> deserialize::FromSql<Text, DB> for PostVisibility
where
    DB: Backend<RawValue = [u8]>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        deserialize::FromSql::<Text, DB>::from_sql(bytes).and_then(|string: String| {
            string
                .parse::<PostVisibility>()
                .map_err(|e| Box::new(e) as Box<StdError + Send + Sync>)
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VisibilityParseError;

impl fmt::Display for VisibilityParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse PostVisibility")
    }
}

impl StdError for VisibilityParseError {
    fn description(&self) -> &str {
        "Failed to parse PostVisibility"
    }

    fn cause(&self) -> Option<&StdError> {
        None
    }
}

#[cfg(feature = "rocket")]
mod rocket {
    use std::str::Utf8Error;

    use rocket::{http::RawStr, request::FromFormValue};

    use super::{PostVisibility, VisibilityParseError};

    impl<'v> FromFormValue<'v> for PostVisibility {
        type Error = VisibilityParseError;

        fn from_form_value(form_value: &'v RawStr) -> Result<Self, Self::Error> {
            Ok(form_value.url_decode()?.parse()?)
        }
    }

    impl From<Utf8Error> for VisibilityParseError {
        fn from(_: Utf8Error) -> Self {
            VisibilityParseError
        }
    }
}
