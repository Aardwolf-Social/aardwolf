use std::error::Error as StdError;

use diesel::{backend::Backend, deserialize, serialize, sql_types::Text};
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
pub enum Permission {
    #[strum(serialize = "make-post")]
    MakePost,
    #[strum(serialize = "make-media-post")]
    MakeMediaPost,
    #[strum(serialize = "make-comment")]
    MakeComment,
    #[strum(serialize = "follow-user")]
    FollowUser,
    #[strum(serialize = "make-persona")]
    MakePersona,
    #[strum(serialize = "switch-persona")]
    SwitchPersona,
    #[strum(serialize = "delete-persona")]
    DeletePersona,
    #[strum(serialize = "manage-follow-request")]
    ManageFollowRequest,
    #[strum(serialize = "configure-instance")]
    ConfigureInstance,
    #[strum(serialize = "ban-user")]
    BanUser,
    #[strum(serialize = "block-instance")]
    BlockInstance,
    #[strum(serialize = "grant-role")]
    GrantRole,
    #[strum(serialize = "revoke-role")]
    RevokeRole,
}

impl<DB> serialize::ToSql<Text, DB> for Permission
where
    DB: Backend,
    str: serialize::ToSql<Text, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, DB>) -> serialize::Result {
        let name: &'static str = self.into();

        name.to_sql(out)
    }
}

impl<DB> deserialize::FromSql<Text, DB> for Permission
where
    DB: Backend,
    *const str: deserialize::FromSql<diesel::sql_types::Text, DB>,
{
    fn from_sql(bytes: <DB as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        deserialize::FromSql::<Text, DB>::from_sql(bytes).and_then(|string: String| {
            string
                .parse::<Permission>()
                .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)
        })
    }
}
