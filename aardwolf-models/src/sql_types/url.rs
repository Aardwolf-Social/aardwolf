use std::io::Write;
use std::error::Error as StdError;
use std::str::FromStr;

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

impl FromStr for Url {
    type Err = <OrigUrl as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        FromStr::from_str(s).map(Url)
    }
}

mod rocket {
    use std::str::Utf8Error;

    use rocket::http::RawStr;
    use rocket::request::FromFormValue;
    use url::ParseError;

    use super::Url;

    impl<'v> FromFormValue<'v> for Url {
        type Error = UrlParseError;

        fn from_form_value(form_value: &'v RawStr) -> Result<Self, Self::Error> {
            Ok(Url(form_value.url_decode()?.parse()?))
        }
    }

    #[derive(Debug, Fail)]
    pub enum UrlParseError {
        #[fail(display = "Failed to parse URL, {:?}", _0)]
        Url(ParseError),
        #[fail(display = "Failed to read bytes, {}", _0)]
        Decode(#[cause] Utf8Error),
    }

    impl From<ParseError> for UrlParseError {
        fn from(e: ParseError) -> Self {
            UrlParseError::Url(e)
        }
    }

    impl From<Utf8Error> for UrlParseError {
        fn from(e: Utf8Error) -> Self {
            UrlParseError::Decode(e)
        }
    }
}
