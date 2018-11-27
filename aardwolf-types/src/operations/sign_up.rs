use aardwolf_models::user::{
    email::{CreationError, EmailToken, NewEmail, UnverifiedEmail},
    local_auth::{NewLocalAuth, PasswordCreationError},
    NewUser,
};
use diesel::{self, pg::PgConnection, Connection};

use crate::{
    error::AardwolfFail,
    forms::auth::{ValidateSignUpFormFail, ValidatedSignUpForm},
    traits::DbAction,
    wrapper::{DbActionWrapper, Wrapped},
};

pub struct SignUp(pub ValidatedSignUpForm);

impl Wrapped for SignUp {
    type Wrapper = DbActionWrapper<Self, <Self as DbAction>::Item, <Self as DbAction>::Error>;
}

impl DbAction for SignUp {
    type Item = (UnverifiedEmail, EmailToken);
    type Error = SignUpFail;

    fn db_action(self, conn: &PgConnection) -> Result<(UnverifiedEmail, EmailToken), SignUpFail> {
        conn.transaction::<_, SignUpFail, _>(|| {
            let user = NewUser::new()
                .insert(conn)
                .map_err(|_| SignUpFail::UserCreateError)?;

            let user = match user
                .into_verified(conn)
                .map_err(|_| SignUpFail::UserLookup)?
            {
                Ok(_unauthenticatec_user) => return Err(SignUpFail::VerifiedUser),
                Err(unverified_user) => unverified_user,
            };

            let _local_auth =
                NewLocalAuth::new_from_two(&user, self.0.password, self.0.password_confirmation)?
                    .insert(conn)
                    .map_err(|_| SignUpFail::LocalAuthCreateError)?;

            let (email, token) = NewEmail::new(self.0.email, &user)?;

            let email = email
                .insert(conn)
                .map_err(|_| SignUpFail::EmailCreateError)?;

            Ok((email, token))
        })
    }
}

#[derive(Clone, Debug, Fail, Serialize)]
pub enum SignUpFail {
    #[fail(display = "Sign up failed because {}", _0)]
    ValidationError(#[cause] ValidateSignUpFormFail),
    #[fail(display = "Failed to insert local_auth into database")]
    LocalAuthCreateError,
    #[fail(display = "Failed to insert user into database")]
    UserCreateError,
    #[fail(display = "Failed to insert email into database")]
    EmailCreateError,
    #[fail(display = "Failed to hash password")]
    PasswordHashError,
    #[fail(display = "Failed to create confirmation token")]
    CreateTokenError,
    #[fail(display = "New user shouldn't be verified")]
    VerifiedUser,
    #[fail(display = "Failed to lookup newly created user")]
    UserLookup,
    #[fail(display = "Failed to perform database transaction")]
    Transaction,
}

impl AardwolfFail for SignUpFail {}

impl From<ValidateSignUpFormFail> for SignUpFail {
    fn from(e: ValidateSignUpFormFail) -> SignUpFail {
        SignUpFail::ValidationError(e)
    }
}

impl From<diesel::result::Error> for SignUpFail {
    fn from(_: diesel::result::Error) -> Self {
        SignUpFail::Transaction
    }
}

impl From<PasswordCreationError> for SignUpFail {
    fn from(e: PasswordCreationError) -> Self {
        match e {
            PasswordCreationError::Validation(e) => SignUpFail::ValidationError(e.into()),
            PasswordCreationError::Bcrypt => SignUpFail::PasswordHashError,
        }
    }
}

impl From<CreationError> for SignUpFail {
    fn from(_: CreationError) -> Self {
        SignUpFail::CreateTokenError
    }
}

#[cfg(test)]
mod tests {
    use aardwolf_test_helpers::models::{
        create_plaintext_password, gen_string, with_connection, GenericError,
    };
    use diesel::pg::PgConnection;

    use crate::{forms::auth::ValidatedSignUpForm, operations::sign_up::SignUp, traits::DbAction};

    fn setup<F>(f: F)
    where
        F: FnOnce(&PgConnection, ValidatedSignUpForm) -> Result<(), GenericError>,
    {
        with_connection(|conn| {
            let email = format!("{}@{}.{}", gen_string()?, gen_string()?, gen_string()?);
            let pass = gen_string()?;

            let form = ValidatedSignUpForm {
                email,
                password: create_plaintext_password(&pass)?,
                password_confirmation: create_plaintext_password(&pass)?,
            };

            f(conn, form)
        })
    }

    #[test]
    fn sign_up_succeedes() {
        setup(|conn, form| {
            let operation = SignUp(form);

            assert!(operation.db_action(conn).is_ok());
            Ok(())
        })
    }

    #[test]
    fn password_mismatch_fails_signup() {
        setup(|conn, mut form| {
            form.password_confirmation = create_plaintext_password("not the password")?;

            let operation = SignUp(form);

            assert!(operation.db_action(conn).is_err());
            Ok(())
        });
    }

    #[test]
    fn duplicate_email_fails_signup() {
        with_connection(|conn| {
            let email = format!("{}@{}.{}", gen_string()?, gen_string()?, gen_string()?);
            let pass = gen_string()?;

            let form = ValidatedSignUpForm {
                email: email.clone(),
                password: create_plaintext_password(&pass)?,
                password_confirmation: create_plaintext_password(&pass)?,
            };

            let operation = SignUp(form);

            assert!(operation.db_action(conn).is_ok());

            let form = ValidatedSignUpForm {
                email,
                password: create_plaintext_password(&pass)?,
                password_confirmation: create_plaintext_password(&pass)?,
            };

            let operation = SignUp(form);

            assert!(operation.db_action(conn).is_err());
            Ok(())
        })
    }
}
