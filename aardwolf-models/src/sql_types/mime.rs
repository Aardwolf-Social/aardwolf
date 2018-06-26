<<<<<<< HEAD
use std::error::Error as StdError;
use std::io::Write;
=======
use std::{error::Error as StdError, io::Write};
>>>>>>> origin/master

use diesel::{backend::Backend, deserialize, serialize, sql_types::Text};
use mime::Mime as OrigMime;

#[derive(AsExpression, Debug, FromSqlRow)]
#[sql_type = "Text"]
pub struct Mime(pub OrigMime);

impl<DB> serialize::ToSql<Text, DB> for Mime
where
    DB: Backend,
{
    fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, DB>) -> serialize::Result {
        serialize::ToSql::<Text, DB>::to_sql(self.0.as_ref(), out)
    }
}

impl<DB> deserialize::FromSql<Text, DB> for Mime
where
    DB: Backend<RawValue = [u8]>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        deserialize::FromSql::<Text, DB>::from_sql(bytes).and_then(|s: String| {
            s.parse()
                .map(Mime)
                .map_err(|e| Box::new(e) as Box<StdError + Send + Sync>)
        })
    }
}

impl From<OrigMime> for Mime {
    fn from(u: OrigMime) -> Self {
        Mime(u)
    }
}
