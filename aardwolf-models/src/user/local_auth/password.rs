#![allow(proc_macro_derive_resolution_fallback)]
use std::{fmt, io::Write};

use bcrypt::{hash, verify};
use diesel::{backend::Backend, deserialize, serialize, sql_types::Text};
use serde::de::{Deserialize, Deserializer};

/// Create a trait used to verify passwords.
///
/// This trait exists to ensure passwords can only be verified if this trait is in scope. In the
/// majority of cases, this trait will not be in scope.
pub(crate) trait Verify {
    fn verify(&self, _: PlaintextPassword) -> Result<(), VerificationError>;
}

/// Create a trait used to create passwords.
///
/// This trait exists to ensure passwords can only be created if this trait is in scope. In the
/// majority of cases, this trait will not be in scope.
pub(crate) trait Create: Sized {
    fn create(_: PlaintextPassword) -> Result<Self, CreationError>;
}

/// Create a trait used to validate passwords.
///
/// This trait exists to ensure passwords can only be validated if this trait is in scope. In the
/// majority of cases, this trait will not be in scope.
pub(crate) trait Validate: Sized {
    /// Verify that the password is valid by comparing it against another password.
    ///
    /// This *must* be used in cases where a user is given the option to type and retype a
    /// password. This *must not* be used for any other case.
    ///
    /// On a succesfull compare, one of the two passwords *must* be returned. Since they *must*
    /// be the same, it does not matter which is returned.
    ///
    /// On a failed compare, a `ValidationError` *must* be returned.
    fn compare(self, _: Self) -> Result<Self, ValidationError>;

    /// Verify that the password is valid by performing checks on the inner string.
    ///
    /// On a succesfull validation, the password *must* be returned.
    ///
    /// On a failed validation, a `ValidationError` *must* be returned.
    fn validate(self) -> Result<Self, ValidationError>;
}

/// The error used when verifying a password fails.
///
/// A password verification can fail if any step leading to the verification fails, or if the
/// password itself cannot be verified with the given `PlaintextPassword`
#[derive(Clone, Copy, Debug, Eq, Fail, PartialEq)]
pub enum VerificationError {
    /// The password could not be checked because something failed before that step
    #[fail(display = "Error validating password")]
    Process,
    /// The password was checked and was found to be invalid
    #[fail(display = "Invalid password")]
    Password,
}

/// The error used when creating a password fails.
#[derive(Clone, Copy, Debug, Eq, Fail, PartialEq)]
pub enum CreationError {
    /// This happens when a password does not meet the requirements to be considered usable.
    /// Currently, this means the password is too short, or two submitted passwords do not match.
    #[fail(display = "Error validating password")]
    Validation(#[cause] ValidationError),
    /// This should only happen in very rare circumstances, since generally bcrypt is good about not
    /// having errors.
    #[fail(display = "Error creating password")]
    Bcrypt,
}

impl From<ValidationError> for CreationError {
    fn from(e: ValidationError) -> Self {
        CreationError::Validation(e)
    }
}

/// The error used when validating passwords
///
/// Since there are many errors that can occur when validating a password, and typically we want to
/// show all errors to the user when they are creating a password, this is implemented as a series
/// of booleans for the different kinds of errors.
#[derive(Clone, Copy, Debug, Eq, Fail, PartialEq)]
#[fail(display = "Password is invalid")]
pub struct ValidationError {
    no_match: bool,
    too_short: bool,
}

impl ValidationError {
    /// Was there any error in password validation?
    pub fn any(self) -> bool {
        self.no_match || self.too_short
    }

    /// Passwords do not match
    pub fn no_match(self) -> bool {
        self.no_match
    }

    /// Password is too short
    pub fn too_short(self) -> bool {
        self.too_short
    }

    /// Merges multiple validation errors into a single validation error
    pub fn merge(&mut self, rhs: Self) {
        self.no_match = self.no_match || rhs.no_match;
        self.too_short = self.too_short || rhs.too_short;
    }
}

/// Define a `PlaintextPassword`.
///
/// This type cannot be created by normal means, and the contents cannot be read. It can only be
/// deserialized from some serialized data, and its only use is to create or verify a `Password`.
///
/// This type does not implement Clone, and when it is used to create or verify a password, it is
/// consumed. This ensures that passwords in our application are not misused.
///
/// Debug and Display are both implemented for PlaintextPassword, but they simply print eight
/// asterisks.
pub struct PlaintextPassword(String);

impl PlaintextPassword {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<'de> Deserialize<'de> for PlaintextPassword {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(PlaintextPassword(String::deserialize(deserializer)?))
    }
}

#[cfg(feature = "rocket")]
mod rocket {
    use std::str::Utf8Error;

    use rocket::{http::RawStr, request::FromFormValue};

    use super::PlaintextPassword;

    impl<'v> FromFormValue<'v> for PlaintextPassword {
        type Error = Utf8Error;

        fn from_form_value(form_value: &'v RawStr) -> Result<Self, Self::Error> {
            Ok(PlaintextPassword(form_value.url_decode()?))
        }
    }
}

impl fmt::Debug for PlaintextPassword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "********")
    }
}

impl fmt::Display for PlaintextPassword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "********")
    }
}

impl Validate for PlaintextPassword {
    fn compare(self, rhs: Self) -> Result<Self, ValidationError> {
        if self.0 == rhs.0 {
            Ok(self)
        } else {
            Err(ValidationError {
                no_match: true,
                too_short: false,
            })
        }
    }

    fn validate(self) -> Result<Self, ValidationError> {
        let mut validation_error = ValidationError {
            no_match: false,
            too_short: false,
        };

        if self.0.len() < 8 {
            validation_error.too_short = true;
        }

        if validation_error.any() {
            Err(validation_error)
        } else {
            Ok(self)
        }
    }
}

/// Define a `Password`.
///
/// This type can only be created through the `Create` trait's `create` method. It can be
/// serialized, but only through its `ToSql` method for use with Diesel. Because of this limitation,
/// it is very important that no stray `to_sql` methods are used in applications consuming this
/// libary.
///
/// The only use this type has is to be "verified" via the `Verify` trait. Once a password is
/// "verified", a user can be considered "logged in".
///
/// Debug and Display are both implemented for Password, but they simply print eight asterisks.
#[derive(AsExpression, FromSqlRow)]
#[sql_type = "Text"]
pub struct Password(String);

impl<DB> serialize::ToSql<Text, DB> for Password
where
    DB: Backend,
{
    fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, DB>) -> serialize::Result {
        serialize::ToSql::<Text, DB>::to_sql(&self.0, out)
    }
}

impl<DB> deserialize::FromSql<Text, DB> for Password
where
    DB: Backend<RawValue = [u8]>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        deserialize::FromSql::<Text, DB>::from_sql(bytes).map(Password)
    }
}

impl fmt::Debug for Password {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "********")
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "********")
    }
}

impl Verify for Password {
    fn verify(&self, given_password: PlaintextPassword) -> Result<(), VerificationError> {
        verify(&given_password.0, &self.0)
            .map_err(|e| {
                error!("Error verifying password: {}", e);

                VerificationError::Process
            })
            .and_then(|verified| {
                if verified {
                    Ok(())
                } else {
                    Err(VerificationError::Password)
                }
            })
    }
}

impl Create for Password {
    fn create(password: PlaintextPassword) -> Result<Password, CreationError> {
        #[cfg(any(test, feature = "test"))]
        warn!("BUILT IN TEST MODE");

        #[cfg(not(any(test, feature = "test")))]
        let h = hash(&password.0, bcrypt::DEFAULT_COST);
        #[cfg(any(test, feature = "test"))]
        let h = hash(&password.0, 4);

        h.map_err(|e| {
            error!("Error creating password: {}", e);

            CreationError::Bcrypt
        })
        .map(Password)
    }
}

#[cfg(test)]
mod tests {
    use super::{Create, Password, Validate, Verify};
    use crate::test_helper::create_plaintext_password;

    #[test]
    fn create_and_verify_password() {
        let pass = "testpass";
        let password = create_plaintext_password(pass).unwrap();

        let hashed_password = Password::create(password);
        assert!(
            hashed_password.is_ok(),
            "Failed to create password from PlaintextPassword"
        );
        let hashed_password = hashed_password.unwrap();

        let password = create_plaintext_password(pass).unwrap();

        assert!(
            hashed_password.verify(password).is_ok(),
            "Failed to verify password"
        );
    }

    #[test]
    fn dont_verify_bad_password() {
        let password = create_plaintext_password("testpass").unwrap();

        let hashed_password = Password::create(password);
        assert!(
            hashed_password.is_ok(),
            "Failed to create password from PlaintextPassword"
        );
        let hashed_password = hashed_password.unwrap();

        let password = create_plaintext_password("not the same password").unwrap();

        assert!(
            hashed_password.verify(password).is_err(),
            "Should not have verified invalid password"
        );
    }

    #[test]
    fn validate_long_password() {
        let password = create_plaintext_password("testpass").unwrap();

        assert!(
            password.validate().is_ok(),
            "Password should have passed validation"
        );
    }

    #[test]
    fn dont_validate_short_password() {
        let password = create_plaintext_password("short").unwrap();

        assert!(
            password.validate().is_err(),
            "Password should have passed validation"
        );
    }

    #[test]
    fn validate_same_password() {
        let pass = "testpass";
        let pass1 = create_plaintext_password(pass).unwrap();
        let pass2 = create_plaintext_password(pass).unwrap();

        assert!(
            pass1.compare(pass2).is_ok(),
            "Identical passwords should pass validation"
        );
    }

    #[test]
    fn dont_validate_different_password() {
        let pass1 = create_plaintext_password("testpass").unwrap();
        let pass2 = create_plaintext_password("not the same password").unwrap();

        assert!(
            pass1.compare(pass2).is_err(),
            "Different passwords should not pass validation"
        );
    }
}
