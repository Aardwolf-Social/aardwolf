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
pub enum ReactionType {
    Like,
    Dislike,
    Seen,
}

impl fmt::Display for ReactionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReactionType::Like => write!(f, "LIKE"),
            ReactionType::Dislike => write!(f, "DISLIKE"),
            ReactionType::Seen => write!(f, "SEEN"),
        }
    }
}

impl FromStr for ReactionType {
    type Err = ReactionTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LIKE" => Ok(ReactionType::Like),
            "DISLIKE" => Ok(ReactionType::Dislike),
            "SEEN" => Ok(ReactionType::Seen),
            _ => Err(ReactionTypeParseError),
        }
    }
}

impl<DB> serialize::ToSql<Text, DB> for ReactionType
where
    DB: Backend,
{
    fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, DB>) -> serialize::Result {
        serialize::ToSql::<Text, DB>::to_sql(&format!("{}", self), out)
    }
}

impl<DB> deserialize::FromSql<Text, DB> for ReactionType
where
    DB: Backend<RawValue = [u8]>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        deserialize::FromSql::<Text, DB>::from_sql(bytes).and_then(|string: String| {
            string
                .parse::<ReactionType>()
                .map_err(|e| Box::new(e) as Box<StdError + Send + Sync>)
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReactionTypeParseError;

impl fmt::Display for ReactionTypeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing ReactionType")
    }
}

impl StdError for ReactionTypeParseError {
    fn description(&self) -> &str {
        "Error parsing ReactionType"
    }

    fn cause(&self) -> Option<&StdError> {
        None
    }
}
