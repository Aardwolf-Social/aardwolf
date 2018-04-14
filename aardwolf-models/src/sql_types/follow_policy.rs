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
pub enum FollowPolicy {
    AutoAccept,
    AutoReject,
    ManualReview,
}

impl fmt::Display for FollowPolicy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FollowPolicy::AutoAccept => write!(f, "ACCEPT"),
            FollowPolicy::AutoReject => write!(f, "REJECT"),
            FollowPolicy::ManualReview => write!(f, "MANUAL"),
        }
    }
}

impl FromStr for FollowPolicy {
    type Err = FollowPolicyParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ACCEPT" => Ok(FollowPolicy::AutoAccept),
            "REJECT" => Ok(FollowPolicy::AutoReject),
            "MANUAL" => Ok(FollowPolicy::ManualReview),
            _ => Err(FollowPolicyParseError),
        }
    }
}

impl<DB> serialize::ToSql<Text, DB> for FollowPolicy
where
    DB: Backend,
{
    fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, DB>) -> serialize::Result {
        serialize::ToSql::<Text, DB>::to_sql(&format!("{}", self), out)
    }
}

impl<DB> deserialize::FromSql<Text, DB> for FollowPolicy
where
    DB: Backend<RawValue = [u8]>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        deserialize::FromSql::<Text, DB>::from_sql(bytes).and_then(|string: String| {
            string
                .parse::<FollowPolicy>()
                .map_err(|e| Box::new(e) as Box<StdError + Send + Sync>)
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FollowPolicyParseError;

impl fmt::Display for FollowPolicyParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse FollowPolicy")
    }
}

impl StdError for FollowPolicyParseError {
    fn description(&self) -> &str {
        "Failed to parse FollowPolicy"
    }

    fn cause(&self) -> Option<&StdError> {
        None
    }
}
