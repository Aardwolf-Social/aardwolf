#![allow(proc_macro_derive_resolution_fallback)]
mod token;

use chrono::{offset::Utc, DateTime};
use diesel::{self, pg::PgConnection};

use self::token::{create_token, HashedEmailToken};
pub use self::token::{CreationError, EmailToken, EmailVerificationToken, VerificationError};
use schema::emails;
use user::{AuthenticatedUser, UnverifiedUser, UserLike};

pub struct VerifiedEmail {
    id: i32,
    email: String,
    user_id: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl VerifiedEmail {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

#[derive(Debug, Queryable, QueryableByName)]
#[table_name = "emails"]
pub struct Email {
    id: i32,
    email: String,
    user_id: i32,
    verified: bool,
    verification_token: Option<HashedEmailToken>,
    confirmed_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Email {
    pub fn first_by_user_id(
        user_id: i32,
        conn: &PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::prelude::*;

        emails::table
            .filter(emails::dsl::user_id.eq(user_id))
            .get_result(conn)
    }

    pub fn by_id(id: i32, conn: &PgConnection) -> Result<Self, diesel::result::Error> {
        use diesel::prelude::*;

        emails::table
            .filter(emails::dsl::id.eq(id))
            .get_result(conn)
    }

    pub fn into_verified(self) -> Result<VerifiedEmail, UnverifiedEmail> {
        if self.verified {
            Ok(VerifiedEmail {
                id: self.id,
                email: self.email,
                user_id: self.user_id,
                created_at: self.created_at,
                updated_at: self.updated_at,
            })
        } else {
            Err(UnverifiedEmail {
                id: self.id,
                email: self.email,
                user_id: self.user_id,
                verified: false,
                verification_token: self.verification_token,
                confirmed_at: None,
                created_at: self.created_at,
                updated_at: self.updated_at,
            })
        }
    }
}

pub struct VerifyEmail {
    id: i32,
    email: String,
    user_id: i32,
    #[allow(dead_code)]
    verified: bool,
    #[allow(dead_code)]
    verification_token: Option<HashedEmailToken>,
    confirmed_at: DateTime<Utc>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(AsChangeset)]
#[table_name = "emails"]
pub struct EmailVerificationChangeset {
    #[allow(dead_code)]
    id: i32,
    verified: bool,
    verification_token: Option<HashedEmailToken>,
    confirmed_at: DateTime<Utc>,
}

impl VerifyEmail {
    pub(crate) fn store_verify(
        self,
        conn: &PgConnection,
    ) -> Result<VerifiedEmail, diesel::result::Error> {
        use diesel::prelude::*;
        use schema::emails;

        diesel::update(emails::table)
            .set(&EmailVerificationChangeset {
                id: self.id,
                verified: true,
                verification_token: None,
                confirmed_at: self.confirmed_at,
            })
            .execute(conn)
            .map(|_| VerifiedEmail {
                id: self.id,
                email: self.email,
                user_id: self.user_id,
                created_at: self.created_at,
                updated_at: self.updated_at,
            })
            .map_err(|e| {
                error!("Failed to verify email: {}, {:?}", e, e);
                e
            })
    }
}

#[derive(Queryable, QueryableByName)]
#[table_name = "emails"]
pub struct UnverifiedEmail {
    id: i32,
    email: String,
    user_id: i32, // foreign key to User
    verified: bool,
    verification_token: Option<HashedEmailToken>,
    #[allow(dead_code)]
    confirmed_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl UnverifiedEmail {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub(crate) fn verify_and_log_in(
        self,
        user: UnverifiedUser,
        token: EmailVerificationToken,
    ) -> Result<(AuthenticatedUser, VerifyEmail), VerificationError> {
        let res = self.verify(token).map(|verify_email| {
            (
                AuthenticatedUser {
                    id: user.id,
                    primary_email: None,
                    primary_persona: None,
                    created_at: user.created_at,
                    updated_at: user.updated_at,
                },
                verify_email,
            )
        });

        drop(user);

        res
    }

    pub fn verify(self, token: EmailVerificationToken) -> Result<VerifyEmail, VerificationError> {
        if self.verification_token.is_some() && !self.verified {
            token::VerifyEmail::verify_email(self.verification_token.as_ref().unwrap(), token).map(
                |_| VerifyEmail {
                    id: self.id,
                    email: self.email,
                    user_id: self.user_id,
                    verified: true,
                    verification_token: None,
                    confirmed_at: Utc::now(),
                    created_at: self.created_at,
                    updated_at: self.updated_at,
                },
            )
        } else {
            Err(VerificationError::Process)
        }
    }
}

#[derive(Insertable)]
#[table_name = "emails"]
pub struct NewEmail {
    email: String,
    user_id: i32,
    verified: bool,
    verification_token: HashedEmailToken,
}

impl NewEmail {
    pub fn insert(self, conn: &PgConnection) -> Result<UnverifiedEmail, diesel::result::Error> {
        use diesel::prelude::*;

        diesel::insert_into(emails::table)
            .values(&self)
            .get_result(conn)
    }

    pub fn new<U: UserLike>(email: String, user: &U) -> Result<(Self, EmailToken), CreationError> {
        create_token().map(|(email_token, verification_token)| {
            (
                NewEmail {
                    email,
                    user_id: user.id(),
                    verified: false,
                    verification_token,
                },
                email_token,
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use test_helper::*;

    #[test]
    fn create_email() {
        with_connection(|conn| {
            with_unverified_user(conn, |user| {
                with_unverified_email(conn, &user, |_email, _token| Ok(()))
            })
        })
    }

    #[test]
    fn verify_email() {
        with_connection(|conn| {
            with_unverified_user(conn, |user| {
                with_unverified_email(conn, &user, |email, token| {
                    let token = transmute_email_token(&token)?;
                    email.verify(token)?.store_verify(conn)?;

                    Ok(())
                })
            })
        })
    }
}
