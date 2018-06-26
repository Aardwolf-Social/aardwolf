use std::{error::Error as StdError, fmt, io::Write, str::FromStr};

use diesel::{backend::Backend, deserialize, serialize, sql_types::Text};

#[derive(AsExpression, Clone, Copy, Debug, Eq, FromSqlRow, Hash, PartialEq)]
#[sql_type = "Text"]
pub enum Permission {
    MakePost,
    MakeMediaPost,
    MakeComment,
    FollowUser,
    MakePersona,
    SwitchPersona,
    DeletePersona,
    ManageFollowRequest,
    ConfigureInstance,
    BanUser,
    BlockInstance,
    GrantRole,
    RevokeRole,
}

impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Permission::MakePost => write!(f, "make-post"),
            Permission::MakeMediaPost => write!(f, "make-media-post"),
            Permission::MakeComment => write!(f, "make-comment"),
            Permission::FollowUser => write!(f, "follow-user"),
            Permission::MakePersona => write!(f, "make-persona"),
            Permission::SwitchPersona => write!(f, "switch-persona"),
            Permission::DeletePersona => write!(f, "delete-persona"),
            Permission::ManageFollowRequest => write!(f, "manage-follow-request"),
            Permission::ConfigureInstance => write!(f, "configure-instance"),
            Permission::BanUser => write!(f, "ban-user"),
            Permission::BlockInstance => write!(f, "block-instance"),
            Permission::GrantRole => write!(f, "grant-role"),
            Permission::RevokeRole => write!(f, "revoke-role"),
        }
    }
}

impl FromStr for Permission {
    type Err = PermissionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "make-post" => Ok(Permission::MakePost),
            "make-media-post" => Ok(Permission::MakeMediaPost),
            "make-comment" => Ok(Permission::MakeComment),
            "follow-user" => Ok(Permission::FollowUser),
            "make-persona" => Ok(Permission::MakePersona),
            "switch-persona" => Ok(Permission::SwitchPersona),
            "delete-persona" => Ok(Permission::DeletePersona),
            "manage-follow-request" => Ok(Permission::ManageFollowRequest),
            "configure-instance" => Ok(Permission::ConfigureInstance),
            "ban-user" => Ok(Permission::BanUser),
            "block-instance" => Ok(Permission::BlockInstance),
            "grant-role" => Ok(Permission::GrantRole),
            "revoke-role" => Ok(Permission::RevokeRole),
            _ => Err(PermissionParseError),
        }
    }
}

impl<DB> serialize::ToSql<Text, DB> for Permission
where
    DB: Backend,
{
    fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, DB>) -> serialize::Result {
        serialize::ToSql::<Text, DB>::to_sql(&format!("{}", self), out)
    }
}

impl<DB> deserialize::FromSql<Text, DB> for Permission
where
    DB: Backend<RawValue = [u8]>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        deserialize::FromSql::<Text, DB>::from_sql(bytes).and_then(|string: String| {
            string
                .parse::<Permission>()
                .map_err(|e| Box::new(e) as Box<StdError + Send + Sync>)
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PermissionParseError;

impl fmt::Display for PermissionParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse Permission")
    }
}

impl StdError for PermissionParseError {
    fn description(&self) -> &str {
        "Failed to parse Permission"
    }

    fn cause(&self) -> Option<&StdError> {
        None
    }
}
