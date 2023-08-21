use std::env;

use chrono::{offset::Utc, DateTime, Duration as OldDuration};
use chrono_tz::Tz;
use diesel::{pg::PgConnection, Connection};
use dotenvy::dotenv;
use mime::TEXT_PLAIN;
use openssl::rsa::Rsa;
use rand::{distributions::Alphanumeric, rngs::OsRng, Rng};
use serde_json;
use thiserror::Error;
use url::Url as OrigUrl;
use uuid::Uuid;

use crate::{
    base_actor::{
        follow_request::{FollowRequest, NewFollowRequest},
        follower::{Follower, NewFollower},
        group::{
            group_base_actor::{GroupBaseActor, NewGroupBaseActor},
            Group, NewGroup,
        },
        persona::{NewPersona, Persona},
        BaseActor, NewBaseActor,
    },
    base_post::{
        direct_post::{DirectPost, NewDirectPost},
        post::{
            comment::{
                reaction::{NewReaction, Reaction},
                Comment, NewComment,
            },
            media_post::{MediaPost, NewMediaPost},
            NewPost, Post,
        },
        BasePost, NewBasePost,
    },
    file::{File, NewFile},
    generate_urls::GenerateUrls,
    sql_types::{FollowPolicy, PostVisibility, ReactionType, Url},
    timer::{
        event::{Event, NewEvent},
        event_notification::{EventNotification, NewEventNotification},
        NewTimer, Timer,
    },
    user::{
        email::{EmailToken, EmailVerificationToken, NewEmail, UnverifiedEmail, VerifiedEmail},
        local_auth::{LocalAuth, NewLocalAuth, PlaintextPassword},
        AuthenticatedUser, NewUser, UnauthenticatedUser, UnverifiedUser, UserLike,
    },
};

pub fn create_plaintext_password(pass: &str) -> Result<PlaintextPassword, anyhow::Error> {
    let v = serde_json::Value::String(pass.to_owned());
    let pass = serde_json::from_value(v)?;

    Ok(pass)
}

pub fn transmute_email_token(token: &EmailToken) -> Result<EmailVerificationToken, anyhow::Error> {
    let s = serde_json::to_string(token)?;
    let token = serde_json::from_str(&s)?;

    Ok(token)
}

pub fn gen_string() -> String {
    OsRng.sample_iter(&Alphanumeric).take(10).map(char::from).collect()
}

pub fn gen_url() -> Url {
    let mut url: OrigUrl = "https://example.com".parse().unwrap();

    url.set_path(&gen_string());

    Url(url)
}

pub fn gen_bool() -> bool {
    OsRng.gen()
}

pub fn gen_datetime() -> DateTime<Utc> {
    let hours = OsRng.gen_range(0..=10000);

    Utc::now()
        .checked_add_signed(OldDuration::hours(hours))
        .unwrap()
}

#[derive(Debug, Error)]
#[error("Error in time bounds")]
pub struct TimeBounds;

pub fn with_connection<F>(f: F)
where
    F: FnOnce(&mut PgConnection) -> Result<(), anyhow::Error>,
{
    dotenv().ok();

    let db_url = env::var("TEST_DATABASE_URL").unwrap();

    let mut conn = PgConnection::establish(&db_url).unwrap();

    conn.test_transaction(|conn| {
        f(conn).map_err(|e| {
            println!("Error: {}, {:?}", e, e);
            e
        })
    });
}

pub fn make_base_actor(conn: &mut PgConnection) -> Result<BaseActor, diesel::result::Error> {
    let (_pr, pu) = gen_keypair();

    NewBaseActor::new(
        gen_string(),
        gen_url(),
        gen_url(),
        gen_url(),
        FollowPolicy::AutoAccept,
        pu,
        gen_string(),
    )
    .insert(conn)
}

pub fn gen_keypair() -> (Vec<u8>, Vec<u8>) {
    let priv_key = Rsa::generate(2048).unwrap();

    (
        priv_key.private_key_to_der().unwrap(),
        priv_key.public_key_to_der_pkcs1().unwrap(),
    )
}

pub struct UrlGenerator;

impl GenerateUrls for UrlGenerator {
    fn activitypub_id(&self, uuid: &Uuid) -> String {
        uuid.to_string()
    }

    fn profile_url(&self, uuid: &Uuid) -> Url {
        format!("https://example.com/{}", uuid).parse().unwrap()
    }

    fn inbox_url(&self, uuid: &Uuid) -> Url {
        format!("https://example.com/{}", uuid).parse().unwrap()
    }

    fn outbox_url(&self, uuid: &Uuid) -> Url {
        format!("https://example.com/{}", uuid).parse().unwrap()
    }

    fn post_id(&self, _: &BaseActor, uuid: &Uuid) -> String {
        format!("https://example.com/{}", uuid)
    }

    fn post_url(&self, _: &BaseActor, uuid: &Uuid) -> Url {
        format!("https://example.com/{}", uuid).parse().unwrap()
    }
}

pub fn user_make_base_actor(
    conn: &mut PgConnection,
    user: &AuthenticatedUser,
) -> Result<BaseActor, diesel::result::Error> {
    let (pr, pu) = gen_keypair();

    NewBaseActor::local(
        gen_string(),
        user,
        FollowPolicy::AutoAccept,
        pr,
        pu,
        UrlGenerator,
    )
    .insert(conn)
}

pub fn make_group(
    conn: &mut PgConnection,
    base_actor: &BaseActor,
) -> Result<Group, diesel::result::Error> {
    NewGroup::new(base_actor).insert(conn)
}

pub fn make_group_base_actor(
    conn: &mut PgConnection,
    group: &Group,
    base_actor: &BaseActor,
) -> Result<GroupBaseActor, diesel::result::Error> {
    NewGroupBaseActor::new(group, base_actor).insert(conn)
}

pub fn make_follow_request(
    conn: &mut PgConnection,
    follower: &BaseActor,
    requested_follow: &BaseActor,
) -> Result<FollowRequest, diesel::result::Error> {
    NewFollowRequest::new(follower, requested_follow).insert(conn)
}

pub fn make_follower(
    conn: &mut PgConnection,
    follower: &BaseActor,
    follows: &BaseActor,
) -> Result<Follower, diesel::result::Error> {
    NewFollower::new(follower, follows).insert(conn)
}

pub fn make_persona(
    conn: &mut PgConnection,
    base_actor: &BaseActor,
) -> Result<Persona, diesel::result::Error> {
    NewPersona::new(
        PostVisibility::Public,
        gen_bool(),
        None,
        gen_string(),
        base_actor,
    )
    .insert(conn)
}

pub fn make_base_post(
    conn: &mut PgConnection,
    posted_by: &BaseActor,
) -> Result<BasePost, diesel::result::Error> {
    NewBasePost::local(
        None,
        TEXT_PLAIN.into(),
        posted_by,
        None,
        PostVisibility::Public,
        UrlGenerator,
    )
    .insert(conn)
}

pub fn make_post_with(
    conn: &mut PgConnection,
    base_post: &BasePost,
) -> Result<Post, diesel::result::Error> {
    NewPost::new(gen_string(), Some(gen_string()), &base_post).insert(conn)
}

pub fn make_post(conn: &mut PgConnection) -> Result<Post, diesel::result::Error> {
    let base_actor = make_base_actor(conn)?;
    let base_post = make_base_post(conn, &base_actor)?;

    make_post_with(conn, &base_post)
}

pub fn make_comment(
    conn: &mut PgConnection,
    conversation: &Post,
    parent: &Post,
    post: &Post,
) -> Result<Comment, diesel::result::Error> {
    NewComment::new(conversation, parent, post).insert(conn)
}

pub fn make_reaction(
    conn: &mut PgConnection,
    comment: &Comment,
) -> Result<Reaction, diesel::result::Error> {
    NewReaction::new(ReactionType::Like, comment).insert(conn)
}

pub fn make_file(conn: &mut PgConnection) -> Result<File, diesel::result::Error> {
    NewFile::new("Cargo.toml").unwrap().insert(conn)
}

pub fn make_media_post(
    conn: &mut PgConnection,
    file: &File,
    post: &Post,
) -> Result<MediaPost, diesel::result::Error> {
    NewMediaPost::new(file, post).insert(conn)
}

pub fn make_direct_post(
    conn: &mut PgConnection,
    base_post: &BasePost,
    base_actor: &BaseActor,
) -> Result<DirectPost, diesel::result::Error> {
    NewDirectPost::new(base_post, base_actor).insert(conn)
}

pub fn make_timer(conn: &mut PgConnection) -> Result<Timer, diesel::result::Error> {
    NewTimer::new(gen_datetime()).insert(conn)
}

pub fn make_event(
    conn: &mut PgConnection,
    owner: &Persona,
    start: &Timer,
    end: &Timer,
) -> Result<Event, diesel::result::Error> {
    NewEvent::new(owner, start, end, Tz::UTC, gen_string(), gen_string())
        .unwrap()
        .insert(conn)
}

pub fn make_event_notification(
    conn: &mut PgConnection,
    event: &Event,
    timer: &Timer,
) -> Result<EventNotification, diesel::result::Error> {
    NewEventNotification::new(event, timer).insert(conn)
}

pub fn make_unverified_user(
    conn: &mut PgConnection,
) -> Result<UnverifiedUser, diesel::result::Error> {
    let unauthenticated_user = NewUser::new().insert(conn)?;

    match unauthenticated_user.into_verified(conn)? {
        Ok(_) => panic!("User is already verified"),
        Err(unverified_user) => Ok(unverified_user),
    }
}

pub fn make_unverified_email<U>(
    conn: &mut PgConnection,
    user: &U,
) -> Result<(UnverifiedEmail, EmailToken), diesel::result::Error>
where
    U: UserLike,
{
    let (email, token) = NewEmail::new(gen_string(), user).unwrap();

    let email = email.insert(conn)?;

    Ok((email, token))
}

pub fn make_local_auth(
    conn: &mut PgConnection,
    user: &UnverifiedUser,
    password: &str,
) -> Result<LocalAuth, diesel::result::Error> {
    let password = create_plaintext_password(password).unwrap();

    NewLocalAuth::new(user, password).unwrap().insert(conn)
}

pub fn make_verified_authenticated_user(
    conn: &mut PgConnection,
    password: &str,
) -> Result<(AuthenticatedUser, VerifiedEmail), anyhow::Error> {
    let unauthenticated_user = NewUser::new().insert(conn)?;

    let user = match unauthenticated_user.into_verified(conn)? {
        Ok(_) => panic!("User is already verified"),
        Err(unverified_user) => unverified_user,
    };

    let password = create_plaintext_password(password)?;
    NewLocalAuth::new(&user, password)?.insert(conn)?;

    let (email, token) = NewEmail::new(gen_string(), &user)?;
    let email = email.insert(conn)?;
    let token = transmute_email_token(&token)?;

    let (user, email) = user.verify(email, token)?.store_verify(conn)?;

    Ok((user, email))
}

#[derive(Debug, Error)]
#[error("User is already verified")]
pub struct AlreadyVerified;

pub fn make_unverified_authenticated_user(
    conn: &mut PgConnection,
    password: &str,
) -> Result<AuthenticatedUser, anyhow::Error> {
    let user = make_unverified_user(conn)?;
    let _ = make_unverified_email(conn, &user)?;
    let auth = make_local_auth(conn, &user, password)?;
    let user = UnauthenticatedUser::by_id(user.id(), conn)?;

    let user = user
        .log_in_local(auth, create_plaintext_password(password).unwrap())
        .unwrap();

    Ok(user)
}

pub fn make_verified_user_make_persona(
    conn: &mut PgConnection,
    password: &str,
) -> Result<(AuthenticatedUser, BaseActor, Persona), diesel::result::Error> {
    let (user, _email) = make_verified_authenticated_user(conn, password).unwrap();
    let base_actor = user_make_base_actor(conn, &user)?;
    let persona = make_persona(conn, &base_actor)?;

    Ok((user.clone(), base_actor.clone(), persona))
}
