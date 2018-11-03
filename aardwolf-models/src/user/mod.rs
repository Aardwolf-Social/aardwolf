#![allow(proc_macro_derive_resolution_fallback)]
use chrono::{offset::Utc, DateTime};
use diesel::{self, connection::Connection, pg::PgConnection};

pub mod email;
pub mod local_auth;
mod permissions;
pub mod role;

use self::{
    email::{Email, EmailVerificationToken, UnverifiedEmail, VerifiedEmail, VerifyEmail},
    local_auth::LocalAuth,
};
pub use self::{
    local_auth::{PlaintextPassword, VerificationError},
    permissions::{PermissionError, PermissionResult, PermissionedUser, PersonaDeleter},
};
use schema::users;
use sql_types::Role;

pub trait UserLike {
    fn id(&self) -> i32;
    fn primary_email(&self) -> Option<i32>;
    fn created_at(&self) -> DateTime<Utc>;
    fn updated_at(&self) -> DateTime<Utc>;

    fn is_verified(&self, conn: &PgConnection) -> Result<bool, diesel::result::Error> {
        self.has_role(Role::Verified, conn)
    }

    fn is_moderator(&self, conn: &PgConnection) -> Result<bool, diesel::result::Error> {
        self.has_role(Role::Moderator, conn)
    }

    fn is_admin(&self, conn: &PgConnection) -> Result<bool, diesel::result::Error> {
        self.has_role(Role::Admin, conn)
    }

    fn has_role(&self, name: Role, conn: &PgConnection) -> Result<bool, diesel::result::Error> {
        use diesel::prelude::*;
        use schema::{roles, user_roles};

        roles::dsl::roles
            .inner_join(user_roles::dsl::user_roles)
            .filter(user_roles::dsl::user_id.eq(self.id()))
            .filter(roles::dsl::name.eq(name))
            .count()
            .get_result(conn)
            .map(|count: i64| count > 0)
    }
}

#[derive(Debug, Fail)]
pub enum UserVerifyError {
    #[fail(display = "Error in diesel: {}", _0)]
    Diesel(#[cause] diesel::result::Error),
    #[fail(display = "Cannot verify user with other user's ID")]
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

#[derive(Debug, Fail)]
pub enum UpdateFieldError {
    #[fail(display = "Error updating record: {}", _0)]
    Diesel(#[cause] diesel::result::Error),
    #[fail(display = "Provided records are not related")]
    Relation,
}

impl From<diesel::result::Error> for UpdateFieldError {
    fn from(e: diesel::result::Error) -> Self {
        UpdateFieldError::Diesel(e)
    }
}

#[derive(Identifiable, Queryable)]
#[table_name = "users"]
pub struct AuthenticatedUser {
    id: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    primary_email: Option<i32>,
}

impl AuthenticatedUser {
    pub fn get_authenticated_user_by_id(
        id: i32,
        conn: &PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::prelude::*;

        users::table.find(id).get_result(conn)
    }

    pub fn set_default_email(
        &mut self,
        email: &VerifiedEmail,
        conn: &PgConnection,
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
                ()
            })
    }

    fn verify(&self, email: &VerifiedEmail, conn: &PgConnection) -> Result<(), UserVerifyError> {
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
        conn: &PgConnection,
    ) -> Result<(AuthenticatedUser, VerifiedEmail), UserVerifyError> {
        conn.transaction(|| {
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
#[table_name = "users"]
pub struct QueriedUser {
    id: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    primary_email: Option<i32>,
}

impl UserLike for QueriedUser {
    fn id(&self) -> i32 {
        self.id
    }

    fn primary_email(&self) -> Option<i32> {
        self.primary_email
    }

    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

#[derive(Debug, Queryable, QueryableByName)]
#[table_name = "users"]
pub struct UnauthenticatedUser {
    id: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    primary_email: Option<i32>,
}

impl UnauthenticatedUser {
    pub fn log_in_local(
        self,
        local_auth: LocalAuth,
        password: PlaintextPassword,
    ) -> Result<AuthenticatedUser, VerificationError> {
        local_auth.log_in(self, password)
    }

    pub fn to_verified(
        self,
        conn: &PgConnection,
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

    pub fn by_id(id: i32, conn: &PgConnection) -> Result<Self, diesel::result::Error> {
        use diesel::prelude::*;

        users::table.find(id).first(conn)
    }

    pub fn by_email_id(
        email_id: i32,
        conn: &PgConnection,
    ) -> Result<(Self, Email), diesel::result::Error> {
        use diesel::prelude::*;
        use schema::emails;

        users::dsl::users
            .inner_join(emails::dsl::emails.on(emails::dsl::user_id.eq(users::dsl::id)))
            .filter(emails::dsl::id.eq(email_id))
            .first(conn)
    }

    pub fn by_email_for_auth(
        email: &str,
        conn: &PgConnection,
    ) -> Result<(Self, Email, LocalAuth), diesel::result::Error> {
        use diesel::prelude::*;
        use schema::{emails, local_auth};

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

    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    created_at: DateTime<Utc>,
    primary_email: Option<i32>,
}

impl NewUser {
    pub fn insert(self, conn: &PgConnection) -> Result<UnauthenticatedUser, diesel::result::Error> {
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

#[cfg(test)]
mod tests {
    use super::UnauthenticatedUser;
    use test_helper::*;

    #[test]
    fn create_user() {
        with_connection(|conn| with_unverified_user(conn, |_| Ok(())))
    }

    #[test]
    fn verify_and_log_in_user() {
        with_connection(|conn| {
            make_verified_authenticated_user(conn, "testpass", |_user, _email| Ok(()))
        })
    }

    #[test]
    fn log_in_unverified_user() {
        with_connection(|conn| {
            with_unverified_user(conn, |user| {
                with_unverified_email(conn, &user, |email, _token| {
                    let password = "password";

                    with_local_auth(conn, &user, password, |_| {
                        let (user, _, auth) =
                            UnauthenticatedUser::by_email_for_auth(email.email(), conn)?;

                        user.log_in_local(auth, create_plaintext_password(password)?)?;

                        Ok(())
                    })
                })
            })
        })
    }

    #[test]
    fn log_in_verified_user() {
        with_connection(|conn| {
            let password = "testpass";
            make_verified_authenticated_user(conn, password, |_user, email| {
                let (user, _, auth) = UnauthenticatedUser::by_email_for_auth(email.email(), conn)?;

                user.log_in_local(auth, create_plaintext_password(password)?)?;

                Ok(())
            })
        })
    }
}
