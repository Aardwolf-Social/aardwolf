use std::error::Error as StdError;
use std::fmt;
use std::io::Write;
use std::str::FromStr;

use diesel::backend::Backend;
use diesel::deserialize;
use diesel::serialize;
use diesel::sql_types::Text;

#[derive(AsExpression, Clone, Copy, Debug, Eq, FromSqlRow, Hash, PartialEq)]
#[sql_type = "Text"]
pub enum PostVisibility {
    Public,
    FollowersOnly,
    FriendsOnly,
    ListedPeopleOnly,
}

impl fmt::Display for PostVisibility {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PostVisibility::Public => write!(f, "PUB"),
            PostVisibility::FollowersOnly => write!(f, "FL"),
            PostVisibility::FriendsOnly => write!(f, "MUT"),
            PostVisibility::ListedPeopleOnly => write!(f, "LIST"),
        }
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
