use std::io::Write;
use std::fmt;
use std::str::Utf8Error;

use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::backend::Backend;
use diesel::deserialize;
use diesel::serialize;
use diesel::sql_types::Text;
use rand::{OsRng, Rng};
use rocket::http::RawStr;
use rocket::request::FromFormValue;

/// A trait used to verify emails
///
/// Emails should only be able to be verified in certain situations, so this trait must not be in
/// scope unless it should be possible to verify an email
pub trait VerifyEmail {
    fn verify_email(&self, token: EmailVerificationToken) -> Result<(), VerificationError>;
}

#[derive(Clone, Copy, Debug, Eq, Fail, PartialEq)]
pub enum CreationError {
    #[fail(display = "Failed to create Random Number Generator")]
    Rng,
    #[fail(display = "Failed to hash generated token")]
    Hash,
}

#[derive(Clone, Copy, Debug, Eq, Fail, PartialEq)]
pub enum VerificationError {
    #[fail(display = "Failed to verify token")]
    Process,
    #[fail(display = "Token was invaid")]
    Token,
}

/// A function used to create email tokens.
///
/// Email tokens should only be able to be created in certain situations, so this function must not
/// be in scope unless it should be possible to verify an email
pub fn create_token() -> Result<(EmailToken, HashedEmailToken), CreationError> {
    let mut rng = OsRng::new().map_err(|_| CreationError::Rng)?;

    let token = rng.gen_ascii_chars()
        .take(32)
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join("");

    let hashed_token = hash(&token, DEFAULT_COST).map_err(|_| CreationError::Hash)?;

    Ok((EmailToken(token), HashedEmailToken(hashed_token)))
}

#[derive(AsExpression, FromSqlRow)]
#[sql_type = "Text"]
pub struct HashedEmailToken(String);

impl fmt::Debug for HashedEmailToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "********")
    }
}

impl fmt::Display for HashedEmailToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "********")
    }
}

impl<DB> serialize::ToSql<Text, DB> for HashedEmailToken
where
    DB: Backend,
{
    fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, DB>) -> serialize::Result {
        serialize::ToSql::<Text, DB>::to_sql(&self.0, out)
    }
}

impl<DB> deserialize::FromSql<Text, DB> for HashedEmailToken
where
    DB: Backend<RawValue = [u8]>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        deserialize::FromSql::<Text, DB>::from_sql(bytes).map(HashedEmailToken)
    }
}

impl VerifyEmail for HashedEmailToken {
    fn verify_email(
        &self,
        email_verification_token: EmailVerificationToken,
    ) -> Result<(), VerificationError> {
        verify(&email_verification_token.0, &self.0)
            .map_err(|_| VerificationError::Process)
            .and_then(|verified| {
                if verified {
                    Ok(())
                } else {
                    Err(VerificationError::Token)
                }
            })
    }
}

#[derive(Serialize)]
pub struct EmailToken(String);

impl fmt::Debug for EmailToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for EmailToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Deserialize)]
pub struct EmailVerificationToken(String);

impl<'v> FromFormValue<'v> for EmailVerificationToken {
    type Error = Utf8Error;

    fn from_form_value(form_value: &'v RawStr) -> Result<Self, Self::Error> {
        Ok(EmailVerificationToken(form_value.url_decode()?))
    }
}

impl fmt::Debug for EmailVerificationToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "********")
    }
}

impl fmt::Display for EmailVerificationToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "********")
    }
}
