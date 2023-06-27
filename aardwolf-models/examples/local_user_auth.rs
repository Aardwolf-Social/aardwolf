extern crate aardwolf_models;
extern crate diesel;
extern crate dotenv;
extern crate env_logger;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::env;

use aardwolf_models::user::{
    email::{EmailVerificationToken, NewEmail},
    local_auth::{NewLocalAuth, PlaintextPassword},
    NewUser, UnauthenticatedUser, UserLike,
};
use diesel::{pg::PgConnection, prelude::*};
use dotenv::dotenv;

#[derive(Deserialize)]
struct Payload {
    email: String,
    password: PlaintextPassword,
    confirm_password: PlaintextPassword,
}

#[derive(Deserialize)]
struct AuthPayload {
    email: String,
    password: PlaintextPassword,
}

#[derive(Deserialize)]
struct VerificationPayload {
    id: i32,
    token: EmailVerificationToken,
}

fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("TEST_DATABASE_URL").unwrap();

    PgConnection::establish(&database_url).unwrap()
}

fn main() {
    env::set_var("RUST_LOG", "aardwolf_models=debug");
    env_logger::init();

    let mut connection = establish_connection();

    connection.test_transaction::<(), diesel::result::Error, _>(|conn| {
        // Create a user. Users are initially unverified
        let (token, email) = {
            let json = json!({
                "email": "test@example.com",
                "password": "testpass",
                "confirm_password": "testpass",
            });

            let payload: Payload = serde_json::from_value(json).unwrap();

            let user = match NewUser::new()
                .insert(conn)
                .unwrap()
                .into_verified(conn)
                .unwrap()
            {
                Ok(_) => panic!("Unexpected verified user"),
                Err(user) => user,
            };

            NewLocalAuth::new_from_two(&user, payload.password, payload.confirm_password)
                .unwrap()
                .insert(conn)
                .unwrap();

            let (new_email, token) = NewEmail::new(payload.email, &user).unwrap();

            let email = new_email.insert(conn).unwrap();

            println!("Created user, local_auth, and email!");
            (token, email)
        };

        // Log in the unverified user
        {
            let json = json!({
                "email": "test@example.com",
                "password": "testpass",
            });

            let payload: AuthPayload = serde_json::from_value(json).unwrap();

            let (user, _email, local_auth) =
                UnauthenticatedUser::by_email_for_auth(&payload.email, conn).unwrap();

            let user = user.log_in_local(local_auth, payload.password).unwrap();

            assert!(
                !user.is_verified(conn).unwrap(),
                "User shouldn't be verified at this point"
            );

            println!("Logged in unverified User!!!");
        }

        // Verify the user
        {
            let json = json!({
                "id": email.id(),
                "token":format!("{}", token)
            });

            let payload: VerificationPayload = serde_json::from_value(json).unwrap();

            let (unauthenticated_user, email) =
                UnauthenticatedUser::by_email_id(payload.id, conn).unwrap();

            let user = match unauthenticated_user.into_verified(conn).unwrap() {
                Ok(_unauthenticatec_user) => panic!("User shouldn't be verified"),
                Err(unverified_user) => unverified_user,
            };

            let email = match email.into_verified() {
                Ok(_verified_email) => panic!("Unverified user should not have a verified email"),
                Err(unverified_email) => unverified_email,
            };

            let (_user, _email) = user
                .verify(email, payload.token)
                .unwrap()
                .store_verify(conn)
                .unwrap();

            println!("Verified user!");
        }

        // log in the verified user
        {
            let json = json!({
                "email": "test@example.com",
                "password": "testpass",
            });

            let payload: AuthPayload = serde_json::from_value(json).unwrap();

            let (user, _email, local_auth) =
                UnauthenticatedUser::by_email_for_auth(&payload.email, conn).unwrap();

            let user = user.log_in_local(local_auth, payload.password).unwrap();

            assert!(
                user.is_verified(conn).unwrap(),
                "User should be verified at this point"
            );

            println!("Logged in verified User!!!");
        }

        Ok(())
    });

    println!("Hewwo, Mr Obama???");
}
