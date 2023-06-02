use std::{error::Error as StdError, io::Write};

use diesel::{backend::Backend, deserialize, serialize, sql_types::Text};
use mime::Mime as OrigMime;
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

#[derive(AsExpression, Clone, Debug, FromSqlRow)]
#[diesel(sql_type = Text)]
pub struct Mime(pub OrigMime);

impl<DB> serialize::ToSql<Text, DB> for Mime
where
    DB: Backend,
{
    fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, DB>) -> serialize::Result {
        serialize::ToSql::<Text, DB>::to_sql(self.as_ref(), out)
    }
}

impl<DB> deserialize::FromSql<Text, DB> for Mime
where
    DB: Backend<RawValue = [u8]>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        deserialize::FromSql::<Text, DB>::from_sql(bytes).and_then(|s: String| {
            s.parse()
                .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)
        })
    }
}

impl std::str::FromStr for Mime {
    type Err = mime::FromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Mime(s.parse()?))
    }
}

impl AsRef<str> for Mime {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl std::fmt::Display for Mime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl From<OrigMime> for Mime {
    fn from(u: OrigMime) -> Self {
        Mime(u)
    }
}

impl Serialize for Mime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Mime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}
