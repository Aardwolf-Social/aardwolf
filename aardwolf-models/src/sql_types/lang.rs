use std::{error::Error as StdError, fmt, io::Write, str::FromStr};

<<<<<<< HEAD
use diesel::backend::Backend;
use diesel::deserialize;
use diesel::serialize;
use diesel::sql_types::Text;
=======
use diesel::{backend::Backend, deserialize, serialize, sql_types::Text};
>>>>>>> origin/master

#[derive(AsExpression, Clone, Copy, Debug, Eq, FromSqlRow, Hash, PartialEq)]
#[sql_type = "Text"]
pub enum Lang {
    EnUs,
    EnUk,
    EnAu,
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Lang::EnUs => write!(f, "EnUs"),
            Lang::EnUk => write!(f, "EnUk"),
            Lang::EnAu => write!(f, "EnAu"),
        }
    }
}

impl FromStr for Lang {
    type Err = LangParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "EnUs" => Ok(Lang::EnUs),
            "EnUk" => Ok(Lang::EnUk),
            "EnAu" => Ok(Lang::EnAu),
            _ => Err(LangParseError),
        }
    }
}

impl<DB> serialize::ToSql<Text, DB> for Lang
where
    DB: Backend,
{
    fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, DB>) -> serialize::Result {
        serialize::ToSql::<Text, DB>::to_sql(&format!("{}", self), out)
    }
}

impl<DB> deserialize::FromSql<Text, DB> for Lang
where
    DB: Backend<RawValue = [u8]>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        deserialize::FromSql::<Text, DB>::from_sql(bytes).and_then(|string: String| {
            string
                .parse::<Lang>()
                .map_err(|e| Box::new(e) as Box<StdError + Send + Sync>)
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LangParseError;

impl fmt::Display for LangParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse Lang")
    }
}

impl StdError for LangParseError {
    fn description(&self) -> &str {
        "Failed to parse Lang"
    }

    fn cause(&self) -> Option<&StdError> {
        None
    }
}
