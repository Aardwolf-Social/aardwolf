use std::{error::Error as StdError, str::FromStr};

use diesel::{backend::Backend, deserialize, serialize, sql_types::Text};
use url::Url as OrigUrl;

#[derive(AsExpression, Clone, Debug, FromSqlRow)]
#[diesel(sql_type = Text)]
pub struct Url(pub OrigUrl);

impl<DB> serialize::ToSql<Text, DB> for Url
where
    DB: Backend,
    str: serialize::ToSql<Text, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, DB>) -> serialize::Result {
        self.0.as_str().to_sql(out)
    }
}

impl<DB> deserialize::FromSql<Text, DB> for Url
where
    DB: Backend,
    *const str: deserialize::FromSql<diesel::sql_types::Text, DB>,
{
    fn from_sql(bytes: <DB as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        deserialize::FromSql::<Text, DB>::from_sql(bytes).and_then(|s: String| {
            s.parse()
                .map(Url)
                .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)
        })
    }
}

impl From<OrigUrl> for Url {
    fn from(u: OrigUrl) -> Self {
        Url(u)
    }
}

impl FromStr for Url {
    type Err = <OrigUrl as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        FromStr::from_str(s).map(Url)
    }
}
