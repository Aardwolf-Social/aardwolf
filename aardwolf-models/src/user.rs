use chrono::{offset::Utc, DateTime};
use diesel::{self, connection::Connection, pg::PgConnection};
use thiserror::Error;

pub mod email;
pub mod local_auth;
mod permissions;
pub mod role;

use crate::{
    schema::users,
    sql_types::Role,
    user::{
        email::{Email, EmailVerificationToken, UnverifiedEmail, VerifiedEmail, VerifyEmail},
        local_auth::LocalAuth,
    },
};

pub use self::{
    local_auth::{PlaintextPassword, VerificationError},
    permissions::{
        LocalPersonaCreator, LocalPostCreator, PermissionError, PermissionResult, PermissionedUser,
        PersonaDeleter,
    },
};

pub trait UserLike {
    fn id(&self) -> i32;
    fn primary_email(&self) -> Option<i32>;
    fn primary_persona(&self) -> Option<i32>;
    fn created_at(&self) -> DateTime<Utc>;
    fn updated_at(&self) -> DateTime<Utc>;

    fn is_verified(&self, conn: &mut PgConnection) -> Result<bool, diesel::result::Error> {
        self.has_role(Role::Verified, conn)
    }

    fn is_moderator(&self, conn: &mut PgConnection) -> Result<bool, diesel::result::Error> {
        self.has_role(Role::Moderator, conn)
    }

    fn is_admin(&self, conn: &mut PgConnection) -> Result<bool, diesel::result::Error> {
        self.has_role(Role::Admin, conn)
    }

    fn has_role(&self, name: Role, conn: &mut PgConnection) -> Result<bool, diesel::result::Error> {
        use crate::schema::{roles, user_roles};
        use diesel::prelude::*;

        roles::dsl::roles
            .inner_join(user_roles::dsl::user_roles)
            .filter(user_roles::dsl::user_id.eq(self.id()))
            .filter(roles::dsl::name.eq(name))
            .count()
            .get_result(conn)
            .map(|count: i64| count > 0)
    }
}

#[derive(Debug, Error)]
pub enum UserVerifyError {
    #[error("Error in diesel: {}", _0)]
    Diesel(#[source] diesel::result::Error),
    #[error("Cannot verify user with other user's ID")]
    IdMismatch,
}

impl From<diesel::result::Error> for UserVerifyError {
    fn from(e: diesel::result::Error) -> Self {
        UserVerifyError::Diesel(e)
    }
}

impl From<UpdateFieldError> for UserVerifyError {
    fn from(e: UpdateFieldError) -> Self {
        match e {
            UpdateFieldError::Diesel(d) => UserVerifyError::Diesel(d),
            UpdateFieldError::Relation => UserVerifyError::IdMismatch,
        }
    }
}

#[derive(Debug, Error)]
pub enum UpdateFieldError {
    #[error("Error updating record: {}", _0)]
    Diesel(#[source] diesel::result::Error),
    #[error("Provided records are not related")]
    Relation,
}

impl From<diesel::result::Error> for UpdateFieldError {
    fn from(e: diesel::result::Error) -> Self {
        UpdateFieldError::Diesel(e)
    }
}

#[derive(Debug, Clone, Identifiable, Queryable)]
#[diesel(table_name = users)]
pub struct AuthenticatedUser {
    id: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    primary_email: Option<i32>,
    primary_persona: Option<i32>,
}

impl AuthenticatedUser {
    pub fn get_authenticated_user_by_id(
        id: i32,
        conn: &mut PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::prelude::*;

        users::table.find(id).get_result(conn)
    }

    pub fn set_default_email(
        &mut self,
        email: &VerifiedEmail,
        conn: &mut PgConnection,
    ) -> Result<(), UpdateFieldError> {
        if email.user_id() != self.id {
            return Err(UpdateFieldError::Relation);
        }

        use diesel::prelude::*;

        diesel::update(&*self)
            .set(users::primary_email.eq(Some(email.id())))
            .execute(conn)
            .map_err(From::from)
            .map(|_| {
                self.primary_email = Some(email.id());
            })
    }

    fn verify(
        &self,
        email: &VerifiedEmail,
        conn: &mut PgConnection,
    ) -> Result<(), UserVerifyError> {
        if self.id != email.user_id() {
            return Err(UserVerifyError::IdMismatch);
        }

        permissions::RoleGranter::new()
            .grant_role(self, Role::Verified, conn)
            .map_err(From::from)
    }
}

impl UserLike for AuthenticatedUser {
    fn id(&self) -> i32 {
        self.id
    }

    fn primary_email(&self) -> Option<i32> {
        self.primary_email
    }

    fn primary_persona(&self) -> Option<i32> {
        self.primary_persona
    }

    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

impl PermissionedUser for AuthenticatedUser {}

pub struct MemVerified {
    email: VerifyEmail,
    user: AuthenticatedUser,
}

impl MemVerified {
    pub fn store_verify(
        self,
        conn: &mut PgConnection,
    ) -> Result<(AuthenticatedUser, VerifiedEmail), UserVerifyError> {
        conn.transaction(|conn| {
            let MemVerified { email, mut user } = self;

            email
                .store_verify(conn)
                .map_err(From::from)
                .and_then(|verified_email| {
                    user.verify(&verified_email, conn).and_then(|_| {
                        user.set_default_email(&verified_email, conn)
                            .map(|_| (user, verified_email))
                            .map_err(From::from)
                    })
                })
        })
    }
}

pub struct UnverifiedUser {
    id: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl UserLike for UnverifiedUser {
    fn id(&self) -> i32 {
        self.id
    }

    fn primary_email(&self) -> Option<i32> {
        None
    }

    fn primary_persona(&self) -> Option<i32> {
        None
    }

    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

impl UnverifiedUser {
    pub fn verify(
        self,
        email: UnverifiedEmail,
        token: EmailVerificationToken,
    ) -> Result<MemVerified, email::VerificationError> {
        email
            .verify_and_log_in(self, token)
            .map(|(user, email)| MemVerified { email, user })
    }
}

#[derive(Debug, Queryable, QueryableByName)]
#[diesel(table_name = users)]
pub struct QueriedUser {
    id: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    primary_email: Option<i32>,
    primary_persona: Option<i32>,
}

impl UserLike for QueriedUser {
    fn id(&self) -> i32 {
        self.id
    }

    fn primary_email(&self) -> Option<i32> {
        self.primary_email
    }

    fn primary_persona(&self) -> Option<i32> {
        self.primary_persona
    }

    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

#[derive(Debug, Queryable, QueryableByName)]
#[diesel(table_name = users)]
pub struct UnauthenticatedUser {
    id: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    primary_email: Option<i32>,
    primary_persona: Option<i32>,
}

impl UnauthenticatedUser {
    pub fn log_in_local(
        self,
        local_auth: LocalAuth,
        password: PlaintextPassword,
    ) -> Result<AuthenticatedUser, VerificationError> {
        local_auth.log_in(self, password)
    }

    pub fn into_verified(
        self,
        conn: &mut PgConnection,
    ) -> Result<Result<UnauthenticatedUser, UnverifiedUser>, diesel::result::Error> {
        self.is_verified(conn).map(|has_role| {
            if has_role {
                Ok(self)
            } else {
                Err(UnverifiedUser {
                    id: self.id,
                    created_at: self.created_at,
                    updated_at: self.updated_at,
                })
            }
        })
    }

    pub fn by_id(id: i32, conn: &mut PgConnection) -> Result<Self, diesel::result::Error> {
        use diesel::prelude::*;

        users::table.find(id).first(conn)
    }

    pub fn by_email_id(
        email_id: i32,
        conn: &mut PgConnection,
    ) -> Result<(Self, Email), diesel::result::Error> {
        use crate::schema::emails;
        use diesel::prelude::*;

        users::dsl::users
            .inner_join(emails::dsl::emails.on(emails::dsl::user_id.eq(users::dsl::id)))
            .filter(emails::dsl::id.eq(email_id))
            .first(conn)
    }

    pub fn by_email_for_auth(
        email: &str,
        conn: &mut PgConnection,
    ) -> Result<(Self, Email, LocalAuth), diesel::result::Error> {
        use crate::schema::{emails, local_auth};
        use diesel::prelude::*;

        users::dsl::users
            .inner_join(emails::dsl::emails.on(emails::dsl::user_id.eq(users::dsl::id)))
            .inner_join(local_auth::dsl::local_auth)
            .filter(emails::dsl::email.eq(email))
            .first(conn)
    }
}

impl UserLike for UnauthenticatedUser {
    fn id(&self) -> i32 {
        self.id
    }

    fn primary_email(&self) -> Option<i32> {
        self.primary_email
    }

    fn primary_persona(&self) -> Option<i32> {
        self.primary_persona
    }

    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    created_at: DateTime<Utc>,
    primary_email: Option<i32>,
}

impl NewUser {
    pub fn insert(
        self,
        conn: &mut PgConnection,
    ) -> Result<UnauthenticatedUser, diesel::result::Error> {
        use diesel::prelude::*;

        diesel::insert_into(users::table)
            .values(&self)
            .get_result(conn)
    }

    pub fn new() -> Self {
        NewUser {
            created_at: Utc::now(),
            primary_email: None,
        }
    }
}

impl Default for NewUser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::UnauthenticatedUser;
    use crate::test_helper::*;

    #[test]
    fn create_user() {
        with_connection(|conn| {
            let user = make_unverified_user(conn);

            assert!(user.is_ok());

            Ok(())
        });
    }

    #[test]
    fn verify_and_log_in_user() {
        with_connection(|conn| {
            let user = make_verified_authenticated_user(conn, "testpass");

            assert!(user.is_ok());

            Ok(())
        })
    }

    #[test]
    fn log_in_unverified_user() {
        with_connection(|conn| {
            let user = make_unverified_user(conn)?;
            let (email, _) = make_unverified_email(conn, &user)?;
            let password = "password";
            let _ = make_local_auth(conn, &user, password);
            let (user, _, auth) = UnauthenticatedUser::by_email_for_auth(email.email(), conn)?;

            let result = user.log_in_local(auth, create_plaintext_password(password)?);

            assert!(result.is_ok());

            Ok(())
        })
    }

    #[test]
    fn log_in_verified_user() {
        with_connection(|conn| {
            let password = "testpass";
            let (_, email) = make_verified_authenticated_user(conn, password).unwrap();

            let (user, _, auth) =
                UnauthenticatedUser::by_email_for_auth(email.email(), conn).unwrap();

            let result = user.log_in_local(auth, create_plaintext_password(password)?);

            assert!(result.is_ok());

            Ok(())
        })
    }
}
