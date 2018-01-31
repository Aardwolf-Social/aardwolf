use bs58;
use bcrypt::{DEFAULT_COST, hash, verify};
use ring::rand::SecureRandom;
use diesel;
use diesel::LoadDsl;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use chrono::Utc;

use forms::auth::{SignInForm, SignUpForm, SignUpFormValidationFail, SignInFormValidationFail};
use forms::traits::Validate;
use models::account::{Account, NewAccount};
use models::user::{NewUser, User};
use schema::aardwolf::{accounts, users};

#[derive(Fail, Debug)]
pub(crate) enum SignUpFail {
    #[fail(display = "{}", error)]
    ValidationError {
        error: SignUpFormValidationFail,
    },
    #[fail(display = "Failed to insert account into database")]
    AccountCreateError,
    #[fail(display = "Failed to insert user into database")]
    UserCreateError,
    #[fail(display = "Failed to hash password")]
    PasswordHashError,
    #[fail(display = "Failed to create confirmation token")]
    CreateTokenError,
}

pub(crate) fn create_user_and_account<T: SecureRandom>(form: SignUpForm, gen: &T, db: &PgConnection) -> Result<(), SignUpFail> {
    // validation goes here
    if let Err(validation_error) = form.validate() {
        return Err(SignUpFail::ValidationError { error: validation_error })
    }
    let account = NewAccount {
        username: form.username,
    };
    let account: Account = match diesel::insert_into(accounts::table).values(&account).get_result(db) {
        Ok(account) => account,
        Err(_) => return Err(SignUpFail::AccountCreateError),
    };

    let mut token: Vec<u8> = vec![0; 16];
    generate_token(gen, &mut token)?;
    let strtoken = bs58::encode(&token).into_string();

    let now = Utc::now().naive_utc();

    let hashed_password = hash_password(&form.password)?;
    let user = NewUser {
        encrypted_password: hashed_password,
        account_id: account.id,
        unconfirmed_email: form.email,
        confirmation_token: token,
        confirmation_sent_at: now,
    };

    if let Err(_) = diesel::insert_into(users::table).values(&user).get_result::<User>(db) {
        return Err(SignUpFail::UserCreateError);
    };

    // just printing this out for now so we can copy/paste into the browser to confirm accounts,
    // but obviously this will need to be emailed to the user
    println!("confirmation token url: /auth/confirmation?token={}", &strtoken);

    Ok(())
}

#[derive(Debug, Fail)]
pub(crate) enum ConfirmAccountFail {
    #[fail(display = "token was not found")]
    TokenNotFound,
    #[fail(display = "failed to decode confirmation token")]
    Base58DecodeError,
    #[fail(display = "failed to update the user record")]
    UpdateFail,
}

pub(crate) fn confirm_account(token: &str, db: &PgConnection) -> Result<User, ConfirmAccountFail> {
    use schema::aardwolf::users::dsl::*;

    let token = match bs58::decode(token).into_vec() {
        Ok(t) => t,
        Err(_) => return Err(ConfirmAccountFail::Base58DecodeError),
    };
    let mut user = match users.filter(confirmation_token.eq(token)).first::<User>(db) {
        Ok(user) => user,
        Err(e) => {
            println!("Err: {:#?}", e);
            return Err(ConfirmAccountFail::TokenNotFound);
        }
    };

    let user = match user.confirm().save_changes::<User>(db) {
        Ok(user) => user,
        Err(e) => {
            println!("Err: {:#?}", e);
            return Err(ConfirmAccountFail::UpdateFail);
        },
    };
    Ok(user)
}

fn hash_password(password: &str) -> Result<String, SignUpFail> {
    Ok(match hash(password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return Err(SignUpFail::PasswordHashError),
    })
}

fn check_password(password: &str, hash: &str) -> Result<(), SignInFail> {
    if let Err(_) = verify(password, hash) {
        return Err(SignInFail::GenericLoginError);
    }
    Ok(())
}

fn generate_token<T: SecureRandom>(gen: &T, buffer: &mut [u8]) -> Result<(), SignUpFail> {
    if let Err(_) = gen.fill(buffer) {
        return Err(SignUpFail::CreateTokenError);
    }
    Ok(())
}

#[derive(Fail, Debug)]
pub(crate) enum SignInFail {
    #[fail(display = "{}", error)]
    ValidationError {
        error: SignInFormValidationFail
    },
    // this is the generic "login failed" error the user will see
    #[fail(display = "Invalid username or password")]
    GenericLoginError
}

pub(crate) fn sign_in(form: &SignInForm, db: &PgConnection) -> Result<User, SignInFail> {
    use schema::aardwolf::users::dsl::*;

    if let Err(validation_error) = form.validate() {
        return Err(SignInFail::ValidationError { error: validation_error });
    }

    // check csrf token

    let user = match users.filter(email.eq(&form.email)).first::<User>(db) {
        Ok(user) => user,
        Err(_) => {
            println!("user {} not found", &form.email);
            return Err(SignInFail::GenericLoginError);
        },
    };

    check_password(&form.password, &user.encrypted_password)?;

    Ok(user)
}
