use std::{error::Error as StdError, fmt, io::Write, str::FromStr};

use diesel::{backend::Backend, deserialize, serialize, sql_types::Text};
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

#[derive(AsExpression, Clone, Copy, Debug, Eq, FromSqlRow, Hash, PartialEq)]
#[sql_type = "Text"]
pub enum FollowPolicy {
    AutoAccept,
    AutoReject,
    ManualReview,
}

impl FollowPolicy {
    pub fn as_str(&self) -> &str {
        match *self {
            FollowPolicy::AutoAccept => "ACCEPT",
            FollowPolicy::AutoReject => "REJECT",
            FollowPolicy::ManualReview => "MANUAL",
        }
    }
}

impl fmt::Display for FollowPolicy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Serialize for FollowPolicy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
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

impl<'de> Deserialize<'de> for FollowPolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<FollowPolicy>().map_err(serde::de::Error::custom)
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
                .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)
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

    fn cause(&self) -> Option<&dyn StdError> {
        None
    }
}
