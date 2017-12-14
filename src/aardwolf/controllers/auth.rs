use base64;
use bcrypt::{DEFAULT_COST, hash, verify};
use ring::rand::SecureRandom;
use diesel;
use diesel::LoadDsl;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use failure::Error;
use chrono::Utc;

use Pool;
use forms::auth::{SignInForm, SignUpForm};
use models::account::{Account, NewAccount};
use models::user::{NewUser, User};
use schema::aardwolf::{accounts, users};

#[derive(Fail, Debug)]
pub(crate) enum SignUpFail {
    #[fail(display = "Passwords do not match")]
    PasswordMatchError,
    #[fail(display = "Failed to insert account into database")]
    AccountCreateError,
    #[fail(display = "Failed to insert user into database")]
    UserCreateError,
    #[fail(display = "Failed to hash password")]
    PasswordHashError,
}

pub(crate) fn create_user_and_account<T: SecureRandom>(form: SignUpForm, gen: &T, db: &PgConnection) -> Result<(), SignUpFail> {
    // validation goes here
    if form.password != form.password_confirmation {
        return Err(SignUpFail::PasswordMatchError);
    }
    let account = NewAccount {
        username: form.username,
    };
    let account: Account = match diesel::insert(&account).into(accounts::table).get_result(db) {
        Ok(account) => account,
        Err(e) => return Err(SignUpFail::AccountCreateError),
    };

    let mut token: Vec<u8> = vec![0; 16];
    generate_token(gen, &mut token)?;
    let strtoken = base64::encode(&token);

    let now = Utc::now().naive_utc();

    let hashed_password = hash_password(&form.password)?;
    let user = NewUser {
        encrypted_password: hashed_password,
        account_id: account.id,
        unconfirmed_email: form.email,
        confirmation_token: token,
        confirmation_sent_at: now,
    };

    let user: User = match diesel::insert(&user).into(users::table).get_result(db) {
        Ok(user) => user,
        Err(e) => return Err(SignUpFail::UserCreateError),
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
    Base64DecodeError,
    #[fail(display = "failed to update the user record")]
    UpdateFail,
}

pub(crate) fn confirm_account(token: &str, db: &PgConnection) -> Result<User, ConfirmAccountFail> {
    use schema::aardwolf::users::dsl::*;

    let token = match base64::decode(token) {
        Ok(t) => t,
        Err(_) => return Err(ConfirmAccountFail::Base64DecodeError),
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
        Err(e) => return Err(SignUpFail::PasswordHashError),
    })
}

fn check_password(password: &str, hash: &str) -> Result<(), SignInFail> {
    if let Err(e) = verify(password, hash) {
        return Err(SignInFail::WrongPassword);
    }
    Ok(())
}

fn generate_token<T: SecureRandom>(gen: &T, buffer: &mut [u8]) -> Result<(), SignUpFail> {
    gen.fill(buffer);
    Ok(())
}

#[derive(Fail, Debug)]
pub(crate) enum SignInFail {
    #[fail(display = "user not found")]
    UserNotFound,
    #[fail(display = "password is incorrect")]
    WrongPassword,
}

pub(crate) fn sign_in(form: &SignInForm, db: &PgConnection) -> Result<User, SignInFail> {
    use schema::aardwolf::users::dsl::*;

    // check csrf token

    let user = match users.filter(email.eq(&form.email)).first::<User>(db) {
        Ok(user) => user,
        Err(e) => {
            println!("user {} not found", &form.email);
            return Err(SignInFail::UserNotFound);
        },
    };

    check_password(&form.password, &user.encrypted_password)?;

    Ok(user)
}
