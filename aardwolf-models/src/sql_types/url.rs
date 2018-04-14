use std::io::Write;
use std::error::Error as StdError;

use diesel::backend::Backend;
use diesel::deserialize;
use diesel::serialize;
use diesel::sql_types::Text;
use url::Url as OrigUrl;

#[derive(AsExpression, Debug, FromSqlRow)]
#[sql_type = "Text"]
pub struct Url(pub OrigUrl);

impl<DB> serialize::ToSql<Text, DB> for Url
where
    DB: Backend,
{
    fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, DB>) -> serialize::Result {
        serialize::ToSql::<Text, DB>::to_sql(self.0.as_str(), out)
    }
}

impl<DB> deserialize::FromSql<Text, DB> for Url
where
    DB: Backend<RawValue = [u8]>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        deserialize::FromSql::<Text, DB>::from_sql(bytes).and_then(|s: String| {
            s.parse()
                .map(Url)
                .map_err(|e| Box::new(e) as Box<StdError + Send + Sync>)
        })
    }
}

impl From<OrigUrl> for Url {
    fn from(u: OrigUrl) -> Self {
        Url(u)
    }
}
