use std::error::Error as StdError;

use diesel::{backend::Backend, deserialize, serialize, sql_types::Text};
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};
use strum_macros::{Display, EnumString, IntoStaticStr};

#[derive(
    AsExpression,
    Clone,
    Copy,
    Debug,
    Display,
    EnumString,
    Eq,
    FromSqlRow,
    Hash,
    IntoStaticStr,
    PartialEq,
)]
#[diesel(sql_type = Text)]
pub enum FollowPolicy {
    #[strum(serialize = "ACCEPT")]
    AutoAccept,
    #[strum(serialize = "REJECT")]
    AutoReject,
    #[strum(serialize = "MANUAL")]
    ManualReview,
}

impl Serialize for FollowPolicy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
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
    str: serialize::ToSql<Text, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, DB>) -> serialize::Result {
        let name: &'static str = self.into();

        name.to_sql(out)
    }
}

impl<DB> deserialize::FromSql<Text, DB> for FollowPolicy
where
    DB: Backend,
    *const str: deserialize::FromSql<diesel::sql_types::Text, DB>,
{
    fn from_sql(bytes: <DB as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        deserialize::FromSql::<Text, DB>::from_sql(bytes).and_then(|string: String| {
            string
                .parse::<FollowPolicy>()
                .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)
        })
    }
}
