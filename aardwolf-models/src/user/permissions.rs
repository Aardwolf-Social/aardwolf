use chrono::offset::Utc;
use diesel::{self, pg::PgConnection};
use mime::Mime as OrigMime;

use super::UserLike;
use base_actor::{
    follow_request::{FollowRequest, NewFollowRequest},
    follower::{Follower, NewFollower},
    persona::{NewPersona, Persona},
    {BaseActor, NewBaseActor},
};
use base_post::{
    post::{
        comment::{Comment, NewComment},
        media_post::{MediaPost, NewMediaPost},
        {NewPost, Post},
    },
    {BasePost, NewBasePost},
};
use file::{image::Image, File};
use sql_types::{FollowPolicy, Permission, PostVisibility, Role, Url};

#[derive(Clone, Debug, Fail)]
pub enum PermissionError {
    #[fail(display = "Failed to check user's permission")]
    Diesel,
    #[fail(display = "User doesn't have this permission")]
    Permission,
}

impl From<diesel::result::Error> for PermissionError {
    fn from(_: diesel::result::Error) -> Self {
        PermissionError::Diesel
    }
}

pub type PermissionResult<T> = Result<T, PermissionError>;

/// Define things a logged-in user is allowed to do.
///
/// The end-goal for this trait is to produce types like `PostCreator`, `UserFollower`, and
/// `InstanceConfigurator`. These types would *only* be producable through this trait, and would be
/// the only ways to perform the actions associated with the permission they came from.
///
/// This way, permission checking would be enforced by the compiler, since "making a post" or
/// "configuring the instance" would not be possible without calling these methods.
pub trait PermissionedUser: UserLike + Sized {
    fn can_post<'a>(
        &self,
        base_actor: &'a BaseActor,
        conn: &PgConnection,
    ) -> PermissionResult<PostMaker<'a>> {
        self.with_actor(base_actor).and_then(|actor| {
            self.has_permission(Permission::MakePost, conn)
                .map(|_| PostMaker(actor))
        })
    }

    fn can_post_media<'a>(
        &self,
        base_actor: &'a BaseActor,
        conn: &PgConnection,
    ) -> PermissionResult<MediaPostMaker<'a>> {
        self.with_actor(base_actor).and_then(|actor| {
            self.has_permission(Permission::MakeMediaPost, conn)
                .map(|_| MediaPostMaker(actor))
        })
    }

    fn can_post_comment<'a>(
        &self,
        base_actor: &'a BaseActor,
        conn: &PgConnection,
    ) -> PermissionResult<CommentMaker<'a>> {
        self.with_actor(base_actor).and_then(|actor| {
            self.has_permission(Permission::MakeComment, conn)
                .map(|_| CommentMaker(actor))
        })
    }

    fn can_follow<'a>(
        &self,
        base_actor: &'a BaseActor,
        conn: &PgConnection,
    ) -> PermissionResult<ActorFollower<'a>> {
        self.with_actor(base_actor).and_then(|actor| {
            self.has_permission(Permission::FollowUser, conn)
                .map(|_| ActorFollower(actor))
        })
    }

    fn can_make_persona(&self, conn: &PgConnection) -> PermissionResult<LocalPersonaCreator<Self>>
    where
        Self: Clone,
    {
        self.has_permission(Permission::MakePersona, conn)
            .map(|_| LocalPersonaCreator(self.clone()))
    }

    fn can_switch_persona(
        &self,
        persona: Persona,
        conn: &PgConnection,
    ) -> PermissionResult<PersonaSwitcher> {
        self.with_persona(persona, conn).and_then(|persona| {
            self.has_permission(Permission::SwitchPersona, conn)
                .map(|_| PersonaSwitcher(persona))
        })
    }

    fn can_delete_persona(
        &self,
        persona: Persona,
        conn: &PgConnection,
    ) -> PermissionResult<PersonaDeleter> {
        self.with_persona(persona, conn).and_then(|persona| {
            self.has_permission(Permission::DeletePersona, conn)
                .map(|_| PersonaDeleter(persona))
        })
    }

    fn can_manage_follow_requests<'a>(
        &self,
        base_actor: &'a BaseActor,
        conn: &PgConnection,
    ) -> PermissionResult<FollowRequestManager<'a>> {
        self.with_actor(base_actor).and_then(|actor| {
            self.has_permission(Permission::ManageFollowRequest, conn)
                .map(|_| FollowRequestManager(actor))
        })
    }

    fn can_configure_instance(&self, conn: &PgConnection) -> PermissionResult<()> {
        self.has_permission(Permission::ConfigureInstance, conn)
    }

    fn can_ban_user(&self, conn: &PgConnection) -> PermissionResult<()> {
        self.has_permission(Permission::BanUser, conn)
    }

    fn can_block_instance(&self, conn: &PgConnection) -> PermissionResult<()> {
        self.has_permission(Permission::BlockInstance, conn)
    }

    fn can_grant_role(&self, conn: &PgConnection) -> PermissionResult<RoleGranter> {
        self.has_permission(Permission::GrantRole, conn)
            .map(|_| RoleGranter::new())
    }

    fn can_revoke_role(&self, conn: &PgConnection) -> PermissionResult<RoleRevoker> {
        self.has_permission(Permission::RevokeRole, conn)
            .map(|_| RoleRevoker::new())
    }

    fn with_actor<'a>(&self, base_actor: &'a BaseActor) -> PermissionResult<&'a BaseActor> {
        base_actor
            .local_user()
            .and_then(|id| {
                if id == self.id() {
                    Some(base_actor)
                } else {
                    None
                }
            })
            .ok_or(PermissionError::Permission)
    }

    fn with_persona(&self, persona: Persona, conn: &PgConnection) -> PermissionResult<Persona> {
        persona
            .belongs_to_user(self, conn)
            .map_err(|_| PermissionError::Permission)
            .and_then(|belongs| {
                if belongs {
                    Ok(persona)
                } else {
                    Err(PermissionError::Permission)
                }
            })
    }

    fn has_permission(&self, permission: Permission, conn: &PgConnection) -> PermissionResult<()> {
        use diesel::prelude::*;
        use schema::{permissions, role_permissions, roles, user_roles};

        roles::dsl::roles
            .inner_join(user_roles::dsl::user_roles)
            .inner_join(role_permissions::dsl::role_permissions)
            .inner_join(
                permissions::dsl::permissions
                    .on(role_permissions::dsl::permission_id.eq(permissions::dsl::id)),
            )
            .filter(user_roles::dsl::user_id.eq(self.id()))
            .filter(permissions::dsl::name.eq(permission))
            .count()
            .get_result(conn)
            .map_err(From::from)
            .and_then(|count: i64| {
                if count > 0 {
                    Ok(())
                } else {
                    Err(PermissionError::Permission)
                }
            })
    }
}

pub struct RoleGranter(());

impl RoleGranter {
    pub(crate) fn new() -> RoleGranter {
        RoleGranter(())
    }

    pub fn grant_role<U: UserLike>(
        &self,
        user: &U,
        role: Role,
        conn: &PgConnection,
    ) -> Result<(), diesel::result::Error> {
        use diesel::prelude::*;
        use schema::{roles, user_roles};

        if user.has_role(role, conn)? {
            return Ok(());
        }

        roles::table
            .filter(roles::dsl::name.eq(role))
            .select(roles::dsl::id)
            .get_result(conn)
            .and_then(|role_id: i32| {
                diesel::insert_into(user_roles::table)
                    .values((
                        user_roles::dsl::user_id.eq(user.id()),
                        user_roles::dsl::role_id.eq(role_id),
                        user_roles::dsl::created_at.eq(Utc::now()),
                    ))
                    .execute(conn)
                    .map(|_| ())
            })
    }
}

pub struct RoleRevoker(());

impl RoleRevoker {
    pub(crate) fn new() -> RoleRevoker {
        RoleRevoker(())
    }

    pub fn revoke_role<U: UserLike>(
        &self,
        user: &U,
        role: Role,
        conn: &PgConnection,
    ) -> Result<(), diesel::result::Error> {
        use diesel::prelude::*;
        use schema::{roles, user_roles};

        if !user.has_role(role, conn)? {
            return Ok(());
        }

        roles::table
            .filter(roles::dsl::name.eq(role))
            .select(roles::dsl::id)
            .get_result(conn)
            .and_then(|role_id: i32| {
                let user_role = user_roles::table
                    .filter(user_roles::dsl::user_id.eq(user.id()))
                    .filter(user_roles::dsl::role_id.eq(role_id));

                diesel::delete(user_role).execute(conn)
            })
            .map(|_| ())
    }
}

pub struct PostMaker<'a>(&'a BaseActor);

impl<'a> PostMaker<'a> {
    #[cfg_attr(feature = "cargo-clippy", allow(clippy::too_many_arguments))]
    pub fn make_post(
        &self,
        name: Option<String>,
        media_type: OrigMime,
        icon: Option<&Image>,
        visibility: PostVisibility,
        content: String,
        source: String,
        conn: &PgConnection,
    ) -> Result<(BasePost, Post), diesel::result::Error> {
        use diesel::prelude::*;
        use schema::{base_posts, posts};

        conn.transaction(|| {
            diesel::insert_into(base_posts::table)
                .values(&NewBasePost::new(
                    name,
                    media_type,
                    self.0,
                    icon,
                    visibility,
                ))
                .get_result(conn)
                .and_then(|base_post: BasePost| {
                    diesel::insert_into(posts::table)
                        .values(&NewPost::new(content, Some(source), &base_post))
                        .get_result(conn)
                        .map(|post: Post| (base_post, post))
                })
        })
    }
}

pub struct MediaPostMaker<'a>(&'a BaseActor);

impl<'a> MediaPostMaker<'a> {
    #[cfg_attr(feature = "cargo-clippy", allow(clippy::too_many_arguments))]
    pub fn make_media_post(
        &self,
        name: Option<String>,
        media_type: OrigMime,
        icon: Option<&Image>,
        visibility: PostVisibility,
        content: String,
        source: String,
        media: &File,
        conn: &PgConnection,
    ) -> Result<(BasePost, Post, MediaPost), diesel::result::Error> {
        use diesel::prelude::*;
        use schema::media_posts;

        conn.transaction(|| {
            PostMaker(self.0)
                .make_post(
                    name,
                    media_type,
                    icon,
                    visibility,
                    content,
                    source,
                    conn,
                )
                .and_then(|(base_post, post)| {
                    diesel::insert_into(media_posts::table)
                        .values(&NewMediaPost::new(media, &post))
                        .get_result(conn)
                        .map(|media_post: MediaPost| (base_post, post, media_post))
                })
        })
    }
}

pub struct CommentMaker<'a>(&'a BaseActor);

impl<'a> CommentMaker<'a> {
    #[cfg_attr(feature = "cargo-clippy", allow(clippy::too_many_arguments))]
    pub fn make_comment(
        &self,
        name: Option<String>,
        media_type: OrigMime,
        icon: Option<&Image>,
        visibility: PostVisibility,
        content: String,
        source: String,
        conversation: &Post,
        parent: &Post,
        conn: &PgConnection,
    ) -> Result<(BasePost, Post, Comment), CommentError> {
        use diesel::prelude::*;
        use schema::{base_posts, comments};

        let conversation_base: BasePost = base_posts::table
            .filter(base_posts::dsl::id.eq(conversation.base_post()))
            .get_result(conn)?;

        if !conversation_base.is_visible_by(self.0, conn)? {
            return Err(CommentError::Permission);
        }

        if parent.id() != conversation.id() {
            let parent_base: BasePost = base_posts::table
                .filter(base_posts::dsl::id.eq(parent.base_post()))
                .get_result(conn)?;

            if !parent_base.is_visible_by(self.0, conn)? {
                return Err(CommentError::Permission);
            }
        }

        conn.transaction(|| {
            PostMaker(self.0)
                .make_post(
                    name,
                    media_type,
                    icon,
                    visibility,
                    content,
                    source,
                    conn,
                )
                .and_then(|(base_post, post)| {
                    diesel::insert_into(comments::table)
                        .values(NewComment::new(conversation, parent, &post))
                        .get_result(conn)
                        .map(|comment: Comment| (base_post, post, comment))
                })
        })
        .map_err(From::from)
    }
}

#[derive(Debug, Fail)]
pub enum CommentError {
    #[fail(display = "Error creating comment")]
    Diesel(diesel::result::Error),
    #[fail(display = "Not allowed to comment on provided post")]
    Permission,
}

impl From<diesel::result::Error> for CommentError {
    fn from(e: diesel::result::Error) -> Self {
        CommentError::Diesel(e)
    }
}

pub struct ActorFollower<'a>(&'a BaseActor);

impl<'a> ActorFollower<'a> {
    pub fn follow_actor(
        &self,
        target_actor: &BaseActor,
        conn: &PgConnection,
    ) -> Result<FollowRequest, FollowError> {
        use diesel::prelude::*;
        use schema::follow_requests;

        match target_actor.follow_policy() {
            FollowPolicy::AutoAccept | FollowPolicy::ManualReview => {
                diesel::insert_into(follow_requests::table)
                    .values(&NewFollowRequest::new(self.0, target_actor))
                    .get_result(conn)
                    .map_err(From::from)
            }
            FollowPolicy::AutoReject => Err(FollowError::Reject),
        }
    }
}

#[derive(Debug, Fail)]
pub enum FollowError {
    #[fail(display = "Error creating follow request")]
    Diesel(#[cause] diesel::result::Error),
    #[fail(display = "Target actor is not accepting follow requests")]
    Reject,
}

impl From<diesel::result::Error> for FollowError {
    fn from(e: diesel::result::Error) -> Self {
        FollowError::Diesel(e)
    }
}

pub struct FollowRequestManager<'a>(&'a BaseActor);

impl<'a> FollowRequestManager<'a> {
    pub fn accept_follow_request(
        &self,
        follow_request: FollowRequest,
        conn: &PgConnection,
    ) -> Result<Follower, FollowRequestManagerError> {
        use diesel::prelude::*;
        use schema::followers;

        if follow_request.requested_follow() != self.0.id() {
            return Err(FollowRequestManagerError::IdMismatch);
        }

        conn.transaction(|| {
            diesel::delete(&follow_request)
                .execute(conn)
                .and_then(|_| {
                    diesel::insert_into(followers::table)
                        .values(&NewFollower::from(follow_request))
                        .get_result(conn)
                })
                .map_err(From::from)
        })
    }

    pub fn reject_follow_request(
        &self,
        follow_request: &FollowRequest,
        conn: &PgConnection,
    ) -> Result<(), FollowRequestManagerError> {
        use diesel::prelude::*;

        if follow_request.requested_follow() != self.0.id() {
            return Err(FollowRequestManagerError::IdMismatch);
        }

        diesel::delete(follow_request)
            .execute(conn)
            .map(|_| ())
            .map_err(From::from)
    }
}

#[derive(Debug, Fail)]
pub enum FollowRequestManagerError {
    #[fail(display = "Error managing follow request")]
    Diesel(#[cause] diesel::result::Error),
    #[fail(display = "Cannot manage other actor's follow requests")]
    IdMismatch,
}

impl From<diesel::result::Error> for FollowRequestManagerError {
    fn from(e: diesel::result::Error) -> Self {
        FollowRequestManagerError::Diesel(e)
    }
}

pub struct LocalPersonaCreator<U: UserLike>(U);

impl<U: UserLike> LocalPersonaCreator<U> {
    #[cfg_attr(feature = "cargo-clippy", allow(clippy::too_many_arguments))]
    pub fn create_persona(
        &self,
        display_name: String,
        profile_url: Url,
        inbox_url: Url,
        outbox_url: Url,
        follow_policy: FollowPolicy,
        default_visibility: PostVisibility,
        is_searchable: bool,
        avatar: Option<&Image>,
        shortname: String,
        private_key_der: Vec<u8>,
        public_key_der: Vec<u8>,
        conn: &PgConnection,
    ) -> Result<(BaseActor, Persona), diesel::result::Error> {
        use diesel::Connection;

        conn.transaction(|| {
            NewBaseActor::local(
                display_name,
                profile_url,
                inbox_url,
                outbox_url,
                &self.0,
                follow_policy,
                private_key_der,
                public_key_der,
            )
            .insert(conn)
            .and_then(|base_actor| {
                NewPersona::new(
                    default_visibility,
                    is_searchable,
                    avatar,
                    shortname,
                    &base_actor,
                )
                .insert(conn)
                .and_then(|persona| {
                    use diesel::prelude::*;
                    use crate::schema::users::dsl::{users, id, primary_persona};

                    if self.0.primary_persona().is_none() {
                        diesel::update(users.filter(id.eq(self.0.id())))
                            .set(primary_persona.eq(persona.id()))
                            .execute(conn)
                            .map(|_| (base_actor, persona))

                    } else {
                        Ok((base_actor, persona))
                    }
                })
            })
        })
    }
}

pub struct PersonaDeleter(Persona);

impl PersonaDeleter {
    pub fn delete_persona(self, conn: &PgConnection) -> Result<(), diesel::result::Error> {
        self.0.delete(conn)
    }
}

pub struct PersonaSwitcher(Persona);

impl PersonaSwitcher {
    pub fn switch_persona(self) -> i32 {
        self.0.id()
    }
}
