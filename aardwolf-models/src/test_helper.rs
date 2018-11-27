use std::env;
use std::io::Error as IoError;

use chrono::{offset::Utc, DateTime, Duration as OldDuration};
use chrono_tz::Tz;
use diesel::{self, pg::PgConnection, Connection};
use dotenv::dotenv;
use mime::TEXT_PLAIN;
use rand::{distributions::Alphanumeric, rngs::OsRng, Error as RandError, Rng};
use serde_json;
use url::{ParseError as UrlParseError, Url as OrigUrl};

use base_actor::{
    follow_request::{FollowRequest, NewFollowRequest},
    follower::{Follower, NewFollower},
    group::{
        group_base_actor::{GroupBaseActor, NewGroupBaseActor},
        {Group, NewGroup},
    },
    persona::{NewPersona, Persona},
    {BaseActor, NewBaseActor},
};
use base_post::{
    direct_post::{DirectPost, NewDirectPost},
    post::{
        comment::{
            reaction::{NewReaction, Reaction},
            {Comment, NewComment},
        },
        media_post::{MediaPost, NewMediaPost},
        {NewPost, Post},
    },
    {BasePost, NewBasePost},
};
use file::{File, FileCreationError, NewFile};
use sql_types::{FollowPolicy, PostVisibility, ReactionType, Url};
use timer::{
    event::{Event, EventCreationError, NewEvent},
    event_notification::{EventNotification, NewEventNotification},
    {NewTimer, Timer},
};
use user::{
    email::{
        CreationError as EmailCreationError, EmailToken, EmailVerificationToken, NewEmail,
        UnverifiedEmail, VerificationError as EmailVerificationError, VerifiedEmail,
    },
    local_auth::{
        LocalAuth, NewLocalAuth, PasswordCreationError, PlaintextPassword,
        VerificationError as PasswordVerificationError,
    },
    QueriedUser,
    {AuthenticatedUser, NewUser, UnauthenticatedUser, UnverifiedUser, UserLike, UserVerifyError},
};

#[derive(Debug, Fail)]
pub enum GenericError {
    #[fail(display = "Error in diesel: {}", _0)]
    Diesel(#[cause] diesel::result::Error),

    #[fail(display = "IO Error: {}", _0)]
    Io(#[cause] IoError),

    #[fail(display = "Failed to parse url: {}", _0)]
    Url(#[cause] UrlParseError),

    #[fail(display = "Failed to create file: {}", _0)]
    File(#[cause] FileCreationError),

    #[fail(display = "Failed to create event: {}", _0)]
    Event(#[cause] EventCreationError),

    #[fail(display = "Failed to create password: {}", _0)]
    Password(#[cause] PasswordCreationError),

    #[fail(display = "Failed in Serde JSON: {}", _0)]
    SerdeJson(#[cause] serde_json::Error),

    #[fail(display = "Failed to create Email: {}", _0)]
    EmailCreation(#[cause] EmailCreationError),

    #[fail(display = "Failed to verify Email: {}", _0)]
    EmailVerification(#[cause] EmailVerificationError),

    #[fail(display = "Failed to verify user: {}", _0)]
    UserVerification(#[cause] UserVerifyError),

    #[fail(display = "Failed to verify password: {}", _0)]
    PasswordVerification(#[cause] PasswordVerificationError),

    #[fail(display = "Failed to be random: {}", _0)]
    Rand(#[cause] RandError),

    #[fail(display = "Generated time is out of bounds")]
    TimeBounds,

    #[fail(display = "Item should not be verified at this point")]
    Verified,

    #[fail(display = "Other error: {}", _0)]
    Other(#[cause] failure::Error),
}

impl From<diesel::result::Error> for GenericError {
    fn from(e: diesel::result::Error) -> Self {
        GenericError::Diesel(e)
    }
}

impl From<IoError> for GenericError {
    fn from(e: IoError) -> Self {
        GenericError::Io(e)
    }
}

impl From<UrlParseError> for GenericError {
    fn from(e: UrlParseError) -> Self {
        GenericError::Url(e)
    }
}

impl From<FileCreationError> for GenericError {
    fn from(e: FileCreationError) -> Self {
        GenericError::File(e)
    }
}

impl From<EventCreationError> for GenericError {
    fn from(e: EventCreationError) -> Self {
        GenericError::Event(e)
    }
}

impl From<PasswordCreationError> for GenericError {
    fn from(e: PasswordCreationError) -> Self {
        GenericError::Password(e)
    }
}

impl From<serde_json::Error> for GenericError {
    fn from(e: serde_json::Error) -> Self {
        GenericError::SerdeJson(e)
    }
}

impl From<EmailCreationError> for GenericError {
    fn from(e: EmailCreationError) -> Self {
        GenericError::EmailCreation(e)
    }
}

impl From<EmailVerificationError> for GenericError {
    fn from(e: EmailVerificationError) -> Self {
        GenericError::EmailVerification(e)
    }
}

impl From<UserVerifyError> for GenericError {
    fn from(e: UserVerifyError) -> Self {
        GenericError::UserVerification(e)
    }
}

impl From<PasswordVerificationError> for GenericError {
    fn from(e: PasswordVerificationError) -> Self {
        GenericError::PasswordVerification(e)
    }
}

impl From<RandError> for GenericError {
    fn from(e: RandError) -> Self {
        GenericError::Rand(e)
    }
}

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

    Utc::now()
        .checked_add_signed(OldDuration::hours(hours))
        .ok_or(GenericError::TimeBounds)
}

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
    let base_actor = NewBaseActor::new::<QueriedUser>(
        gen_string()?,
        gen_url()?,
        gen_url()?,
        gen_url()?,
        None,
        FollowPolicy::AutoAccept,
        json!({}),
    )
    .insert(conn)?;

    f(base_actor)
}

pub fn user_with_base_actor<F>(
    conn: &PgConnection,
    user: &AuthenticatedUser,
    f: F,
) -> Result<(), GenericError>
where
    F: FnOnce(BaseActor) -> Result<(), GenericError>,
{
    let base_actor = NewBaseActor::new(
        gen_string()?,
        gen_url()?,
        gen_url()?,
        gen_url()?,
        Some(user),
        FollowPolicy::AutoAccept,
        json!({}),
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
    let base_post = NewBasePost::new(
        None,
        TEXT_PLAIN,
        posted_by,
        None,
        PostVisibility::Public,
        json!({}),
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
        Ok(_) => return Err(GenericError::Verified),
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
        Ok(_) => return Err(GenericError::Verified),
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
