use chrono::{offset::Utc, DateTime};
use diesel::{self, pg::PgConnection};

mod password;

use crate::{
    schema::local_auth,
    user::{
        local_auth::password::Password, AuthenticatedUser, UnauthenticatedUser, UnverifiedUser,
    },
};

pub use self::password::{
    CreationError as PasswordCreationError, PlaintextPassword, ValidationError, VerificationError,
};

/// `LocalAuth` can be queried from the database, but is only really usable as a tool to "log in" a
/// user.
#[derive(Debug, Queryable, QueryableByName)]
#[diesel(table_name = local_auth)]
pub struct LocalAuth {
    id: i32,
    password: Password,
    user_id: i32, // foreign key to User
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl LocalAuth {
    pub fn id(&self) -> i32 {
        self.id
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

    /// Log In a user, given an `UnauthenticatedUser` and a `PlaintextPassword`.
    ///
    /// This method ensures first that the `UnauthenticatedUser` is the same user that this
    /// `LocalAuth` is associated with, and then continues to verify the `PlaintextPassword`
    /// against this type's `Password`. Upon succesful password verification, an
    /// `AuthenticatedUser` is created.
    pub(crate) fn log_in(
        self,
        user: UnauthenticatedUser,
        password: PlaintextPassword,
    ) -> Result<AuthenticatedUser, VerificationError> {
        use self::password::Verify;

        if self.user_id != user.id {
            return Err(VerificationError::Process);
        }

        

        self.password.verify(password).map(|_| AuthenticatedUser {
            id: user.id,
            primary_email: user.primary_email,
            primary_persona: user.primary_persona,
            created_at: user.created_at,
            updated_at: user.updated_at,
        })
    }
}

/// This type exists to create new `LocalAuth` record in the database.
#[derive(Insertable)]
#[diesel(table_name = local_auth)]
pub struct NewLocalAuth {
    password: Password,
    created_at: DateTime<Utc>,
    user_id: i32,
}

impl NewLocalAuth {
    /// Insert into the database
    pub fn insert(self, conn: &mut PgConnection) -> Result<LocalAuth, diesel::result::Error> {
        use diesel::prelude::*;

        diesel::insert_into(local_auth::table)
            .values(&self)
            .get_result(conn)
    }

    /// Create a `NewLocalAuth`
    pub fn new(
        user: &UnverifiedUser,
        password: PlaintextPassword,
    ) -> Result<Self, PasswordCreationError> {
        use self::password::Validate;

        let password = password.validate()?;

        NewLocalAuth::create(user, password)
    }

    /// Create a `NewLocalAuth` with a redundant password to check for consistency.
    pub fn new_from_two(
        user: &UnverifiedUser,
        password: PlaintextPassword,
        password2: PlaintextPassword,
    ) -> Result<Self, PasswordCreationError> {
        use self::password::Validate;

        let password = password
            .validate()
            .and_then(|password| password.compare(password2))?;

        NewLocalAuth::create(user, password)
    }

    fn create(
        user: &UnverifiedUser,
        password: PlaintextPassword,
    ) -> Result<Self, PasswordCreationError> {
        use self::password::Create;
        let password = Password::create(password)?;

        Ok(NewLocalAuth {
            password,
            created_at: Utc::now(),
            user_id: user.id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::NewLocalAuth;
    use crate::test_helper::*;

    #[test]
    fn create_local_auth() {
        with_connection(|conn| {
            let user = make_unverified_user(conn)?;
            let password = "testpass";

            let _ = make_local_auth(conn, &user, password);

            Ok(())
        })
    }

    #[test]
    fn dont_create_local_auth_with_invalid_password() {
        with_connection(|conn| {
            let user = make_unverified_user(conn)?;
            let password = create_plaintext_password("short")?;

            let local_auth = NewLocalAuth::new(&user, password);

            assert!(
                local_auth.is_err(),
                "Should not have created local auth with bad password"
            );

            Ok(())
        })
    }

    #[test]
    fn dont_create_local_auth_with_mismatched_passwords() {
        with_connection(|conn| {
            let user = make_unverified_user(conn)?;
            let p1 = create_plaintext_password("agoodpassword")?;
            let p2 = create_plaintext_password("abadpassword")?;

            let local_auth = NewLocalAuth::new_from_two(&user, p1, p2);

            assert!(
                local_auth.is_err(),
                "Should not have created LocalAuth from mismatched passwords"
            );

            Ok(())
        })
    }
}
