use std::{error::Error as StdError, fmt, io::Write, str::FromStr};

use diesel::{backend::Backend, deserialize, serialize, sql_types::Text};

#[derive(AsExpression, Clone, Copy, Debug, Eq, FromSqlRow, Hash, PartialEq)]
#[sql_type = "Text"]
pub enum Role {
    Verified,
    Moderator,
    Admin,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Role::Verified => write!(f, "verified"),
            Role::Moderator => write!(f, "moderator"),
            Role::Admin => write!(f, "admin"),
        }
    }
}

impl FromStr for Role {
    type Err = RoleParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "verified" => Ok(Role::Verified),
            "moderator" => Ok(Role::Moderator),
            "admin" => Ok(Role::Admin),
            _ => Err(RoleParseError),
        }
    }
}

impl<DB> serialize::ToSql<Text, DB> for Role
where
    DB: Backend,
{
    fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, DB>) -> serialize::Result {
        serialize::ToSql::<Text, DB>::to_sql(&format!("{}", self), out)
    }
}

impl<DB> deserialize::FromSql<Text, DB> for Role
where
    DB: Backend<RawValue = [u8]>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        deserialize::FromSql::<Text, DB>::from_sql(bytes).and_then(|string: String| {
            string
                .parse::<Role>()
                .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RoleParseError;

impl fmt::Display for RoleParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse Role")
    }
}

impl StdError for RoleParseError {
    fn description(&self) -> &str {
        "Failed to parse Role"
    }

    fn cause(&self) -> Option<&dyn StdError> {
        None
    }
}
