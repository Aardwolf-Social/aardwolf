use std::env;

use chrono::{offset::Utc, DateTime, Duration as OldDuration};
use chrono_tz::Tz;
use diesel::{pg::PgConnection, Connection};
use dotenv::dotenv;
use failure::{Error, Fail};
use mime::TEXT_PLAIN;
use openssl::rsa::Rsa;
use rand::{distributions::Alphanumeric, rngs::OsRng, Rng};
use serde_json;
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
        BaseActor, GenerateUrls, NewBaseActor,
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

pub type GenericError = Error;

pub fn create_plaintext_password(pass: &str) -> Result<PlaintextPassword, GenericError> {
    let v = serde_json::Value::String(pass.to_owned());
    let pass = serde_json::from_value(v)?;

    Ok(pass)
}

pub fn transmute_email_token(token: &EmailToken) -> Result<EmailVerificationToken, GenericError> {
    let s = serde_json::to_string(token)?;
    let token = serde_json::from_str(&s)?;

    Ok(token)
}

pub fn gen_string() -> Result<String, GenericError> {
    let mut rng = OsRng::new()?;

    Ok(rng.sample_iter(&Alphanumeric).take(10).collect())
}

pub fn gen_url() -> Result<Url, GenericError> {
    let mut url: OrigUrl = "https://example.com".parse()?;

    url.set_path(&gen_string()?);

    Ok(Url(url))
}

pub fn gen_bool() -> Result<bool, GenericError> {
    Ok(OsRng::new()?.gen())
}

pub fn gen_datetime() -> Result<DateTime<Utc>, GenericError> {
    let hours = OsRng::new()?.gen_range(0, 10000);

    Ok(Utc::now()
        .checked_add_signed(OldDuration::hours(hours))
        .ok_or(TimeBounds)?)
}

#[derive(Debug, Fail)]
#[fail(display = "Error in time bounds")]
pub struct TimeBounds;

pub fn with_connection<F>(f: F)
where
    F: FnOnce(&PgConnection) -> Result<(), GenericError>,
{
    dotenv().ok();

    let db_url = env::var("TEST_DATABASE_URL").unwrap();

    let conn = PgConnection::establish(&db_url).unwrap();

    conn.test_transaction(|| {
        f(&conn).map_err(|e| {
            println!("Error: {}, {:?}", e, e);
            e
        })
    });
}

pub fn with_base_actor<F>(conn: &PgConnection, f: F) -> Result<(), GenericError>
where
    F: FnOnce(BaseActor) -> Result<(), GenericError>,
{
    let (pr, pu) = gen_keypair()?;

    let base_actor = NewBaseActor::new(
        gen_string()?,
        gen_url()?,
        gen_url()?,
        gen_url()?,
        FollowPolicy::AutoAccept,
        pr,
        pu,
        gen_string()?,
    )
    .insert(conn)?;

    f(base_actor)
}

pub fn gen_keypair() -> Result<(Vec<u8>, Vec<u8>), GenericError> {
    let priv_key = Rsa::generate(2048)?;

    Ok((
        priv_key.private_key_to_der()?,
        priv_key.public_key_to_der_pkcs1()?,
    ))
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
}

pub fn user_with_base_actor<F>(
    conn: &PgConnection,
    user: &AuthenticatedUser,
    f: F,
) -> Result<(), GenericError>
where
    F: FnOnce(BaseActor) -> Result<(), GenericError>,
{
    let (pr, pu) = gen_keypair()?;

    let base_actor = NewBaseActor::local(
        gen_string()?,
        user,
        FollowPolicy::AutoAccept,
        pr,
        pu,
        UrlGenerator,
    )
    .insert(conn)?;

    f(base_actor)
}

pub fn with_group<F>(conn: &PgConnection, base_actor: &BaseActor, f: F) -> Result<(), GenericError>
where
    F: FnOnce(Group) -> Result<(), GenericError>,
{
    let group = NewGroup::new(base_actor).insert(conn)?;

    f(group)
}

pub fn with_group_base_actor<F>(
    conn: &PgConnection,
    group: &Group,
    base_actor: &BaseActor,
    f: F,
) -> Result<(), GenericError>
where
    F: FnOnce(GroupBaseActor) -> Result<(), GenericError>,
{
    let group_base_actor = NewGroupBaseActor::new(group, base_actor).insert(conn)?;

    f(group_base_actor)
}

pub fn with_follow_request<F>(
    conn: &PgConnection,
    follower: &BaseActor,
    requested_follow: &BaseActor,
    f: F,
) -> Result<(), GenericError>
where
    F: FnOnce(FollowRequest) -> Result<(), GenericError>,
{
    let follow_request = NewFollowRequest::new(follower, requested_follow).insert(conn)?;

    f(follow_request)
}

pub fn with_follower<F>(
    conn: &PgConnection,
    follower: &BaseActor,
    follows: &BaseActor,
    f: F,
) -> Result<(), GenericError>
where
    F: FnOnce(Follower) -> Result<(), GenericError>,
{
    let follower = NewFollower::new(follower, follows).insert(conn)?;

    f(follower)
}

pub fn with_persona<F>(
    conn: &PgConnection,
    base_actor: &BaseActor,
    f: F,
) -> Result<(), GenericError>
where
    F: FnOnce(Persona) -> Result<(), GenericError>,
{
    let persona = NewPersona::new(
        PostVisibility::Public,
        gen_bool()?,
        None,
        gen_string()?,
        base_actor,
    )
    .insert(conn)?;

    f(persona)
}

pub fn with_base_post<F>(
    conn: &PgConnection,
    posted_by: &BaseActor,
    f: F,
) -> Result<(), GenericError>
where
    F: FnOnce(BasePost) -> Result<(), GenericError>,
{
    let base_post = NewBasePost::local(
        None,
        TEXT_PLAIN,
        posted_by,
        None,
        PostVisibility::Public,
        |uuid| format!("{}", uuid),
    )
    .insert(conn)?;

    f(base_post)
}

pub fn with_post<F>(conn: &PgConnection, base_post: &BasePost, f: F) -> Result<(), GenericError>
where
    F: FnOnce(Post) -> Result<(), GenericError>,
{
    let post = NewPost::new(gen_string()?, Some(gen_string()?), base_post).insert(conn)?;

    f(post)
}

pub fn make_post<F>(conn: &PgConnection, f: F) -> Result<(), GenericError>
where
    F: FnOnce(Post) -> Result<(), GenericError>,
{
    with_base_actor(conn, |base_actor| {
        with_base_post(conn, &base_actor, |base_post| {
            with_post(conn, &base_post, f)
        })
    })
}

pub fn with_comment<F>(
    conn: &PgConnection,
    conversation: &Post,
    parent: &Post,
    post: &Post,
    f: F,
) -> Result<(), GenericError>
where
    F: FnOnce(Comment) -> Result<(), GenericError>,
{
    let comment = NewComment::new(conversation, parent, post).insert(conn)?;

    f(comment)
}

pub fn with_reaction<F>(conn: &PgConnection, comment: &Comment, f: F) -> Result<(), GenericError>
where
    F: FnOnce(Reaction) -> Result<(), GenericError>,
{
    let reaction = NewReaction::new(ReactionType::Like, comment).insert(conn)?;

    f(reaction)
}

pub fn with_file<F>(conn: &PgConnection, f: F) -> Result<(), GenericError>
where
    F: FnOnce(File) -> Result<(), GenericError>,
{
    let file = NewFile::new("Cargo.toml")?.insert(conn)?;

    f(file)
}

pub fn with_media_post<F>(
    conn: &PgConnection,
    file: &File,
    post: &Post,
    f: F,
) -> Result<(), GenericError>
where
    F: FnOnce(MediaPost) -> Result<(), GenericError>,
{
    let media_post = NewMediaPost::new(file, post).insert(conn)?;

    f(media_post)
}

pub fn with_direct_post<F>(
    conn: &PgConnection,
    base_post: &BasePost,
    base_actor: &BaseActor,
    f: F,
) -> Result<(), GenericError>
where
    F: FnOnce(DirectPost) -> Result<(), GenericError>,
{
    let direct_post = NewDirectPost::new(base_post, base_actor).insert(conn)?;

    f(direct_post)
}

pub fn with_timer<F>(conn: &PgConnection, f: F) -> Result<(), GenericError>
where
    F: FnOnce(Timer) -> Result<(), GenericError>,
{
    let timer = NewTimer::new(gen_datetime()?).insert(conn)?;

    f(timer)
}

pub fn with_event<F>(
    conn: &PgConnection,
    owner: &Persona,
    start: &Timer,
    end: &Timer,
    f: F,
) -> Result<(), GenericError>
where
    F: FnOnce(Event) -> Result<(), GenericError>,
{
    let event =
        NewEvent::new(owner, start, end, Tz::UTC, gen_string()?, gen_string()?)?.insert(conn)?;

    f(event)
}

pub fn with_event_notification<F>(
    conn: &PgConnection,
    event: &Event,
    timer: &Timer,
    f: F,
) -> Result<(), GenericError>
where
    F: FnOnce(EventNotification) -> Result<(), GenericError>,
{
    let event_notification = NewEventNotification::new(event, timer).insert(conn)?;

    f(event_notification)
}

pub fn with_unverified_user<F>(conn: &PgConnection, f: F) -> Result<(), GenericError>
where
    F: FnOnce(UnverifiedUser) -> Result<(), GenericError>,
{
    let unauthenticated_user = NewUser::new().insert(conn)?;

    let unverified_user = match unauthenticated_user.into_verified(conn)? {
        Ok(_) => return Err(AlreadyVerified.into()),
        Err(unverified_user) => unverified_user,
    };

    f(unverified_user)
}

pub fn with_unverified_email<F, U>(conn: &PgConnection, user: &U, f: F) -> Result<(), GenericError>
where
    F: FnOnce(UnverifiedEmail, EmailToken) -> Result<(), GenericError>,
    U: UserLike,
{
    let (email, token) = NewEmail::new(gen_string()?, user)?;

    let email = email.insert(conn)?;

    f(email, token)
}

pub fn with_local_auth<F>(
    conn: &PgConnection,
    user: &UnverifiedUser,
    password: &str,
    f: F,
) -> Result<(), GenericError>
where
    F: FnOnce(LocalAuth) -> Result<(), GenericError>,
{
    let password = create_plaintext_password(password)?;

    let local_auth = NewLocalAuth::new(user, password)?.insert(conn)?;

    f(local_auth)
}

pub fn make_verified_authenticated_user<F>(
    conn: &PgConnection,
    password: &str,
    f: F,
) -> Result<(), GenericError>
where
    F: FnOnce(AuthenticatedUser, VerifiedEmail) -> Result<(), GenericError>,
{
    let unauthenticated_user = NewUser::new().insert(conn)?;

    let user = match unauthenticated_user.into_verified(conn)? {
        Ok(_) => return Err(AlreadyVerified.into()),
        Err(unverified_user) => unverified_user,
    };

    let password = create_plaintext_password(password)?;
    NewLocalAuth::new(&user, password)?.insert(conn)?;

    let (email, token) = NewEmail::new(gen_string()?, &user)?;
    let email = email.insert(conn)?;
    let token = transmute_email_token(&token)?;

    let (user, email) = user.verify(email, token)?.store_verify(conn)?;

    f(user, email)
}

#[derive(Debug, Fail)]
#[fail(display = "User is already verified")]
pub struct AlreadyVerified;

pub fn make_unverified_authenticated_user<F>(
    conn: &PgConnection,
    password: &str,
    f: F,
) -> Result<(), GenericError>
where
    F: FnOnce(AuthenticatedUser) -> Result<(), GenericError>,
{
    with_unverified_user(conn, |user| {
        with_unverified_email(conn, &user, |_email, _token| {
            with_local_auth(conn, &user, password, |auth| {
                let user = UnauthenticatedUser::by_id(user.id(), conn)?;

                let user = user.log_in_local(auth, create_plaintext_password(password)?)?;

                f(user)
            })
        })
    })
}
