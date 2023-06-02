use std::{fmt, io::Write};

use bcrypt::{hash, verify};
use diesel::{backend::Backend, deserialize, serialize, sql_types::Text};
use failure::Fail;
#[cfg(any(test, feature = "test"))]
use log::warn;
use rand::{distributions::Alphanumeric, rngs::OsRng, Rng};
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

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
    let token = OsRng
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join("");

    #[cfg(any(test, feature = "test"))]
    warn!("BUILT IN TEST MODE");

    #[cfg(not(any(test, feature = "test")))]
    let h = hash(&token, bcrypt::DEFAULT_COST);
    #[cfg(any(test, feature = "test"))]
    let h = hash(&token, 4);

    let hashed_token = h.map_err(|_| CreationError::Hash)?;

    Ok((EmailToken(token), HashedEmailToken(hashed_token)))
}

#[derive(AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
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

pub struct EmailToken(String);

impl Serialize for EmailToken {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        String::serialize(&self.0, serializer)
    }
}

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

pub struct EmailVerificationToken(String);

impl<'de> Deserialize<'de> for EmailVerificationToken {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(EmailVerificationToken(String::deserialize(deserializer)?))
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

#[cfg(test)]
mod tests {
    use super::{create_token, EmailVerificationToken, VerifyEmail};
    use crate::test_helper::transmute_email_token;

    #[test]
    fn create_and_verify_token() {
        let (email_token, hashed_token) = create_token().unwrap();
        let verification_token = transmute_email_token(&email_token).unwrap();

        assert!(
            hashed_token.verify_email(verification_token).is_ok(),
            "Should have verified email with correct token"
        );
    }

    #[test]
    fn dont_verify_invalid_token() {
        let (_email_token, hashed_token) = create_token().unwrap();
        let fake_token = EmailVerificationToken("fake token".to_owned());

        assert!(
            hashed_token.verify_email(fake_token).is_err(),
            "Should not have verified invalid token"
        );
    }
}
